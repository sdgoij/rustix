//! The thread group: sleep and yield.
//!
//! The port has no sleeping syscall, so a sleep is a yield loop against a clock
//! (see `syscalls`). The rest of rustix's thread surface is Linux-specific and
//! stays out.

pub(crate) mod syscalls;
