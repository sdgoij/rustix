//! minix syscalls supporting `rustix::event`.
//!
//! The VFS has `poll`, `eventfd` and the epoll set, and no `select` on this
//! target, which is why the public `select` module is not built for minix.

use super::epoll::CreateFlags;
use super::poll_fd::PollFd;
use crate::backend::c;
use crate::backend::conv::{borrowed_fd, errno, owned_fd_from_raw};
use crate::event::{EventfdFlags, Timespec};
use crate::fd::{BorrowedFd, OwnedFd};
use crate::io;
use core::ptr;

pub(crate) fn eventfd(initval: u32, flags: EventfdFlags) -> io::Result<OwnedFd> {
    let raw = minix_std::fs::eventfd(initval, bitflags_bits!(flags)).map_err(errno)?;
    // SAFETY: `eventfd` hands back a descriptor the caller now owns.
    Ok(unsafe { owned_fd_from_raw(raw) })
}

pub(crate) fn poll(fds: &mut [PollFd<'_>], timeout: Option<&Timespec>) -> io::Result<usize> {
    // `PollFd` is transparent over the port's own `pollfd`, so the slice can be
    // handed over as it stands.
    let fds = unsafe {
        core::slice::from_raw_parts_mut(fds.as_mut_ptr().cast::<minix_std::fs::PollFd>(), fds.len())
    };
    let ready = minix_std::fs::poll(fds, timeout_millis(timeout)).map_err(errno)?;
    Ok(ready as usize)
}

pub(crate) fn pause() {
    // The port blocks for a signal with `sigsuspend`; an empty mask is the
    // closest thing to `pause`, which waits with whatever mask is installed.
    let _ = minix_std::time::sigsuspend(0);
}

pub(crate) fn epoll_create(flags: CreateFlags) -> io::Result<OwnedFd> {
    let raw = minix_std::fs::epoll_create1(bitflags_bits!(flags)).map_err(errno)?;
    // SAFETY: `epoll_create1` hands back a descriptor the caller now owns.
    Ok(unsafe { owned_fd_from_raw(raw) })
}

pub(crate) fn epoll_add(
    epoll: BorrowedFd<'_>,
    source: BorrowedFd<'_>,
    event: &crate::event::epoll::Event,
) -> io::Result<()> {
    epoll_ctl(epoll, c::EPOLL_CTL_ADD, source, event)
}

pub(crate) fn epoll_mod(
    epoll: BorrowedFd<'_>,
    source: BorrowedFd<'_>,
    event: &crate::event::epoll::Event,
) -> io::Result<()> {
    epoll_ctl(epoll, c::EPOLL_CTL_MOD, source, event)
}

pub(crate) fn epoll_del(epoll: BorrowedFd<'_>, source: BorrowedFd<'_>) -> io::Result<()> {
    // `EPOLL_CTL_DEL` ignores the event, but the message still carries one.
    let empty = crate::event::epoll::Event {
        flags: crate::event::epoll::EventFlags::empty(),
        data: crate::event::epoll::EventData::new_u64(0),
    };
    epoll_ctl(epoll, c::EPOLL_CTL_DEL, source, &empty)
}

fn epoll_ctl(
    epoll: BorrowedFd<'_>,
    op: c::c_int,
    source: BorrowedFd<'_>,
    event: &crate::event::epoll::Event,
) -> io::Result<()> {
    // rustix's `Event` is layout-compatible with the port's `EpollEvent`: a
    // 32-bit flag word followed by 64 bits of user data.
    let event = unsafe {
        ptr::read_unaligned(
            (event as *const crate::event::epoll::Event).cast::<minix_std::fs::EpollEvent>(),
        )
    };
    minix_std::fs::epoll_ctl(borrowed_fd(epoll), op, borrowed_fd(source), &event).map_err(errno)
}

pub(crate) unsafe fn epoll_wait(
    epoll: BorrowedFd<'_>,
    events: (*mut crate::event::epoll::Event, usize),
    timeout: Option<&Timespec>,
) -> io::Result<usize> {
    // SAFETY: the caller guarantees `events.0` names `events.1` writable
    // `Event`s, and `EpollEvent` has the same layout.
    let out = unsafe {
        core::slice::from_raw_parts_mut(
            events.0.cast::<minix_std::fs::EpollEvent>(),
            events.1,
        )
    };
    let ready = minix_std::fs::epoll_wait(borrowed_fd(epoll), out, timeout_millis(timeout))
        .map_err(errno)?;
    Ok(ready as usize)
}

/// A `Timespec` as the millisecond timeout the VFS takes. A negative value
/// blocks indefinitely, which is what the VFS reads it as.
fn timeout_millis(timeout: Option<&Timespec>) -> i32 {
    match timeout {
        None => -1,
        Some(ts) => {
            let millis = ts.tv_sec.saturating_mul(1000) + ts.tv_nsec / 1_000_000;
            i32::try_from(millis).unwrap_or(i32::MAX)
        }
    }
}
