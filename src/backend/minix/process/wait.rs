//! The `wait` flag and status names the public layer reads.
//!
//! They are the macros `tools/c-include/sys/wait.h` publishes, with the same
//! `w_exitcode` encoding. PM has no stop or continue encoding, so a child is
//! never reported as stopped or continued, and `WIFCONTINUED` is always false.

pub(crate) use crate::backend::c::{
    WCONTINUED, WEXITED, WEXITSTATUS, WIFCONTINUED, WIFEXITED, WIFSIGNALED, WIFSTOPPED, WNOHANG,
    WNOWAIT, WSTOPPED, WSTOPSIG, WTERMSIG, WUNTRACED,
};
