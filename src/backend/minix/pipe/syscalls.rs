//! minix syscalls supporting `rustix::pipe`.
//!
//! The VFS has `pipe` and nothing else in this group: there is no `splice`,
//! `vmsplice` or `tee`, and no pipe-size control, all of which the public
//! layer keeps behind `linux_kernel`.

use crate::backend::c;
use crate::backend::conv::{errno, owned_fd_from_raw};
use crate::fd::OwnedFd;
use crate::io;
use crate::pipe::PipeFlags;

pub(crate) fn pipe() -> io::Result<(OwnedFd, OwnedFd)> {
    let (read, write) = minix_std::fs::pipe().map_err(errno)?;
    // SAFETY: `pipe` hands back two descriptors the caller now owns.
    unsafe { Ok((owned_fd_from_raw(read), owned_fd_from_raw(write))) }
}

pub(crate) fn pipe_with(flags: PipeFlags) -> io::Result<(OwnedFd, OwnedFd)> {
    let (read, write) = pipe()?;
    for fd in [&read, &write] {
        use crate::fd::AsRawFd as _;
        let raw = fd.as_raw_fd();
        if flags.contains(PipeFlags::CLOEXEC) {
            minix_std::fs::fcntl(raw, c::F_SETFD, c::FD_CLOEXEC).map_err(errno)?;
        }
        if flags.contains(PipeFlags::NONBLOCK) {
            minix_std::fs::fcntl(raw, c::F_SETFL, c::O_NONBLOCK).map_err(errno)?;
        }
    }
    Ok((read, write))
}
