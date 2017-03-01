use libc::c_int;

// These constants don't appear in rust-lang/libc, so we define them here
// because musl defines them. Maybe we should add them to libc...
pub const O_LARGEFILE: c_int = 0100000;
pub const O_NOATIME: c_int = 01000000;
pub const O_PATH: c_int = 010000000;
pub const O_TMPFILE: c_int = 020200000;

pub const F_SETSIG: c_int = 10;
pub const F_GETSIG: c_int = 11;

pub const F_SETOWN_EX: c_int = 15;
pub const F_GETOWN_EX: c_int = 16;

pub const F_GETOWNER_UIDS: c_int = 17;
