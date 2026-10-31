//! Decoding the `sockaddr` a server writes back.
#![allow(unsafe_code)]

use crate::backend::c;
use crate::io::Errno;
use crate::net::addr::SocketAddrLen;
use crate::net::{
    AddressFamily, Ipv4Addr, Ipv6Addr, SocketAddrAny, SocketAddrUnix, SocketAddrV4, SocketAddrV6,
};
use core::mem::size_of;
use core::slice;

// This must match the header of `sockaddr`.
#[repr(C)]
pub(crate) struct sockaddr_header {
    sa_family: u16,
}

/// Read the `sa_family` field from a socket address returned from the OS.
///
/// # Safety
///
/// `storage` must point to a least an initialized `sockaddr_header`.
#[inline]
pub(crate) unsafe fn read_sa_family(storage: *const c::sockaddr) -> u16 {
    // Assert that we know the layout of `sockaddr`.
    let _ = c::sockaddr {
        sa_family: 0_u16,
        sa_data: [0; 14],
    };

    unsafe { (*storage.cast::<sockaddr_header>()).sa_family }
}

/// Set the `sa_family` field of a socket address to `AF_UNSPEC`, so that we
/// can test for `AF_UNSPEC` to test whether it was stored to.
///
/// # Safety
///
/// `storage` must point to a least an initialized `sockaddr_header`.
#[inline]
pub(crate) unsafe fn initialize_family_to_unspec(storage: *mut c::sockaddr) {
    unsafe { (*storage.cast::<sockaddr_header>()).sa_family = c::AF_UNSPEC as _ };
}

/// Check if a socket address returned from the OS is considered non-empty.
#[inline]
pub(crate) unsafe fn sockaddr_nonempty(_storage: *const c::sockaddr, len: SocketAddrLen) -> bool {
    len != 0
}

#[inline]
pub(crate) fn read_sockaddr_v4(addr: &SocketAddrAny) -> Result<SocketAddrV4, Errno> {
    if addr.address_family() != AddressFamily::INET {
        return Err(Errno::AFNOSUPPORT);
    }
    assert!(addr.addr_len() as usize >= size_of::<c::sockaddr_in>());
    let decode = unsafe { &*addr.as_ptr().cast::<c::sockaddr_in>() };
    Ok(SocketAddrV4::new(
        Ipv4Addr::from(decode.sin_addr.s_addr.to_ne_bytes()),
        u16::from_be(decode.sin_port),
    ))
}

#[inline]
pub(crate) fn read_sockaddr_v6(addr: &SocketAddrAny) -> Result<SocketAddrV6, Errno> {
    if addr.address_family() != AddressFamily::INET6 {
        return Err(Errno::AFNOSUPPORT);
    }
    assert!(addr.addr_len() as usize >= size_of::<c::sockaddr_in6>());
    let decode = unsafe { &*addr.as_ptr().cast::<c::sockaddr_in6>() };
    Ok(SocketAddrV6::new(
        Ipv6Addr::from(decode.sin6_addr.s6_addr),
        u16::from_be(decode.sin6_port),
        u32::from_be(decode.sin6_flowinfo),
        decode.sin6_scope_id,
    ))
}

#[inline]
pub(crate) fn read_sockaddr_unix(addr: &SocketAddrAny) -> Result<SocketAddrUnix, Errno> {
    if addr.address_family() != AddressFamily::UNIX {
        return Err(Errno::AFNOSUPPORT);
    }
    let offset = super::addr::offsetof_sun_path();
    let len = addr.addr_len() as usize;

    assert!(len >= offset);

    if len == offset {
        Ok(SocketAddrUnix::new_unnamed())
    } else {
        let decode = unsafe { &*addr.as_ptr().cast::<c::sockaddr_un>() };

        if decode.sun_path[0] == 0 {
            let bytes = &decode.sun_path[1..len - offset];

            // SAFETY: Convert `&[c_char]` to `&[u8]`.
            let bytes = unsafe { slice::from_raw_parts(bytes.as_ptr().cast::<u8>(), bytes.len()) };

            return SocketAddrUnix::new_abstract_name(bytes);
        }

        // Otherwise we expect a NUL-terminated filesystem path.
        let bytes = &decode.sun_path[..len - 1 - offset];

        // SAFETY: Convert `&[c_char]` to `&[u8]`.
        let bytes = unsafe { slice::from_raw_parts(bytes.as_ptr().cast::<u8>(), bytes.len()) };

        assert_eq!(decode.sun_path[len - 1 - offset], 0);
        SocketAddrUnix::new(bytes)
    }
}
