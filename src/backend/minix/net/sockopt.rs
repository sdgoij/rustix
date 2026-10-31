//! The socket options the port answers.
//!
//! The net server stores no options: `setsockopt` is not implemented at all,
//! and `getsockopt` answers only the socket's type. Everything else returns
//! `ENOSYS`, which is what the C ABI behind `tools/c-include/sys/socket.h`
//! reports, rather than `ENOPROTOOPT` that would claim the option is unknown.
#![allow(dead_code)]

use crate::backend::c;
use crate::backend::conv::{borrowed_fd, errno};
use crate::fd::BorrowedFd;
use crate::io;
use crate::net::sockopt::Timeout;
use crate::net::{AddressFamily, Ipv4Addr, Ipv6Addr, SocketType};
use core::time::Duration;

pub(crate) fn socket_type(fd: BorrowedFd<'_>) -> io::Result<SocketType> {
    let raw = borrowed_fd(fd);
    match minix_std::net::socket_type(raw) {
        Ok(kind) => Ok(SocketType::from_raw(kind as u32)),
        // A descriptor the net server cannot name is a local-domain socket,
        // whose type the driver does know.
        Err(_) => minix_std::uds::socket_minor(raw)
            .map(|_| SocketType::from_raw(c::SOCK_STREAM as u32))
            .map_err(errno),
    }
}

fn nosys<T>() -> io::Result<T> {
    Err(io::Errno::NOSYS)
}

pub(crate) fn set_socket_reuseaddr(_fd: BorrowedFd<'_>, _reuseaddr: bool) -> io::Result<()> {
    nosys()
}
pub(crate) fn socket_reuseaddr(_fd: BorrowedFd<'_>) -> io::Result<bool> {
    nosys()
}
pub(crate) fn set_socket_broadcast(_fd: BorrowedFd<'_>, _broadcast: bool) -> io::Result<()> {
    nosys()
}
pub(crate) fn socket_broadcast(_fd: BorrowedFd<'_>) -> io::Result<bool> {
    nosys()
}
pub(crate) fn set_socket_linger(_fd: BorrowedFd<'_>, _linger: Option<Duration>) -> io::Result<()> {
    nosys()
}
pub(crate) fn socket_linger(_fd: BorrowedFd<'_>) -> io::Result<Option<Duration>> {
    nosys()
}
pub(crate) fn set_socket_timeout(
    _fd: BorrowedFd<'_>,
    _id: Timeout,
    _timeout: Option<Duration>,
) -> io::Result<()> {
    nosys()
}
pub(crate) fn socket_timeout(_fd: BorrowedFd<'_>, _id: Timeout) -> io::Result<Option<Duration>> {
    nosys()
}
pub(crate) fn socket_error(_fd: BorrowedFd<'_>) -> io::Result<Result<(), io::Errno>> {
    nosys()
}
pub(crate) fn set_socket_keepalive(_fd: BorrowedFd<'_>, _keepalive: bool) -> io::Result<()> {
    nosys()
}
pub(crate) fn socket_keepalive(_fd: BorrowedFd<'_>) -> io::Result<bool> {
    nosys()
}
pub(crate) fn set_socket_recv_buffer_size(_fd: BorrowedFd<'_>, _size: usize) -> io::Result<()> {
    nosys()
}
pub(crate) fn socket_recv_buffer_size(_fd: BorrowedFd<'_>) -> io::Result<usize> {
    nosys()
}
pub(crate) fn set_socket_send_buffer_size(_fd: BorrowedFd<'_>, _size: usize) -> io::Result<()> {
    nosys()
}
pub(crate) fn socket_send_buffer_size(_fd: BorrowedFd<'_>) -> io::Result<usize> {
    nosys()
}
pub(crate) fn socket_domain(_fd: BorrowedFd<'_>) -> io::Result<AddressFamily> {
    nosys()
}
pub(crate) fn socket_acceptconn(_fd: BorrowedFd<'_>) -> io::Result<bool> {
    nosys()
}
pub(crate) fn set_socket_oobinline(_fd: BorrowedFd<'_>, _value: bool) -> io::Result<()> {
    nosys()
}
pub(crate) fn socket_oobinline(_fd: BorrowedFd<'_>) -> io::Result<bool> {
    nosys()
}
pub(crate) fn set_socket_reuseport(_fd: BorrowedFd<'_>, _value: bool) -> io::Result<()> {
    nosys()
}
pub(crate) fn socket_reuseport(_fd: BorrowedFd<'_>) -> io::Result<bool> {
    nosys()
}
pub(crate) fn set_ip_ttl(_fd: BorrowedFd<'_>, _ttl: u32) -> io::Result<()> {
    nosys()
}
pub(crate) fn ip_ttl(_fd: BorrowedFd<'_>) -> io::Result<u32> {
    nosys()
}
pub(crate) fn set_ipv6_v6only(_fd: BorrowedFd<'_>, _only_v6: bool) -> io::Result<()> {
    nosys()
}
pub(crate) fn ipv6_v6only(_fd: BorrowedFd<'_>) -> io::Result<bool> {
    nosys()
}
pub(crate) fn set_ip_multicast_if(_fd: BorrowedFd<'_>, _value: &Ipv4Addr) -> io::Result<()> {
    nosys()
}
pub(crate) fn ip_multicast_if(_fd: BorrowedFd<'_>) -> io::Result<Ipv4Addr> {
    nosys()
}
pub(crate) fn set_ipv6_multicast_if(_fd: BorrowedFd<'_>, _value: u32) -> io::Result<()> {
    nosys()
}
pub(crate) fn ipv6_multicast_if(_fd: BorrowedFd<'_>) -> io::Result<u32> {
    nosys()
}
pub(crate) fn set_ip_multicast_loop(_fd: BorrowedFd<'_>, _value: bool) -> io::Result<()> {
    nosys()
}
pub(crate) fn ip_multicast_loop(_fd: BorrowedFd<'_>) -> io::Result<bool> {
    nosys()
}
pub(crate) fn set_ip_multicast_ttl(_fd: BorrowedFd<'_>, _value: u32) -> io::Result<()> {
    nosys()
}
pub(crate) fn ip_multicast_ttl(_fd: BorrowedFd<'_>) -> io::Result<u32> {
    nosys()
}
pub(crate) fn set_ipv6_multicast_loop(_fd: BorrowedFd<'_>, _value: bool) -> io::Result<()> {
    nosys()
}
pub(crate) fn ipv6_multicast_loop(_fd: BorrowedFd<'_>) -> io::Result<bool> {
    nosys()
}
pub(crate) fn ipv6_unicast_hops(_fd: BorrowedFd<'_>) -> io::Result<u8> {
    nosys()
}
pub(crate) fn set_ipv6_unicast_hops(_fd: BorrowedFd<'_>, _hops: Option<u8>) -> io::Result<()> {
    nosys()
}
pub(crate) fn set_ipv6_multicast_hops(_fd: BorrowedFd<'_>, _hops: u32) -> io::Result<()> {
    nosys()
}
pub(crate) fn ipv6_multicast_hops(_fd: BorrowedFd<'_>) -> io::Result<u32> {
    nosys()
}
pub(crate) fn set_ip_add_membership(
    _fd: BorrowedFd<'_>,
    _multiaddr: &Ipv4Addr,
    _interface: &Ipv4Addr,
) -> io::Result<()> {
    nosys()
}
pub(crate) fn set_ipv6_add_membership(
    _fd: BorrowedFd<'_>,
    _multiaddr: &Ipv6Addr,
    _interface: u32,
) -> io::Result<()> {
    nosys()
}
pub(crate) fn set_ip_drop_membership(
    _fd: BorrowedFd<'_>,
    _multiaddr: &Ipv4Addr,
    _interface: &Ipv4Addr,
) -> io::Result<()> {
    nosys()
}
pub(crate) fn set_ipv6_drop_membership(
    _fd: BorrowedFd<'_>,
    _multiaddr: &Ipv6Addr,
    _interface: u32,
) -> io::Result<()> {
    nosys()
}
pub(crate) fn set_ipv6_tclass(_fd: BorrowedFd<'_>, _value: u32) -> io::Result<()> {
    nosys()
}
pub(crate) fn ipv6_tclass(_fd: BorrowedFd<'_>) -> io::Result<u32> {
    nosys()
}
pub(crate) fn set_tcp_nodelay(_fd: BorrowedFd<'_>, _nodelay: bool) -> io::Result<()> {
    nosys()
}
pub(crate) fn tcp_nodelay(_fd: BorrowedFd<'_>) -> io::Result<bool> {
    nosys()
}
pub(crate) fn set_tcp_keepcnt(_fd: BorrowedFd<'_>, _count: u32) -> io::Result<()> {
    nosys()
}
pub(crate) fn tcp_keepcnt(_fd: BorrowedFd<'_>) -> io::Result<u32> {
    nosys()
}
pub(crate) fn set_tcp_keepidle(_fd: BorrowedFd<'_>, _duration: Duration) -> io::Result<()> {
    nosys()
}
pub(crate) fn tcp_keepidle(_fd: BorrowedFd<'_>) -> io::Result<Duration> {
    nosys()
}
pub(crate) fn set_tcp_keepintvl(_fd: BorrowedFd<'_>, _duration: Duration) -> io::Result<()> {
    nosys()
}
pub(crate) fn tcp_keepintvl(_fd: BorrowedFd<'_>) -> io::Result<Duration> {
    nosys()
}
