//! The minix backend.
//!
//! minix is a microkernel: there is no kernel ABI to call, and no C library on
//! the other side of one. Every operation is a message to a server, which
//! `minix-std` wraps, so this backend calls that crate directly and keeps its
//! own copy of the C-level names the shared layer above it expects.

#![allow(clippy::undocumented_unsafe_blocks)]

pub(crate) mod c;
mod conv;

pub(crate) mod fd {
    pub use crate::maybe_polyfill::os::fd::*;
    #[allow(unused_imports)]
    pub(crate) use RawFd as LibcFd;
}

pub(crate) mod io;

#[cfg(feature = "fs")]
pub(crate) mod fs;

#[cfg(feature = "system")]
pub(crate) mod system;

#[cfg(feature = "pipe")]
pub(crate) mod pipe;

/// The minimum `IOV_MAX` POSIX requires. The VFS has no `readv`, so the
/// scatter/gather calls walk this many slices at most, as on any target
/// without the vector calls.
const MAX_IOV: usize = 16;
