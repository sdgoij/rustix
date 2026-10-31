//! minix syscalls supporting `rustix::process`.
//!
//! The calls PM answers are the ones that exist: the working directory, the
//! process and group identifiers, priority, `umask`, the kill family, `wait`
//! and `waitpid`. There is no `setpgid`, `setsid`, `nice`, `getppid`,
//! `getcwd`, `chroot` or rlimit call, so those report `NOSYS` (or, where the
//! public signature cannot fail, the "no limit" answer).

use crate::backend::c;
use crate::backend::conv::{borrowed_fd, errno};
use crate::fd::BorrowedFd;
use crate::ffi::CStr;
use crate::fs::Mode;
use crate::io;
use crate::process::{Flock, Gid, Pid, RawPid, Resource, Rlimit, Signal, Uid, WaitOptions, WaitStatus};
use crate::process::{WaitId, WaitIdOptions, WaitIdStatus};
use core::mem::MaybeUninit;

pub(crate) fn chdir(path: &CStr) -> io::Result<()> {
    minix_std::fs::chdir(path.to_bytes()).map_err(errno)
}

pub(crate) fn fchdir(dirfd: BorrowedFd<'_>) -> io::Result<()> {
    minix_std::fs::fchdir(borrowed_fd(dirfd)).map_err(errno)
}

pub(crate) fn chroot(_path: &CStr) -> io::Result<()> {
    // PM has no root-changing call.
    Err(io::Errno::NOSYS)
}

pub(crate) fn getcwd(buf: &mut [MaybeUninit<u8>]) -> io::Result<()> {
    // SAFETY: `MaybeUninit<u8>` has the same layout as `u8`, and the walk only
    // writes whole bytes.
    let out = unsafe { core::slice::from_raw_parts_mut(buf.as_mut_ptr().cast::<u8>(), buf.len()) };
    minix_std::fs::getcwd(out).map(|_| ()).map_err(errno)
}

pub(crate) fn getppid() -> Option<Pid> {
    let (_, ppid) = minix_std::process::getpid().ok()?;
    Pid::from_raw(ppid)
}

pub(crate) fn getpgid(pid: Option<Pid>) -> io::Result<Pid> {
    let raw = minix_std::process::getpgid(Pid::as_raw(pid)).map_err(errno)?;
    // SAFETY: PM returned a group id, which is a positive process id.
    Ok(unsafe { Pid::from_raw_unchecked(raw) })
}

pub(crate) fn setpgid(_pid: Option<Pid>, _pgid: Option<Pid>) -> io::Result<()> {
    // PM has no call for changing a process group.
    Err(io::Errno::NOSYS)
}

#[must_use]
pub(crate) fn getpgrp() -> Pid {
    let raw = minix_std::process::getpgrp().unwrap_or(0);
    // SAFETY: the fallback is only reached when PM cannot name the group, and
    // a pid of zero means "the caller's own group" to the rest of the API.
    unsafe { Pid::from_raw_unchecked(raw) }
}

pub(crate) fn umask(mask: Mode) -> Mode {
    let raw = minix_std::fs::umask(mask.bits()).unwrap_or(0);
    Mode::from_bits_retain(raw)
}

pub(crate) fn nice(_inc: i32) -> io::Result<i32> {
    // PM has no relative priority call; `getpriority`/`setpriority` are the
    // absolute pair.
    Err(io::Errno::NOSYS)
}

pub(crate) fn getpriority_user(uid: Uid) -> io::Result<i32> {
    minix_std::process::getpriority(c::PRIO_USER, uid.as_raw() as i32).map_err(errno)
}

pub(crate) fn getpriority_pgrp(pgid: Option<Pid>) -> io::Result<i32> {
    minix_std::process::getpriority(c::PRIO_PGRP, Pid::as_raw(pgid)).map_err(errno)
}

pub(crate) fn getpriority_process(pid: Option<Pid>) -> io::Result<i32> {
    minix_std::process::getpriority(c::PRIO_PROCESS, Pid::as_raw(pid)).map_err(errno)
}

pub(crate) fn setpriority_user(uid: Uid, priority: i32) -> io::Result<()> {
    minix_std::process::setpriority(c::PRIO_USER, uid.as_raw() as i32, priority).map_err(errno)
}

pub(crate) fn setpriority_pgrp(pgid: Option<Pid>, priority: i32) -> io::Result<()> {
    minix_std::process::setpriority(c::PRIO_PGRP, Pid::as_raw(pgid), priority).map_err(errno)
}

pub(crate) fn setpriority_process(pid: Option<Pid>, priority: i32) -> io::Result<()> {
    minix_std::process::setpriority(c::PRIO_PROCESS, Pid::as_raw(pid), priority).map_err(errno)
}

pub(crate) fn getrlimit(_limit: Resource) -> Rlimit {
    // The port enforces no resource limits, so every answer is "unlimited",
    // which is what a `None` field means.
    Rlimit {
        current: None,
        maximum: None,
    }
}

pub(crate) fn setrlimit(_limit: Resource, _new: Rlimit) -> io::Result<()> {
    Err(io::Errno::NOSYS)
}

pub(crate) fn prlimit(_pid: Option<Pid>, _limit: Resource, _new: Rlimit) -> io::Result<Rlimit> {
    Err(io::Errno::NOSYS)
}

pub(crate) fn wait(waitopts: WaitOptions) -> io::Result<Option<(Pid, WaitStatus)>> {
    _waitpid(!0, waitopts)
}

pub(crate) fn waitpid(
    pid: Option<Pid>,
    waitopts: WaitOptions,
) -> io::Result<Option<(Pid, WaitStatus)>> {
    _waitpid(Pid::as_raw(pid), waitopts)
}

pub(crate) fn waitpgid(pgid: Pid, waitopts: WaitOptions) -> io::Result<Option<(Pid, WaitStatus)>> {
    _waitpid(-pgid.as_raw_nonzero().get(), waitopts)
}

pub(crate) fn _waitpid(
    pid: RawPid,
    waitopts: WaitOptions,
) -> io::Result<Option<(Pid, WaitStatus)>> {
    let (child, status) = minix_std::process::waitpid(pid, waitopts.bits() as i32).map_err(errno)?;
    Ok(Pid::from_raw(child).map(|pid| (pid, WaitStatus::new(status))))
}

pub(crate) fn waitid(_id: WaitId<'_>, _options: WaitIdOptions) -> io::Result<Option<WaitIdStatus>> {
    // PM reports an exit status but has no `siginfo_t`-shaped reply.
    Err(io::Errno::NOSYS)
}

pub(crate) fn getsid(pid: Option<Pid>) -> io::Result<Pid> {
    let raw = minix_std::process::getsid(Pid::as_raw(pid)).map_err(errno)?;
    // SAFETY: PM returned a session id, which is a positive process id.
    Ok(unsafe { Pid::from_raw_unchecked(raw) })
}

pub(crate) fn setsid() -> io::Result<Pid> {
    // PM has no call for starting a session.
    Err(io::Errno::NOSYS)
}

pub(crate) fn kill_process(pid: Pid, sig: Signal) -> io::Result<()> {
    minix_std::time::kill(pid.as_raw_nonzero().get(), sig.as_raw()).map_err(errno)
}

pub(crate) fn kill_process_group(pid: Pid, sig: Signal) -> io::Result<()> {
    minix_std::time::kill(pid.as_raw_nonzero().get().wrapping_neg(), sig.as_raw()).map_err(errno)
}

pub(crate) fn kill_current_process_group(sig: Signal) -> io::Result<()> {
    minix_std::time::kill(0, sig.as_raw()).map_err(errno)
}

pub(crate) fn test_kill_process(pid: Pid) -> io::Result<()> {
    minix_std::time::kill(pid.as_raw_nonzero().get(), 0).map_err(errno)
}

pub(crate) fn test_kill_process_group(pid: Pid) -> io::Result<()> {
    minix_std::time::kill(pid.as_raw_nonzero().get().wrapping_neg(), 0).map_err(errno)
}

pub(crate) fn test_kill_current_process_group() -> io::Result<()> {
    minix_std::time::kill(0, 0).map_err(errno)
}

pub(crate) fn fcntl_getlk(_fd: BorrowedFd<'_>, _lock: &Flock) -> io::Result<Option<Flock>> {
    // The VFS has no record locks, which is the same answer `fcntl_lock`
    // gives.
    Err(io::Errno::NOSYS)
}

pub(crate) fn getgroups(buf: &mut [Gid]) -> io::Result<usize> {
    // `Gid` is a transparent wrapper over the same integer PM writes, so the
    // buffer can be handed over as it stands.
    let raw =
        unsafe { core::slice::from_raw_parts_mut(buf.as_mut_ptr().cast::<i32>(), buf.len()) };
    let count = minix_std::process::getgroups(raw).map_err(errno)?;
    Ok(count as usize)
}
