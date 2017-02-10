use c_types::c_int;

#[no_mangle]
pub extern "C" fn _Exit(ec: c_int) -> ! {
    unsafe {
        syscall!(EXIT_GROUP, ec);
        syscall!(EXIT, ec);
    }
    loop {}
}
