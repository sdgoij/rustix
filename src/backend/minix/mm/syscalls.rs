//! minix syscalls supporting `rustix::mm`.
//!
//! `mmap`, `munmap` and `mprotect` are VM server calls; `minix_std::vmem`
//! carries them, keeping the reason a mapping was refused (`-EAGAIN` when the
//! region table is full, `-EINVAL` for a request VM will not take). `munmap`
//! drops that reason, so a failure reports `ENOMEM`, which is what the port's
//! C ABI reports for it.

use crate::backend::c;
use crate::backend::conv::{borrowed_fd, errno};
use crate::fd::BorrowedFd;
use crate::io;
use super::types::{Advice, MapFlags, MprotectFlags, MsyncFlags, ProtFlags};
use core::ffi::c_void;

/// The errno for a negative reason a VM call returned.
fn vm_errno(reason: i32) -> io::Errno {
    io::Errno::from_raw_os_error(reason.saturating_neg())
}

pub(crate) unsafe fn mmap(
    ptr: *mut c_void,
    len: usize,
    prot: ProtFlags,
    flags: MapFlags,
    fd: BorrowedFd<'_>,
    offset: u64,
) -> io::Result<*mut c_void> {
    // A file offset that does not fit the port's `i64` is a request VM cannot
    // represent, not a wrap.
    let offset = i64::try_from(offset).map_err(|_| io::Errno::OVERFLOW)?;
    // SAFETY: the caller vouches for the range.
    unsafe {
        minix_std::vmem::mmap_status(
            ptr.cast::<u8>(),
            len,
            prot.bits() as i32,
            flags.bits() as i32,
            borrowed_fd(fd),
            offset,
        )
    }
    .map(|p| p.cast::<c_void>())
    .map_err(vm_errno)
}

pub(crate) unsafe fn mmap_anonymous(
    ptr: *mut c_void,
    len: usize,
    prot: ProtFlags,
    flags: MapFlags,
) -> io::Result<*mut c_void> {
    // SAFETY: the caller vouches for the range.
    unsafe {
        minix_std::vmem::mmap_status(
            ptr.cast::<u8>(),
            len,
            prot.bits() as i32,
            (flags.bits() | c::MAP_ANONYMOUS as u32) as i32,
            -1,
            0,
        )
    }
    .map(|p| p.cast::<c_void>())
    .map_err(vm_errno)
}

pub(crate) unsafe fn munmap(ptr: *mut c_void, len: usize) -> io::Result<()> {
    // SAFETY: the caller vouches for the range.
    let r = unsafe { minix_std::vmem::munmap(ptr.cast::<u8>(), len) };
    if r < 0 {
        Err(io::Errno::NOMEM)
    } else {
        Ok(())
    }
}

pub(crate) unsafe fn mprotect(
    ptr: *mut c_void,
    len: usize,
    flags: MprotectFlags,
) -> io::Result<()> {
    // SAFETY: the caller vouches for the range.
    unsafe { minix_std::vmem::mprotect(ptr.cast::<u8>(), len, flags.bits() as i32) }
        .map_err(vm_errno)
}

pub(crate) unsafe fn mlock(_ptr: *mut c_void, _len: usize) -> io::Result<()> {
    // Every mapped page is resident, so there is nothing to pin.
    Ok(())
}

pub(crate) unsafe fn munlock(_ptr: *mut c_void, _len: usize) -> io::Result<()> {
    Ok(())
}

pub(crate) unsafe fn msync(
    _addr: *mut c_void,
    _len: usize,
    _flags: MsyncFlags,
) -> io::Result<()> {
    // A shared mapping in VM is coherent without a flush, and there is no
    // backing store to wait on.
    Ok(())
}

pub(crate) unsafe fn madvise(_addr: *mut c_void, _len: usize, _advice: Advice) -> io::Result<()> {
    // VM has no advice call; the pager has no hint to act on.
    Ok(())
}
