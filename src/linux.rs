use core::sync::atomic::AtomicU32;

#[inline]
pub fn wait(a: &AtomicU32, expected: u32) {
    unsafe {
        libc::syscall(
            libc::SYS_futex,
            a,
            libc::FUTEX_WAIT,
            expected,
            core::ptr::null::<libc::timespec>(),
        );
    };
}

#[inline]
pub fn wake_one(ptr: *const AtomicU32) {
    unsafe {
        libc::syscall(
            libc::SYS_futex,
            ptr,
            libc::FUTEX_WAKE,
            1i32,
        );
    };
}

#[inline]
pub fn wake_all(ptr: *const AtomicU32) {
    unsafe {
        libc::syscall(
            libc::SYS_futex,
            ptr,
            libc::FUTEX_WAKE,
            i32::MAX,
        );
    };
}
