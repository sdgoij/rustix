//! minix syscalls supporting `rustix::thread`.
//!
//! There is no kernel sleep call: the runtime suspends by yielding the CPU and
//! re-reading the clock until the deadline passes, which is what std's PAL does.
//! The port delivers no asynchronous signals to a sleeping thread, so a
//! relative sleep always runs to completion and is never reported interrupted.

use crate::backend::conv::errno;
use crate::clockid::ClockId;
use crate::io;
use crate::thread::NanosleepRelativeResult;
use crate::timespec::Timespec;

/// The clock a minix sleep can measure against.
///
/// A relative sleep only needs a clock that cannot go backwards, and a deadline
/// is compared on the clock it names, so the realtime and monotonic ids are
/// both answered; any other id is one PM does not serve.
fn sleep_clock(id: ClockId) -> io::Result<ClockId> {
    match id {
        ClockId::Monotonic | ClockId::Realtime => Ok(id),
        _ => Err(io::Errno::INVAL),
    }
}

fn read_clock(id: ClockId) -> io::Result<Timespec> {
    minix_std::time::clock_gettime(id as i32)
        .map(|ts| Timespec {
            tv_sec: ts.tv_sec,
            tv_nsec: ts.tv_nsec,
        })
        .map_err(errno)
}

/// A monotonic deadline `duration` from now, saturating rather than wrapping.
fn deadline_from(duration: &Timespec) -> io::Result<Timespec> {
    let now = read_clock(ClockId::Monotonic)?;
    let mut sec = now.tv_sec.saturating_add(duration.tv_sec);
    let mut nsec = now.tv_nsec.saturating_add(duration.tv_nsec);
    if nsec >= 1_000_000_000 {
        nsec -= 1_000_000_000;
        sec = sec.saturating_add(1);
    }
    Ok(Timespec {
        tv_sec: sec,
        tv_nsec: nsec,
    })
}

/// Yield until `deadline` on `id`'s clock has passed.
fn sleep_until(id: ClockId, deadline: Timespec) -> NanosleepRelativeResult {
    loop {
        match read_clock(id) {
            Ok(now) => {
                if now.tv_sec > deadline.tv_sec
                    || (now.tv_sec == deadline.tv_sec && now.tv_nsec >= deadline.tv_nsec)
                {
                    return NanosleepRelativeResult::Ok;
                }
            }
            Err(e) => return NanosleepRelativeResult::Err(e),
        }
        minix_std::thread_yield();
    }
}

pub(crate) fn clock_nanosleep_relative(
    id: ClockId,
    request: &Timespec,
) -> NanosleepRelativeResult {
    let id = match sleep_clock(id) {
        Ok(id) => id,
        Err(e) => return NanosleepRelativeResult::Err(e),
    };
    let deadline = match deadline_from(request) {
        Ok(deadline) => deadline,
        Err(e) => return NanosleepRelativeResult::Err(e),
    };
    sleep_until(id, deadline)
}

pub(crate) fn clock_nanosleep_absolute(id: ClockId, request: &Timespec) -> io::Result<()> {
    let id = sleep_clock(id)?;
    match sleep_until(id, *request) {
        NanosleepRelativeResult::Ok => Ok(()),
        NanosleepRelativeResult::Interrupted(_) => {
            // Nothing here interrupts a sleep, so this cannot happen; a caller
            // told INTR would retry, which is the right recovery if it did.
            Err(io::Errno::INTR)
        }
        NanosleepRelativeResult::Err(e) => Err(e),
    }
}

pub(crate) fn nanosleep(request: &Timespec) -> NanosleepRelativeResult {
    let deadline = match deadline_from(request) {
        Ok(deadline) => deadline,
        Err(e) => return NanosleepRelativeResult::Err(e),
    };
    sleep_until(ClockId::Monotonic, deadline)
}

pub(crate) fn sched_yield() {
    minix_std::thread_yield();
}
