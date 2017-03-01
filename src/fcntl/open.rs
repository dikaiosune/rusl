use super::constants::{O_TMPFILE};
use libc::*;
use syscall_mgt::syscall_return;
use va_list::VaList;

#[no_mangle]
pub unsafe extern "C" fn open(filename: *const c_char, flags: c_int, mut args: VaList) -> c_int {
    let mut mode: mode_t = 0;

    if (flags & O_CREAT) != 0 || ((flags & O_TMPFILE) == O_TMPFILE) {
        mode = args.get::<mode_t>();
    }

    let fd = syscall!(OPEN, filename, flags, mode);
    if fd >= 0 && (flags & O_CLOEXEC) != 0 {
        syscall!(FCNTL, fd, F_SETFD, FD_CLOEXEC);
    }

    syscall_return(fd) as c_int
}
