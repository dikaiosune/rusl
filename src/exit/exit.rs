use c_types::c_int;
use super::_Exit::_Exit;

#[linkage = "weak"]
#[no_mangle]
pub extern "C" fn __funcs_on_exit() {}

#[linkage = "weak"]
#[no_mangle]
pub extern "C" fn __stdio_exit() {}

#[linkage = "weak"]
#[no_mangle]
pub extern "C" fn _fini() {}

#[linkage = "weak"]
#[no_mangle]
pub extern "C" fn __libc_exit_fini() {
    _fini()
}

#[no_mangle]
pub extern "C" fn exit(code: c_int) -> ! {
    __funcs_on_exit();
    __libc_exit_fini();
    __stdio_exit();
    _Exit(code)
}
