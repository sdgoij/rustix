//! Types for the `termios` module.

#![allow(non_camel_case_types)]

/// `tcflag_t`, which the port's `termios.h` makes `unsigned int`.
pub type tcflag_t = u32;
