//! POSIX shared memory.
//!
//! The port has no `shm_open`: `tools/c-include/sys/mman.h` declares the pair
//! for ABI compatibility and the shared-memory route is the `memfd` one
//! (`crate::fs::memfd_create`), which is a descriptor rather than a name. Both
//! calls report `ENOSYS`, the same answer the C ABI gives.

pub(crate) mod syscalls;
pub(crate) mod types;
