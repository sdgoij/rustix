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

/// Run `f` with the process cwd moved to `dirfd`, then put it back.
///
/// The VFS has only path-based calls, so a directory descriptor handed to an
/// `*at` function becomes an `fchdir` for the length of one call. `AT_FDCWD`
/// and an absolute path need no move at all.
///
/// # Safety
///
/// `f` must not rely on the cwd being where the caller left it, and no other
/// thread may use a relative path while it runs: the cwd is process state.
pub(super) unsafe fn with_dirfd<R>(
    dirfd: BorrowedFd<'_>,
    path: &[u8],
    f: impl FnOnce() -> Result<R, minix_std::MinixErr>,
) -> Result<R, minix_std::MinixErr> {
    use minix_std::MinixErr;

    if dirfd.as_raw_fd() == c::AT_FDCWD || path.first() == Some(&b'/') {
        return f();
    }
    if dirfd.as_raw_fd() < 0 {
        return Err(MinixErr(io::Errno::BADF.raw_os_error()));
    }

    let mut saved = [0u8; minix_std::fs::PATH_MAX];
    let len = match minix_std::fs::getcwd(&mut saved) {
        Ok(len) => len,
        Err(e) => return Err(e),
    };
    if let Err(e) = minix_std::fs::fchdir(dirfd.as_raw_fd()) {
        return Err(e);
    }
    let out = f();
    if minix_std::fs::chdir(&saved[..len]).is_err() {
        // The call already ran. A cwd that cannot be put back would misdirect
        // every later relative path, so report it rather than continue.
        return Err(MinixErr(io::Errno::IO.raw_os_error()));
    }
    out
}

/// Run a two-name call with each name resolved against its own `dirfd`.
///
/// One `fchdir` cannot serve two directories, so the pairs that would need
/// more than one resolution are refused rather than half applied. A name that
/// was relative to the cwd also changes meaning under the move, which is why a
/// cwd-relative name mixed with a dirfd-relative one is refused too.
///
/// # Safety
///
/// As [`with_dirfd`].
pub(super) unsafe fn two_paths<R>(
    old_dirfd: BorrowedFd<'_>,
    old: &[u8],
    new_dirfd: BorrowedFd<'_>,
    new: &[u8],
    f: impl FnOnce() -> Result<R, minix_std::MinixErr>,
) -> Result<R, minix_std::MinixErr> {
    use minix_std::MinixErr;

    let old_via_fd = !is_absolute(old) && old_dirfd.as_raw_fd() != c::AT_FDCWD;
    let new_via_fd = !is_absolute(new) && new_dirfd.as_raw_fd() != c::AT_FDCWD;
    let old_via_cwd = !is_absolute(old) && old_dirfd.as_raw_fd() == c::AT_FDCWD;
    let new_via_cwd = !is_absolute(new) && new_dirfd.as_raw_fd() == c::AT_FDCWD;

    if old_via_fd && new_via_fd {
        if old_dirfd.as_raw_fd() != new_dirfd.as_raw_fd() {
            return Err(MinixErr(io::Errno::NOSYS.raw_os_error()));
        }
        return unsafe { with_dirfd(old_dirfd, old, f) };
    }
    if (old_via_fd && new_via_cwd) || (new_via_fd && old_via_cwd) {
        return Err(MinixErr(io::Errno::NOSYS.raw_os_error()));
    }
    if old_via_fd {
        return unsafe { with_dirfd(old_dirfd, old, f) };
    }
    if new_via_fd {
        return unsafe { with_dirfd(new_dirfd, new, f) };
    }
    f()
}

fn is_absolute(path: &[u8]) -> bool {
    path.first() == Some(&b'/')
}
