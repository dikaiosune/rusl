use core::usize;

use c_types::*;

#[no_mangle]
pub unsafe extern "C" fn strlen(mut s: *const c_schar) -> size_t {
    #[inline]
    fn align() -> usize { core::mem::size_of::<size_t>() }
    #[inline]
    fn ones() -> usize { !0usize }
    #[inline]
    fn highs() -> usize { usize::wrapping_mul(ones(), usize::MAX)/2 + 1 }
    #[inline]
    fn has_zero(x: usize) -> bool {
        usize::wrapping_sub(x, ones() & !x & highs()) != 0
    }
    let a = s;
    let mut w: *const size_t;
    // check zero until align boundary
    while s as usize % align() != 0 {
        if *s == 0 {
            return s as usize - a as usize
        }
        s = s.offset(1)
    }
    // check with word-chunks
    w = s as _;
    while !has_zero(*w) {
        w = w.offset(1)
    }
    // find exact length
    s = w as _;
    while *s != 0 {
        s = s.offset(1)
    }
    s as usize - a as usize
}
