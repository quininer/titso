use std::io::{ self, Read };
use rand_core::{ RngCore, CryptoRng };
use titso_core::{ packet, Titso };


const PASSWORD: &[u8] = b"testpass";
const MASTER_SECRET: &[u8] = &[162, 100, 115, 97, 108, 116, 88, 32, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 102, 115, 101, 99, 114, 101, 116, 88, 32, 68, 26, 251, 199, 97, 220, 134, 225, 7, 220, 184, 136, 115, 125, 145, 136, 191, 208, 31, 20, 103, 166, 71, 160, 5, 3, 77, 248, 229, 219, 240, 3];

pub struct TestRng<R>(R);

impl<R: Read> RngCore for TestRng<R> {
    fn next_u32(&mut self) -> u32 {
        let mut buf = [0; 4];
        self.0.read_exact(&mut buf).unwrap();
        u32::from_le_bytes(buf)
    }

    fn next_u64(&mut self) -> u64 {
        let mut buf = [0; 8];
        self.0.read_exact(&mut buf).unwrap();
        u64::from_le_bytes(buf)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.read_exact(dest).unwrap();
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.0.read_exact(dest).unwrap();
        Ok(())
    }
}

impl<R: Read> CryptoRng for TestRng<R> {}

#[test]
fn test_init_password_mkey() -> anyhow::Result<()> {
    let rng = TestRng(io::Cursor::new(vec![0x42; 64]));

    let (_titso, buf) = Titso::init(rng, PASSWORD)?;

    assert_eq!(buf, MASTER_SECRET);

    Ok(())
}

#[test]
fn test_open_and_store_tag_and_more() -> anyhow::Result<()> {
    let mut titso = Titso::open(PASSWORD, MASTER_SECRET)?;
    let titso = titso.ready();

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
    assert_eq!(value, vec![204, 78, 94, 219, 116, 149, 6, 87, 101, 99, 154, 206, 34, 45, 66, 136, 3, 227, 119, 104, 131, 191, 81, 89, 192, 245, 157, 87, 48, 0, 166, 4, 81, 247, 159, 73, 192, 161, 238, 129, 9, 3, 209, 231, 180, 175, 210, 157, 186, 117, 121, 137, 82, 182, 254, 110, 39, 137, 155, 2, 62, 114, 222, 131, 188, 168, 65, 166, 55, 41, 122, 68]);

    // get
    let item2 = titso.get(&["tag2"], &value)?;
    if let (packet::Type::Fixed(pass), packet::Type::Fixed(pass2)) = (item.password, item2.password) {
        assert_eq!(pass, pass2);
    } else{
        panic!()
    }
    assert_eq!(item.note, item2.note);
    assert_eq!(item.padding, item2.padding);

    Ok(())
}
