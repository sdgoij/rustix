//! The two parameters `rustix::param` asks for.
//!
//! They are constants on this port rather than values read from a kernel
//! auxiliary vector: the page size is fixed at 4 KiB, and the tick rate is the
//! one `sysconf(_SC_CLK_TCK)` reports.

/// The port's page size.
pub(crate) fn page_size() -> usize {
    4096
}

/// The tick rate `times()` counts in.
pub(crate) fn clock_ticks_per_second() -> u64 {
    60
}
