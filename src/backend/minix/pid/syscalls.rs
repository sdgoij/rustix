//! minix syscalls for PIDs.

use crate::pid::Pid;

#[inline]
#[must_use]
pub(crate) fn getpid() -> Pid {
    let (pid, _) = minix_std::process::getpid().unwrap_or((0, 0));
    // SAFETY: PM reports the caller's own id, which is always a live process.
    unsafe { Pid::from_raw_unchecked(pid) }
}
