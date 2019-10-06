mod common;
pub mod error;
pub mod kv;
pub mod primitive;
pub mod packet;
pub mod core;

pub use crate::core::Titso;
pub use common::suggest;

#[macro_export]
macro_rules! chars {
    ( numeric ) => { "0123456789" };
    ( alphabet_lowercase ) => { "abcdefghijklmnopqrstuvwxyz" };
    ( alphabet_uppercase ) => { "ABCDEFGHIJKLMNOPQRSTUVWXYZ" };
    ( punctuation_simple ) => { ",.;-=_+?~!@#" };
    ( punctuation_one ) => { ",./;'[]=-\\`" };
    ( punctuation_more ) => { "~!@#$%^&*()_+{}|:\"<>?" };

    ( $( $name:tt ),* ) => {
        concat!(
            $(
                $crate::chars!($name)
            ),*
        )
    }
}
