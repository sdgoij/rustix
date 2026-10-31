//! Conversions between rustix's types and the C-shaped ones the syscall layer
//! names.
//!
//! The libc backend's version exists mostly to turn `-1` plus `errno` into a
//! `Result`. A minix server returns the error itself, so the conversions left
//! are the descriptor and string ones, plus the error mapping.

use super::c;
use super::fd::{
    AsRawFd as _, BorrowedFd, FromRawFd as _, IntoRawFd as _, LibcFd, OwnedFd, RawFd,
};
use crate::ffi::CStr;
use crate::io;

#[inline]
pub(super) fn c_str(c: &CStr) -> *const c::c_char {
    c.as_ptr().cast()
}

#[inline]
pub(super) fn no_fd() -> LibcFd {
    -1
}

#[inline]
pub(super) fn borrowed_fd(fd: BorrowedFd<'_>) -> LibcFd {
    fd.as_raw_fd() as LibcFd
}

#[inline]
pub(super) fn owned_fd(fd: OwnedFd) -> LibcFd {
    fd.into_raw_fd() as LibcFd
}

/// The `Errno` for an error a server returned.
///
/// [`MinixErr`] holds the positive POSIX number, which is the same number a C
/// caller would find in `errno`, so no translation is needed.
///
/// [`MinixErr`]: minix_std::MinixErr
#[inline]
pub(super) fn errno(err: minix_std::MinixErr) -> io::Errno {
    io::Errno::from_raw_os_error(err.0)
}

/// Convert a byte count, which the VFS never returns negative on success.
#[inline]
pub(super) fn cvt_count(count: i64) -> io::Result<usize> {
    usize::try_from(count).map_err(|_| io::Errno::INVAL)
}

/// Take ownership of a descriptor a server handed back.
///
/// # Safety
///
/// `raw` must be a descriptor the caller owns and has not closed.
#[inline]
pub(super) unsafe fn owned_fd_from_raw(raw: LibcFd) -> OwnedFd {
    unsafe { OwnedFd::from_raw_fd(raw as RawFd) }
}

/// The path a `*at` call may use, or `ENOSYS` when it may not.
///
/// The VFS has only path-based calls, so `AT_FDCWD` and an absolute path need
/// no translation. A real directory descriptor would need the path in it,
/// which the runtime does not expose, so that case reports `ENOSYS` rather
/// than resolving the name against the wrong directory.
#[inline]
pub(super) fn at_path<'a>(dirfd: BorrowedFd<'_>, path: &'a CStr) -> io::Result<&'a [u8]> {
    let bytes = path.to_bytes();
    if dirfd.as_raw_fd() == c::AT_FDCWD || bytes.first() == Some(&b'/') {
        Ok(bytes)
    } else {
        Err(io::Errno::NOSYS)
    }
}
