use c_types::c_int;
use ::exit::_Exit::_Exit;

#[no_mangle]
pub unsafe extern "C" fn _exit(status: c_int) -> ! {
    _Exit(status)
}
