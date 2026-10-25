//! Filesystem path operations.

mod arg;
mod dec_int;
#[cfg(all(feature = "std", target_os = "minix"))]
pub(crate) mod minix_ffi;

pub use arg::{option_into_with_c_str, Arg};
pub use dec_int::{DecInt, Integer};

pub(crate) const SMALL_PATH_BUFFER_SIZE: usize = 256;
