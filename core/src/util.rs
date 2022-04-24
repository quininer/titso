pub struct ScopeZeroed<B: AsMut<[u8]>>(pub B, pub fn(&mut [u8]));

impl<B: AsMut<[u8]>> ScopeZeroed<B> {
    pub fn get_mut(&mut self) -> &mut B {
        &mut self.0
    }
}

impl<B: AsMut<[u8]>> Drop for ScopeZeroed<B> {
    fn drop(&mut self) {
        (self.1)(self.0.as_mut());
    }
}
