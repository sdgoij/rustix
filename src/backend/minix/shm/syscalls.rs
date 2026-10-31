//! minix syscalls supporting `rustix::shm`.
//!
//! There is no POSIX shared-memory name space here, so both calls refuse with
//! `ENOSYS`. The port's shared-memory path is `memfd_create`, which returns a
//! descriptor that a dma-buf or a client can then map.

use crate::backend::c;
use crate::ffi::CStr;
use crate::fd::OwnedFd;
use crate::fs::Mode;
use crate::{io, shm};

pub(crate) fn shm_open(_name: &CStr, _oflags: shm::OFlags, _mode: Mode) -> io::Result<OwnedFd> {
    Err(io::Errno::NOSYS)
}

pub(crate) fn shm_unlink(_name: &CStr) -> io::Result<()> {
    Err(io::Errno::NOSYS)
}
