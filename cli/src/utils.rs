use std::ops::{ Deref, DerefMut };
use titso_core::{ SafeFeatures, SafeBytes };


pub struct SafeTools;
pub struct SafeBuf(seckey::SecBytes);
pub struct SafeBufRef<'a>(seckey::SecReadGuard<'a>);
pub struct SafeBufMut<'a>(seckey::SecWriteGuard<'a>);

impl SafeFeatures for SafeTools {
    type SafeBytes = SafeBuf;

    fn rng_fill(buf: &mut [u8]) {
        getrandom::getrandom(buf).unwrap();
    }

    fn zero_bytes(buf: &mut [u8]) {
        seckey::zero(buf);
    }

    fn safe_heap_alloc() -> Self::SafeBytes {
        SafeBuf(seckey::SecBytes::new(32))
    }
}

impl SafeBytes for SafeBuf {
    type Ref<'a> = SafeBufRef<'a>;
    type RefMut<'a> = SafeBufMut<'a>;

    fn get<'a>(&'a self) -> Self::Ref<'a> {
        SafeBufRef(self.0.read())
    }

    fn get_mut<'a>(&'a mut self) -> Self::RefMut<'a> {
        SafeBufMut(self.0.write())
    }
}

impl AsRef<[u8; 32]> for SafeBufRef<'_> {
    fn as_ref(&self) -> &[u8; 32] {
        self.0.deref().try_into().unwrap()
    }
}

impl AsRef<[u8; 32]> for SafeBufMut<'_> {
    fn as_ref(&self) -> &[u8; 32] {
        self.0.deref().try_into().unwrap()
    }
}

impl AsMut<[u8; 32]> for SafeBufMut<'_> {
    fn as_mut(&mut self) -> &mut [u8; 32] {
        self.0.deref_mut().try_into().unwrap()
    }
}
