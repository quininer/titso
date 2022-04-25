use titso_core::{ packet, Config, SecBytes, Core as Titso };


const PASSWORD: &[u8] = b"testpass";
const MASTER_SECRET: &[u8] = &[162, 100, 115, 97, 108, 116, 88, 32, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 102, 115, 101, 99, 114, 101, 116, 88, 32, 68, 26, 251, 199, 97, 220, 134, 225, 7, 220, 184, 136, 115, 125, 145, 136, 191, 208, 31, 20, 103, 166, 71, 160, 5, 3, 77, 248, 229, 219, 240, 3];

pub struct SimpleBytes([u8; 32]);

impl SecBytes for SimpleBytes {
    fn get_and_unlock(&self) -> &[u8; 32] {
        &self.0
    }

    fn get_mut_and_unlock(&mut self) -> &mut [u8; 32] {
        &mut self.0
    }

    fn lock(&mut self) {}
}

fn malloc() -> Box<dyn SecBytes> {
    Box::new(SimpleBytes([0; 32]))
}

fn zero(buf: &mut [u8]) {
    for b in buf.iter_mut() {
        *b = 0x0;
    }
}

#[test]
fn test_init_password_mkey() -> anyhow::Result<()> {
    use std::sync::atomic;

    static COUNT: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

    fn rng(buf: &mut [u8]) {
        for b in buf.iter_mut() {
            *b = 0x42;
        }
        COUNT.fetch_add(buf.len(), atomic::Ordering::SeqCst);
    }

    let config = Config {
        rng, zero, malloc
    };

    let (_titso, buf) = Titso::create(&config, PASSWORD)?;

    assert_eq!(buf, MASTER_SECRET);

    assert_eq!(COUNT.load(atomic::Ordering::SeqCst), 96);

    Ok(())
}

#[test]
fn test_open_and_store_tag_and_more() -> anyhow::Result<()> {
    fn rng(buf: &mut [u8]) {
        for b in buf.iter_mut() {
            *b = 0x1;
        }
    }

    let config = Config {
        rng, zero, malloc
    };

    let mut titso = Titso::open(&config, MASTER_SECRET, PASSWORD)?;
    let titso = titso.ready()?;

    // tag
    let tag = titso.store_tag(&["tag1", "tag2", "tag3"]);
    assert_eq!(tag.0, [53, 254, 79, 103, 43, 6, 80, 172, 100, 239, 22, 95, 101, 45, 59, 7]);

    // pass
    let rule = packet::Rule {
        count: 1,
        length: 20,
        chars: "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ,.;-=_+?~!@#"
            .chars()
            .collect()
    };
    let site_pass = titso.derive(&["site", "pass"], &rule);
    assert_eq!(site_pass, "dkmPC?STyqosd+0hIqNc");

    // put
    let item = packet::Item {
        password: packet::Type::Fixed("site2-pass".into()),
        note: "site2-note".into(),
        padding: vec![0; 3]
    };
    let value = titso.put(&["tag2"], &item)?;
    assert_eq!(value, vec![120, 77, 169, 195, 53, 252, 237, 231, 146, 139, 28, 195, 149, 183, 42, 95, 49, 247, 135, 170, 45, 19, 178, 132, 66, 126, 85, 235, 155, 40, 154, 91, 184, 116, 223, 224, 110, 165, 126, 178, 28, 189, 211, 19, 41, 64, 106, 104, 6, 104, 247, 27, 239, 208, 9, 10, 158, 64, 134, 211, 72, 123, 27, 81, 77, 92, 74, 41, 2, 44, 224, 27]);

    // get
    let mut value = value;
    let item2 = titso.get(&["tag2"], &mut value)?;
    if let (packet::Type::Fixed(pass), packet::Type::Fixed(pass2)) = (item.password, item2.password) {
        assert_eq!(pass, pass2);
    } else{
        panic!()
    }
    assert_eq!(item.note, item2.note);
    assert_eq!(item.padding, item2.padding);

    Ok(())
}
