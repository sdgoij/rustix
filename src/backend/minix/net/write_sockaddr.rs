//! Encoding addresses into the C shapes the syscalls take.
#![allow(unsafe_code)]

use crate::backend::c;
use crate::net::{SocketAddrV4, SocketAddrV6};

pub(crate) fn encode_sockaddr_v4(v4: &SocketAddrV4) -> c::sockaddr_in {
    c::sockaddr_in {
        sin_family: c::AF_INET as _,
        sin_port: u16::to_be(v4.port()),
        sin_addr: c::in_addr {
            s_addr: u32::from_ne_bytes(v4.ip().octets()),
        },
        sin_zero: [0_u8; 8],
    }
}

pub(crate) fn encode_sockaddr_v6(v6: &SocketAddrV6) -> c::sockaddr_in6 {
    c::sockaddr_in6 {
        sin6_family: c::AF_INET6 as _,
        sin6_port: u16::to_be(v6.port()),
        sin6_flowinfo: u32::to_be(v6.flowinfo()),
        sin6_addr: c::in6_addr {
            s6_addr: v6.ip().octets(),
        },
        sin6_scope_id: v6.scope_id(),
    }
}
