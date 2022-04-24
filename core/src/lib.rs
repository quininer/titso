mod crypto;
mod shield;
mod util;

use crypto::kdf::Kdf;
use shield::Shield;
use util::ScopeZeroed;

pub struct Core {
    config: Config,
    mkey: Shield
}

pub struct Config {
    pub rng: fn(&mut [u8]),
    pub zero: fn(&mut [u8]),
    pub malloc: fn() -> Box<dyn SecBytes>
}

pub trait SecBytes: Send + 'static {
    fn access_and_unlock(&mut self) -> &mut [u8; 32];
    fn lock(&mut self);
}

impl Core {
    pub fn init(config: Config, password: &[u8]) -> Core {
        let mut salt = ScopeZeroed([0; 32], config.zero);
        let mut secret = ScopeZeroed([0; 32], config.zero);
        let salt: &mut [u8; 32] = salt.get_mut();
        let secret = secret.get_mut();

        (config.rng)(salt);

        Kdf::default().derive(password, salt, secret);

        let mkey = {
            let mut mkey = (config.malloc)();
            let mkey_ref = mkey.access_and_unlock();
            (config.rng)(mkey_ref);

            for i in 0..32 {
                secret[i] ^= mkey_ref[i];
            }

            Shield::new(&config, mkey)
        };

        // TODO store secret,salt

        Core { config, mkey }
    }
}
