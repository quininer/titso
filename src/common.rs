use byteorder::{ ByteOrder, LittleEndian };
use gimli_permutation::S;


pub fn with<F>(state: &mut [u32; S], f: F)
    where F: FnOnce(&mut [u8; S * 4])
{
    #[inline]
    fn transmute(arr: &mut [u32; S]) -> &mut [u8; S * 4] {
        unsafe { &mut *(arr as *mut [u32; S] as *mut [u8; S * 4]) }
    }

    LittleEndian::from_slice_u32(state);
    f(transmute(state));
    LittleEndian::from_slice_u32(state);
}
