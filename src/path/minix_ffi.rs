//! The `OsStrExt`/`OsStringExt` the path layer expects, for minix.
//!
//! Minix is not `cfg(unix)`, so neither `std::os::unix::ffi` nor
//! `std::os::wasi::ffi` is available. Both are thin wrappers over the
//! platform-independent `OsStr::as_encoded_bytes` family, so that is what these
//! forward to. The call sites in `arg.rs`/`dec_int.rs` are unchanged.

use std::ffi::{OsStr, OsString};
use std::vec::Vec;

pub(crate) trait OsStrExt {
    fn as_bytes(&self) -> &[u8];
    fn from_bytes(bytes: &[u8]) -> &Self;
}

impl OsStrExt for OsStr {
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self.as_encoded_bytes()
    }

    #[inline]
    fn from_bytes(bytes: &[u8]) -> &Self {
        // SAFETY: `from_encoded_bytes_unchecked` requires only that the bytes
        // round-trip through `as_encoded_bytes`, which is how callers obtain
        // them (`OsStr::as_bytes` above).
        unsafe { OsStr::from_encoded_bytes_unchecked(bytes) }
    }
}

pub(crate) trait OsStringExt: Sized {
    fn from_vec(vec: Vec<u8>) -> OsString;
    fn into_vec(self) -> Vec<u8>;
}

impl OsStringExt for OsString {
    #[inline]
    fn from_vec(vec: Vec<u8>) -> OsString {
        // SAFETY: as `from_bytes` above.
        unsafe { OsString::from_encoded_bytes_unchecked(vec) }
    }

    #[inline]
    fn into_vec(self) -> Vec<u8> {
        self.into_encoded_bytes()
    }
}
