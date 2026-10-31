//! `makedev`, `major` and `minor`.
//!
//! The port encodes a device as the major number in the high 16 bits and the
//! minor in the low 16, which is the encoding `tools/c-include/sys/types.h`
//! publishes and the one the VFS compares against.

use crate::fs::Dev;

#[inline]
pub(crate) fn makedev(maj: u32, min: u32) -> Dev {
    ((Dev::from(maj) & 0xffff) << 16) | (Dev::from(min) & 0xffff)
}

#[inline]
pub(crate) fn major(dev: Dev) -> u32 {
    (dev >> 16) as u32
}

#[inline]
pub(crate) fn minor(dev: Dev) -> u32 {
    (dev & 0xffff) as u32
}
