use c_types::c_int;

#[no_mangle]
pub unsafe extern "C" fn _Exit(ec: c_int) -> ! {
    syscall!(EXIT_GROUP, ec);
    syscall!(EXIT, ec);
    loop {}
}
