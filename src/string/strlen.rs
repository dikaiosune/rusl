use core::usize;

use c_types::*;

#[no_mangle]
pub unsafe extern "C" fn strlen(mut s: *const c_schar) -> size_t {
    macro_rules! align { () => { core::mem::size_of::<size_t>() } }
    macro_rules! ones { () => { !0usize } }
    macro_rules! highs { () => { usize::wrapping_mul(ones!(), usize::MAX)/2 + 1 } }
    macro_rules! has_zero { ($x:expr) => {{
        let x: usize = $x;
        let ones: usize = ones!();
        let highs: usize = highs!();
        usize::wrapping_sub(x, ones & !x & highs) != 0
    }} }
    let a = s;
    let mut w: *const size_t;
    // check zero until align boundary
    while s as usize % align!() != 0 {
        if *s == 0 {
            return s as usize - a as usize
        }
        s = s.offset(1)
    }
    // check with word-chunks
    w = s as _;
    while !has_zero!(*w) {
        w = w.offset(1)
    }
    // find exact length
    s = w as _;
    while *s != 0 {
        s = s.offset(1)
    }
    s as usize - a as usize
}
