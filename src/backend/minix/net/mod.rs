//! The socket family over the port's two devices: the net server's
//! `/dev/tcp`/`/dev/udp` and the local-domain `/dev/uds`.
//!
//! The address types and the `MSG_*` flags are shared shapes; `syscalls`
//! chooses a device per call, because a descriptor here does not say which one
//! it is.

pub(crate) mod addr;
pub(crate) mod msghdr;
pub(crate) mod read_sockaddr;
pub(crate) mod send_recv;
pub(crate) mod sockopt;
pub(crate) mod syscalls;
pub(crate) mod write_sockaddr;
