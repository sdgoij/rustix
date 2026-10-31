//! minix syscalls for user and group ids.
//!
//! PM keeps a real id and an effective id for each, and hands back both, so
//! the pair it returns is the whole answer.

use crate::ugid::{Gid, RawGid, RawUid, Uid};

#[inline]
#[must_use]
pub(crate) fn getuid() -> Uid {
    let (uid, _) = minix_std::process::getuid().unwrap_or((0, 0));
    Uid::from_raw(uid as RawUid)
}

#[inline]
#[must_use]
pub(crate) fn geteuid() -> Uid {
    let (_, euid) = minix_std::process::getuid().unwrap_or((0, 0));
    Uid::from_raw(euid as RawUid)
}

#[inline]
#[must_use]
pub(crate) fn getgid() -> Gid {
    let (gid, _) = minix_std::process::getgid().unwrap_or((0, 0));
    Gid::from_raw(gid as RawGid)
}

#[inline]
#[must_use]
pub(crate) fn getegid() -> Gid {
    let (_, egid) = minix_std::process::getgid().unwrap_or((0, 0));
    Gid::from_raw(egid as RawGid)
}
