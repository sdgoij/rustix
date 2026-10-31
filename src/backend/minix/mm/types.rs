use crate::backend::c;
use bitflags::bitflags;

bitflags! {
    /// `PROT_*` flags for use with [`mmap`].
    ///
    /// For `PROT_NONE`, use `ProtFlags::empty()`.
    ///
    /// [`mmap`]: crate::mm::mmap
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct ProtFlags: u32 {
        /// `PROT_READ`
        const READ = bitcast!(c::PROT_READ);
        /// `PROT_WRITE`
        const WRITE = bitcast!(c::PROT_WRITE);
        /// `PROT_EXEC`
        const EXEC = bitcast!(c::PROT_EXEC);

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

bitflags! {
    /// `PROT_*` flags for use with [`mprotect`].
    ///
    /// For `PROT_NONE`, use `MprotectFlags::empty()`.
    ///
    /// [`mprotect`]: crate::mm::mprotect
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct MprotectFlags: u32 {
        /// `PROT_READ`
        const READ = bitcast!(c::PROT_READ);
        /// `PROT_WRITE`
        const WRITE = bitcast!(c::PROT_WRITE);
        /// `PROT_EXEC`
        const EXEC = bitcast!(c::PROT_EXEC);

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

bitflags! {
    /// `MAP_*` flags for use with [`mmap`].
    ///
    /// For `MAP_ANONYMOUS` (aka `MAP_ANON`), see [`mmap_anonymous`].
    ///
    /// [`mmap`]: crate::mm::mmap
    /// [`mmap_anonymous`]: crates::mm::mmap_anonymous
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct MapFlags: u32 {
        /// `MAP_SHARED`
        const SHARED = bitcast!(c::MAP_SHARED);
        /// `MAP_PRIVATE`
        const PRIVATE = bitcast!(c::MAP_PRIVATE);
        /// `MAP_FIXED`
        const FIXED = bitcast!(c::MAP_FIXED);

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

bitflags! {
    /// `MS_*` flags for use with [`msync`].
    ///
    /// [`msync`]: crate::mm::msync
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct MsyncFlags: u32 {
        /// `MS_SYNC`—Requests an update and waits for it to complete.
        const SYNC = bitcast!(c::MS_SYNC);
        /// `MS_ASYNC`—Specifies that an update be scheduled, but the call
        /// returns immediately.
        const ASYNC = bitcast!(c::MS_ASYNC);
        /// `MS_INVALIDATE`—Asks to invalidate other mappings of the same
        /// file (so that they can be updated with the fresh values just
        /// written).
        const INVALIDATE = bitcast!(c::MS_INVALIDATE);

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

/// `POSIX_MADV_*` constants for use with [`madvise`].
///
/// [`madvise`]: crate::mm::madvise
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
#[non_exhaustive]
pub enum Advice {
    /// `POSIX_MADV_NORMAL`
    Normal = bitcast!(c::POSIX_MADV_NORMAL),
    /// `POSIX_MADV_SEQUENTIAL`
    Sequential = bitcast!(c::POSIX_MADV_SEQUENTIAL),
    /// `POSIX_MADV_RANDOM`
    Random = bitcast!(c::POSIX_MADV_RANDOM),
    /// `POSIX_MADV_WILLNEED`
    WillNeed = bitcast!(c::POSIX_MADV_WILLNEED),
    /// `POSIX_MADV_DONTNEED`
    DontNeed = bitcast!(c::POSIX_MADV_DONTNEED),
}
