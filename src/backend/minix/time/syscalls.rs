//! minix syscalls supporting `rustix::time`.
//!
//! PM serves `clock_gettime`, `clock_getres` and `clock_settime` for the real
//! time and monotonic clocks, and the VFS serves the timerfd family.

use super::types::{TimerfdClockId, TimerfdFlags, TimerfdTimerFlags};
use crate::backend::conv::{borrowed_fd, errno, owned_fd_from_raw};
use crate::clockid::{ClockId, DynamicClockId};
use crate::fd::{BorrowedFd, OwnedFd};
use crate::io;
use crate::time::{Itimerspec, Timespec};

pub(crate) fn clock_getres(id: ClockId) -> Timespec {
    match minix_std::time::clock_getres(id as i32) {
        Ok(ts) => to_rustix(ts),
        Err(_) => Timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
    }
}

pub(crate) fn clock_gettime(id: ClockId) -> Timespec {
    match minix_std::time::clock_gettime(id as i32) {
        Ok(ts) => to_rustix(ts),
        Err(_) => Timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
    }
}

pub(crate) fn clock_gettime_dynamic(id: DynamicClockId<'_>) -> io::Result<Timespec> {
    let raw = match id {
        DynamicClockId::Known(id) => id as i32,
        // The port's PM knows only the real-time and monotonic clocks, so any
        // other dynamic request is the caller's to handle.
        _ => return Err(io::Errno::INVAL),
    };
    minix_std::time::clock_gettime(raw)
        .map(to_rustix)
        .map_err(errno)
}

pub(crate) fn clock_settime(id: ClockId, timespec: Timespec) -> io::Result<()> {
    let ts = minix_std::time::TimeSpec {
        tv_sec: timespec.tv_sec,
        tv_nsec: timespec.tv_nsec,
    };
    minix_std::time::clock_settime(id as i32, &ts).map_err(errno)
}

pub(crate) fn timerfd_create(id: TimerfdClockId, flags: TimerfdFlags) -> io::Result<OwnedFd> {
    let raw = minix_std::fs::timerfd_create(id as i32, bitflags_bits!(flags)).map_err(errno)?;
    // SAFETY: `timerfd_create` hands back a descriptor the caller now owns.
    Ok(unsafe { owned_fd_from_raw(raw) })
}

pub(crate) fn timerfd_settime(
    fd: BorrowedFd<'_>,
    flags: TimerfdTimerFlags,
    new_value: &Itimerspec,
) -> io::Result<Itimerspec> {
    let new_value = minix_std::fs::Itimerspec {
        it_interval: to_minix(new_value.it_interval),
        it_value: to_minix(new_value.it_value),
    };
    let mut old = minix_std::fs::Itimerspec::default();
    minix_std::fs::timerfd_settime(
        borrowed_fd(fd),
        bitflags_bits!(flags),
        &new_value,
        Some(&mut old),
    )
    .map(|()| from_minix(old))
    .map_err(errno)
}

pub(crate) fn timerfd_gettime(fd: BorrowedFd<'_>) -> io::Result<Itimerspec> {
    minix_std::fs::timerfd_gettime(borrowed_fd(fd))
        .map(from_minix)
        .map_err(errno)
}

fn to_rustix(ts: minix_std::time::TimeSpec) -> Timespec {
    Timespec {
        tv_sec: ts.tv_sec,
        tv_nsec: ts.tv_nsec,
    }
}

fn to_minix(ts: Timespec) -> minix_std::time::TimeSpec {
    minix_std::time::TimeSpec {
        tv_sec: ts.tv_sec,
        tv_nsec: ts.tv_nsec,
    }
}

fn from_minix(it: minix_std::fs::Itimerspec) -> Itimerspec {
    Itimerspec {
        it_interval: to_rustix(it.it_interval),
        it_value: to_rustix(it.it_value),
    }
}
