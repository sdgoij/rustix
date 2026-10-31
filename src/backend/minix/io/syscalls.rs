//! minix syscalls supporting `rustix::io`.
//!
//! The VFS has no `readv`/`writev`, so the scatter/gather calls walk the
//! slices, and there is no `dup3`: `dup2` is the one the port has.

use crate::backend::conv::{borrowed_fd, cvt_count, errno, owned_fd_from_raw};
use crate::backend::{c, MAX_IOV};
use crate::fd::{AsFd as _, BorrowedFd, OwnedFd, RawFd};
use crate::io::{self, FdFlags, IoSlice, IoSliceMut};
use crate::ioctl::{IoctlOutput, Opcode};
use core::cmp::min;

/// The count argument is a byte count, so a larger one has an unspecified
/// result; cap it the way the other backends cap it.
const READ_LIMIT: usize = c::ssize_t::MAX as usize;

pub(crate) unsafe fn read(fd: BorrowedFd<'_>, buf: (*mut u8, usize)) -> io::Result<usize> {
    let len = min(buf.1, READ_LIMIT);
    // SAFETY: the caller guarantees `buf` names `buf.1` writable bytes.
    let slice = unsafe { core::slice::from_raw_parts_mut(buf.0, len) };
    unsafe { minix_std::fs::read(borrowed_fd(fd), slice) }
        .map_err(errno)
        .and_then(cvt_count)
}

pub(crate) fn write(fd: BorrowedFd<'_>, buf: &[u8]) -> io::Result<usize> {
    let len = min(buf.len(), READ_LIMIT);
    unsafe { minix_std::fs::write(borrowed_fd(fd), &buf[..len]) }
        .map_err(errno)
        .and_then(cvt_count)
}

pub(crate) unsafe fn pread(
    fd: BorrowedFd<'_>,
    buf: (*mut u8, usize),
    offset: u64,
) -> io::Result<usize> {
    let len = min(buf.1, READ_LIMIT);
    // SAFETY: as in [`read`].
    let slice = unsafe { core::slice::from_raw_parts_mut(buf.0, len) };
    unsafe { minix_std::fs::pread(borrowed_fd(fd), slice, offset) }
        .map_err(errno)
        .and_then(cvt_count)
}

pub(crate) fn pwrite(fd: BorrowedFd<'_>, buf: &[u8], offset: u64) -> io::Result<usize> {
    let len = min(buf.len(), READ_LIMIT);
    unsafe { minix_std::fs::pwrite(borrowed_fd(fd), &buf[..len], offset) }
        .map_err(errno)
        .and_then(cvt_count)
}

pub(crate) fn readv(fd: BorrowedFd<'_>, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
    let mut total = 0;
    for buf in bufs.iter_mut().take(MAX_IOV) {
        if buf.is_empty() {
            continue;
        }
        let slice: &mut [u8] = buf;
        let read = unsafe { read(fd, (slice.as_mut_ptr(), slice.len())) }?;
        total += read;
        if read < slice.len() {
            break;
        }
    }
    Ok(total)
}

pub(crate) fn writev(fd: BorrowedFd<'_>, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
    let mut total = 0;
    for buf in bufs.iter().take(MAX_IOV) {
        if buf.is_empty() {
            continue;
        }
        let written = write(fd, buf)?;
        total += written;
        if written < buf.len() {
            break;
        }
    }
    Ok(total)
}

pub(crate) fn preadv(
    fd: BorrowedFd<'_>,
    bufs: &mut [IoSliceMut<'_>],
    offset: u64,
) -> io::Result<usize> {
    let mut total = 0;
    for buf in bufs.iter_mut().take(MAX_IOV) {
        if buf.is_empty() {
            continue;
        }
        let slice: &mut [u8] = buf;
        let read = unsafe {
            pread(fd, (slice.as_mut_ptr(), slice.len()), offset + total as u64)
        }?;
        total += read;
        if read < slice.len() {
            break;
        }
    }
    Ok(total)
}

pub(crate) fn pwritev(
    fd: BorrowedFd<'_>,
    bufs: &[IoSlice<'_>],
    offset: u64,
) -> io::Result<usize> {
    let mut total = 0;
    for buf in bufs.iter().take(MAX_IOV) {
        if buf.is_empty() {
            continue;
        }
        let written = pwrite(fd, buf, offset + total as u64)?;
        total += written;
        if written < buf.len() {
            break;
        }
    }
    Ok(total)
}

pub(crate) unsafe fn close(raw_fd: RawFd) {
    // `close` has no way to report through rustix's API, and VFS has already
    // dropped its table entry by the time the reply arrives.
    let _ = minix_std::fs::close(raw_fd);
}

#[cfg(feature = "try_close")]
pub(crate) unsafe fn try_close(raw_fd: RawFd) -> io::Result<()> {
    minix_std::fs::close(raw_fd).map_err(errno)
}

#[inline]
pub(crate) unsafe fn ioctl(
    fd: BorrowedFd<'_>,
    request: Opcode,
    arg: *mut c::c_void,
) -> io::Result<IoctlOutput> {
    // The request is `unsigned long` in the header and a 32-bit word in the
    // VFS message; every opcode the port defines fits.
    unsafe { minix_std::fs::ioctl(borrowed_fd(fd), request as u32, arg.cast()) }.map_err(errno)
}

#[inline]
pub(crate) unsafe fn ioctl_readonly(
    fd: BorrowedFd<'_>,
    request: Opcode,
    arg: *mut c::c_void,
) -> io::Result<IoctlOutput> {
    unsafe { ioctl(fd, request, arg) }
}

pub(crate) fn fcntl_getfd(fd: BorrowedFd<'_>) -> io::Result<FdFlags> {
    let flags = minix_std::fs::fcntl(borrowed_fd(fd), c::F_GETFD, 0).map_err(errno)?;
    Ok(FdFlags::from_bits_retain(bitcast!(flags)))
}

pub(crate) fn fcntl_setfd(fd: BorrowedFd<'_>, flags: FdFlags) -> io::Result<()> {
    minix_std::fs::fcntl(borrowed_fd(fd), c::F_SETFD, bitcast!(flags.bits()))
        .map(|_| ())
        .map_err(errno)
}

pub(crate) fn fcntl_dupfd_cloexec(fd: BorrowedFd<'_>, min: RawFd) -> io::Result<OwnedFd> {
    // The VFS has no `F_DUPFD_CLOEXEC`, and `O_CLOEXEC` is a bit `open`
    // ignores on this port, so a plain `F_DUPFD` is the whole of it.
    let raw = minix_std::fs::fcntl(borrowed_fd(fd), c::F_DUPFD, min).map_err(errno)?;
    // SAFETY: `F_DUPFD` returns a descriptor the caller now owns.
    Ok(unsafe { owned_fd_from_raw(raw) })
}

pub(crate) fn dup(fd: BorrowedFd<'_>) -> io::Result<OwnedFd> {
    let raw = minix_std::fs::fcntl(borrowed_fd(fd), c::F_DUPFD, 0).map_err(errno)?;
    // SAFETY: `F_DUPFD` returns a descriptor the caller now owns.
    Ok(unsafe { owned_fd_from_raw(raw) })
}

#[allow(clippy::needless_pass_by_ref_mut)]
pub(crate) fn dup2(fd: BorrowedFd<'_>, new: &mut OwnedFd) -> io::Result<()> {
    minix_std::fs::dup2(borrowed_fd(fd), borrowed_fd(new.as_fd()))
        .map(|_| ())
        .map_err(errno)
}

#[allow(clippy::needless_pass_by_ref_mut)]
pub(crate) fn dup3(fd: BorrowedFd<'_>, new: &mut OwnedFd, _flags: io::DupFlags) -> io::Result<()> {
    // The VFS has no `dup3`; `dup2` differs only in the flags, and the port
    // has no `O_CLOEXEC` for them to set.
    dup2(fd, new)
}
