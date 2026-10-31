//! The memory-map group: `mmap`/`munmap`/`mprotect` over the VM server, and
//! the calls the port accepts as no-ops.
//!
//! `mlock`, `munlock`, `msync` and `madvise` succeed without doing anything, as
//! `tools/c-include/sys/mman.h` documents: there is no pageable backing store
//! (every mapped page is resident, so locking has nothing to pin), a shared
//! mapping is coherent without a flush, and the VM has no advice call. They are
//! not stubs: the port's C ABI answers exactly the same way.

pub(crate) mod syscalls;
pub(crate) mod types;
