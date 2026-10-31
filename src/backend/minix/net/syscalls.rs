//! minix syscalls supporting `rustix::net`.
//!
//! The port has two socket families and they are different devices: `/dev/tcp`
//! and `/dev/udp` speak the net server's `NWIO*` ioctls, while `/dev/uds` is a
//! local-domain socket whose data path is `read`/`write` and whose control data
//! moves through the `NWIOSUDSCTRL`/`NWIOGUDSCTRL` ioctls. `minix_std` names
//! both; this module asks the net server which one an fd is
//! (`net::socket_type`), because the descriptor alone does not say.

use crate::backend::c;
use crate::backend::conv::{borrowed_fd, errno, owned_fd_from_raw};
use crate::fd::{BorrowedFd, OwnedFd};
use crate::io::{self, IoSlice, IoSliceMut};
use crate::net::addr::SocketAddrArg;
use crate::net::{
    AddressFamily, Protocol, RecvAncillaryBuffer, RecvFlags, RecvMsg, ReturnFlags,
    SendAncillaryBuffer, SendFlags, Shutdown, SocketAddrAny, SocketAddrBuf, SocketFlags, SocketType,
};
use core::mem::size_of;
use core::slice;

/// The flags the port's message path honours. A socket here never blocks, so
/// `MSG_DONTWAIT` asks for what a read or write already does, and no signal is
/// delivered to userspace, so `MSG_NOSIGNAL` has nothing to suppress.
const SEND_RECV_FLAGS: u32 = (c::MSG_DONTWAIT | c::MSG_NOSIGNAL) as u32;

fn check_flags(bits: u32, extra: u32) -> io::Result<()> {
    if bits & !(SEND_RECV_FLAGS | extra) != 0 {
        return Err(io::Errno::OPNOTSUPP);
    }
    Ok(())
}

/// Which device an fd is: the net server's, or the local-domain one.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Inet,
    Local,
}

fn kind(fd: i32) -> Kind {
    match minix_std::net::socket_type(fd) {
        Ok(_) => Kind::Inet,
        Err(_) => Kind::Local,
    }
}

/// The `struct sockaddr` a caller passed, decoded enough to name a peer.
enum Decoded<'a> {
    Unix(&'a [u8]),
    Inet([u8; 4], u16),
}

/// # Safety
///
/// `ptr` must point to at least `len` readable bytes whose `sa_family` says how
/// to read the rest.
unsafe fn decode<'a>(ptr: *const c::sockaddr, len: usize) -> io::Result<Decoded<'a>> {
    match unsafe { (*ptr).sa_family as c::c_int } {
        c::AF_UNIX => {
            let offset = super::addr::offsetof_sun_path();
            if len <= offset {
                return Err(io::Errno::INVAL);
            }
            let addr = unsafe { &*ptr.cast::<c::sockaddr_un>() };
            let bytes = &addr.sun_path[..len - offset];
            if bytes[0] == 0 {
                // An abstract name has no filesystem node to bind, and the
                // port's local-domain socket is named by one.
                return Err(io::Errno::AFNOSUPPORT);
            }
            let n = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
            // SAFETY: the bytes live in the caller's address for the length of
            // the closure the address was passed in.
            let path = unsafe { slice::from_raw_parts(bytes.as_ptr().cast::<u8>(), n) };
            Ok(Decoded::Unix(path))
        }
        c::AF_INET => {
            if len < size_of::<c::sockaddr_in>() {
                return Err(io::Errno::INVAL);
            }
            let addr = unsafe { &*ptr.cast::<c::sockaddr_in>() };
            Ok(Decoded::Inet(
                addr.sin_addr.s_addr.to_ne_bytes(),
                u16::from_be(addr.sin_port),
            ))
        }
        _ => Err(io::Errno::AFNOSUPPORT),
    }
}

fn inet_any(ip: [u8; 4], port: u16) -> SocketAddrAny {
    let mut buf = SocketAddrBuf::new();
    let addr = c::sockaddr_in {
        sin_family: c::AF_INET as _,
        sin_port: port.to_be(),
        sin_addr: c::in_addr {
            s_addr: u32::from_ne_bytes(ip),
        },
        sin_zero: [0; 8],
    };
    unsafe {
        buf.storage.as_mut_ptr().cast::<c::sockaddr_in>().write(addr);
        buf.len = size_of::<c::sockaddr_in>() as c::socklen_t;
        buf.into_any()
    }
}

fn unix_any(wire: &[u8]) -> SocketAddrAny {
    let mut buf = SocketAddrBuf::new();
    unsafe {
        let storage = buf.storage.as_mut_ptr().cast::<u8>();
        core::ptr::copy_nonoverlapping(wire.as_ptr(), storage, wire.len());
        buf.len = wire.len() as c::socklen_t;
        buf.into_any()
    }
}

fn peer_any(fd: i32) -> Option<SocketAddrAny> {
    if let Ok((ip, port)) = minix_std::net::getpeername(fd) {
        return Some(inet_any(ip, port));
    }
    let wire = minix_std::uds::peer_name(fd).ok()?;
    // A peer that never bound answers a zeroed address, family included.
    if wire[..2] == [0, 0] {
        return None;
    }
    Some(unix_any(&wire))
}

pub(crate) fn socket(
    domain: AddressFamily,
    type_: SocketType,
    protocol: Option<Protocol>,
) -> io::Result<OwnedFd> {
    socket_with(domain, type_, SocketFlags::empty(), protocol)
}

pub(crate) fn socket_with(
    domain: AddressFamily,
    type_: SocketType,
    flags: SocketFlags,
    protocol: Option<Protocol>,
) -> io::Result<OwnedFd> {
    let raw_protocol = match protocol {
        Some(p) => p.0.get() as c::c_int,
        None => 0,
    };
    let raw_domain = domain.0 as c::c_int;
    let fd = if raw_domain == c::AF_UNIX {
        minix_std::uds::socket()
    } else {
        minix_std::net::socket(
            raw_domain,
            (type_.0 | flags.bits()) as c::c_int,
            raw_protocol,
        )
    }
    .map_err(errno)?;
    if raw_domain == c::AF_UNIX {
        apply_flags(fd, flags)?;
    }
    // SAFETY: `socket` hands back a descriptor the caller now owns.
    Ok(unsafe { owned_fd_from_raw(fd) })
}

fn apply_flags(fd: i32, flags: SocketFlags) -> io::Result<()> {
    if flags.contains(SocketFlags::CLOEXEC) {
        minix_std::fs::fcntl(fd, c::F_SETFD, c::FD_CLOEXEC).map_err(errno)?;
    }
    if flags.contains(SocketFlags::NONBLOCK) {
        minix_std::fs::fcntl(fd, c::F_SETFL, c::O_NONBLOCK).map_err(errno)?;
    }
    Ok(())
}

pub(crate) fn bind(sockfd: BorrowedFd<'_>, addr: &impl SocketAddrArg) -> io::Result<()> {
    let fd = borrowed_fd(sockfd);
    // SAFETY: `with_sockaddr` guarantees the pointer is readable for `len`.
    unsafe {
        addr.with_sockaddr(|ptr, len| match decode(ptr.cast(), len as usize)? {
            Decoded::Unix(path) => minix_std::uds::bind(fd, path).map_err(errno),
            Decoded::Inet(ip, port) => minix_std::net::bind(fd, ip, port).map_err(errno),
        })
    }
}

pub(crate) fn connect(sockfd: BorrowedFd<'_>, addr: &impl SocketAddrArg) -> io::Result<()> {
    let fd = borrowed_fd(sockfd);
    // SAFETY: `with_sockaddr` guarantees the pointer is readable for `len`.
    unsafe {
        addr.with_sockaddr(|ptr, len| match decode(ptr.cast(), len as usize)? {
            Decoded::Unix(path) => minix_std::uds::connect(fd, path).map_err(errno),
            Decoded::Inet(ip, port) => minix_std::net::connect(fd, ip, port).map_err(errno),
        })
    }
}

pub(crate) fn connect_unspec(_sockfd: BorrowedFd<'_>) -> io::Result<()> {
    // The net server has no way to dissolve a UDP association; a connect is
    // only ever added to one.
    Err(io::Errno::OPNOTSUPP)
}

pub(crate) fn listen(sockfd: BorrowedFd<'_>, backlog: c::c_int) -> io::Result<()> {
    let fd = borrowed_fd(sockfd);
    match kind(fd) {
        Kind::Inet => minix_std::net::listen(fd, backlog).map_err(errno),
        Kind::Local => minix_std::uds::listen(fd, backlog).map_err(errno),
    }
}

fn accept_fd(fd: i32) -> io::Result<i32> {
    match minix_std::net::socket_type(fd) {
        Ok(minix_std::net::SOCK_STREAM) => minix_std::net::accept(fd).map_err(errno),
        Ok(_) => Err(io::Errno::OPNOTSUPP),
        Err(_) => minix_std::uds::accept(fd).map_err(errno),
    }
}

pub(crate) fn accept(sockfd: BorrowedFd<'_>) -> io::Result<OwnedFd> {
    let fd = accept_fd(borrowed_fd(sockfd))?;
    // SAFETY: `accept` hands back a descriptor the caller now owns.
    Ok(unsafe { owned_fd_from_raw(fd) })
}

pub(crate) fn accept_with(sockfd: BorrowedFd<'_>, flags: SocketFlags) -> io::Result<OwnedFd> {
    let fd = accept_fd(borrowed_fd(sockfd))?;
    apply_flags(fd, flags)?;
    // SAFETY: `accept` hands back a descriptor the caller now owns.
    Ok(unsafe { owned_fd_from_raw(fd) })
}

pub(crate) fn acceptfrom(sockfd: BorrowedFd<'_>) -> io::Result<(OwnedFd, Option<SocketAddrAny>)> {
    let fd = accept_fd(borrowed_fd(sockfd))?;
    let addr = peer_any(fd);
    // SAFETY: `accept` hands back a descriptor the caller now owns.
    Ok((unsafe { owned_fd_from_raw(fd) }, addr))
}

pub(crate) fn acceptfrom_with(
    sockfd: BorrowedFd<'_>,
    flags: SocketFlags,
) -> io::Result<(OwnedFd, Option<SocketAddrAny>)> {
    let fd = accept_fd(borrowed_fd(sockfd))?;
    apply_flags(fd, flags)?;
    let addr = peer_any(fd);
    // SAFETY: `accept` hands back a descriptor the caller now owns.
    Ok((unsafe { owned_fd_from_raw(fd) }, addr))
}

pub(crate) fn shutdown(sockfd: BorrowedFd<'_>, how: Shutdown) -> io::Result<()> {
    let fd = borrowed_fd(sockfd);
    match kind(fd) {
        Kind::Inet => minix_std::net::shutdown(fd, how as c::c_int).map_err(errno),
        // `/dev/uds` has no shutdown ioctl: the local-domain socket is closed.
        Kind::Local => Err(io::Errno::OPNOTSUPP),
    }
}

pub(crate) fn getsockname(sockfd: BorrowedFd<'_>) -> io::Result<SocketAddrAny> {
    let fd = borrowed_fd(sockfd);
    match kind(fd) {
        Kind::Inet => {
            let (ip, port) = minix_std::net::getsockname(fd).map_err(errno)?;
            Ok(inet_any(ip, port))
        }
        Kind::Local => {
            let wire = minix_std::uds::getsockname(fd).map_err(errno)?;
            Ok(unix_any(&wire))
        }
    }
}

pub(crate) fn getpeername(sockfd: BorrowedFd<'_>) -> io::Result<Option<SocketAddrAny>> {
    Ok(peer_any(borrowed_fd(sockfd)))
}

pub(crate) fn send(sockfd: BorrowedFd<'_>, buf: &[u8], flags: SendFlags) -> io::Result<usize> {
    check_flags(flags.bits(), 0)?;
    let fd = borrowed_fd(sockfd);
    match kind(fd) {
        // SAFETY: `buf` is a valid slice for the length passed.
        Kind::Inet => unsafe { minix_std::net::send(fd, buf) }
            .map_err(errno)
            .and_then(|n| usize::try_from(n).map_err(|_| io::Errno::INVAL)),
        Kind::Local => minix_std::uds::send(fd, buf)
            .map_err(errno)
            .and_then(|n| usize::try_from(n).map_err(|_| io::Errno::INVAL)),
    }
}

pub(crate) unsafe fn recv(
    sockfd: BorrowedFd<'_>,
    buf: (*mut u8, usize),
    flags: RecvFlags,
) -> io::Result<usize> {
    check_flags(flags.bits(), c::MSG_CMSG_CLOEXEC as u32)?;
    let (ptr, len) = buf;
    let fd = borrowed_fd(sockfd);
    // SAFETY: the caller passed a writable buffer of `len` bytes.
    let slice = unsafe { slice::from_raw_parts_mut(ptr, len) };
    match kind(fd) {
        // SAFETY: `slice` is valid for its length.
        Kind::Inet => unsafe { minix_std::net::recv(fd, slice) }
            .map_err(errno)
            .and_then(|n| usize::try_from(n).map_err(|_| io::Errno::INVAL)),
        Kind::Local => minix_std::uds::recv(fd, slice)
            .map_err(errno)
            .and_then(|n| usize::try_from(n).map_err(|_| io::Errno::INVAL)),
    }
}

pub(crate) fn sendto(
    sockfd: BorrowedFd<'_>,
    buf: &[u8],
    flags: SendFlags,
    addr: &impl SocketAddrArg,
) -> io::Result<usize> {
    check_flags(flags.bits(), 0)?;
    let fd = borrowed_fd(sockfd);
    // SAFETY: `with_sockaddr` guarantees the pointer is readable, and `buf` is
    // valid for its length.
    unsafe {
        addr.with_sockaddr(|ptr, len| match decode(ptr.cast(), len as usize)? {
            Decoded::Inet(ip, port) => {
                let dest = minix_std::net::SocketAddr { ip, port };
                minix_std::net::sendto(fd, buf, Some(dest))
                    .map_err(errno)
                    .and_then(|n| usize::try_from(n).map_err(|_| io::Errno::INVAL))
            }
            // The local-domain socket has no sendto form: it is connected.
            Decoded::Unix(_) => Err(io::Errno::AFNOSUPPORT),
        })
    }
}

pub(crate) unsafe fn recvfrom(
    sockfd: BorrowedFd<'_>,
    buf: (*mut u8, usize),
    flags: RecvFlags,
) -> io::Result<(usize, Option<SocketAddrAny>)> {
    check_flags(flags.bits(), c::MSG_CMSG_CLOEXEC as u32)?;
    let (ptr, len) = buf;
    let fd = borrowed_fd(sockfd);
    if kind(fd) != Kind::Inet {
        return Err(io::Errno::AFNOSUPPORT);
    }
    // SAFETY: the caller passed a writable buffer of `len` bytes.
    let slice = unsafe { slice::from_raw_parts_mut(ptr, len) };
    // SAFETY: `slice` is valid for its length.
    let (n, src) = unsafe { minix_std::net::recvfrom(fd, slice) }.map_err(errno)?;
    let n = usize::try_from(n).map_err(|_| io::Errno::INVAL)?;
    // A datagram carries its sender; a zero-length read has none to report.
    let addr = (n > 0).then(|| inet_any(src.ip, src.port));
    Ok((n, addr))
}

pub(crate) fn sendmsg(
    sockfd: BorrowedFd<'_>,
    iov: &[IoSlice<'_>],
    control: &mut SendAncillaryBuffer<'_, '_, '_>,
    flags: SendFlags,
) -> io::Result<usize> {
    check_flags(flags.bits(), 0)?;
    let fd = borrowed_fd(sockfd);
    let control_len = control.control_len();
    if control_len > 0 {
        if kind(fd) == Kind::Inet {
            // Control data is the local-domain socket's protocol.
            return Err(io::Errno::OPNOTSUPP);
        }
        // SAFETY: the buffer holds `control_len` initialized bytes.
        let bytes = unsafe { slice::from_raw_parts(control.as_control_ptr(), control_len) };
        minix_std::uds::send_control_raw(fd, bytes).map_err(errno)?;
    }
    write_body(fd, iov)
}

pub(crate) fn sendmsg_addr(
    sockfd: BorrowedFd<'_>,
    addr: &impl SocketAddrArg,
    iov: &[IoSlice<'_>],
    control: &mut SendAncillaryBuffer<'_, '_, '_>,
    flags: SendFlags,
) -> io::Result<usize> {
    // A datagram to an explicit address is `sendto`; the port's message path
    // carries control data only on a connected local-domain socket, which has
    // no address form.
    let _ = (sockfd, addr, iov, control, flags);
    Err(io::Errno::OPNOTSUPP)
}

pub(crate) fn recvmsg(
    sockfd: BorrowedFd<'_>,
    iov: &mut [IoSliceMut<'_>],
    control: &mut RecvAncillaryBuffer<'_>,
    flags: RecvFlags,
) -> io::Result<RecvMsg> {
    check_flags(flags.bits(), c::MSG_CMSG_CLOEXEC as u32)?;
    let fd = borrowed_fd(sockfd);
    control.clear();
    let bytes = read_body(fd, iov)?;

    let control_len = control.control_len();
    if control_len > 0 {
        // SAFETY: the caller's ancillary buffer is `control_len` bytes.
        let buf = unsafe { slice::from_raw_parts_mut(control.as_control_ptr(), control_len) };
        let got = minix_std::uds::recv_control_raw(fd, buf, control_len).map_err(errno)?;
        if got > 0 && flags.contains(RecvFlags::CMSG_CLOEXEC) {
            set_cmsg_cloexec(&buf[..got]);
        }
        // SAFETY: `got` bytes of the caller's ancillary buffer were just
        // initialized by the driver.
        unsafe { control.set_control_len(got) };
    }

    Ok(RecvMsg {
        bytes,
        flags: ReturnFlags::empty(),
        address: None,
    })
}

/// Apply `MSG_CMSG_CLOEXEC` to the descriptors a `recvmsg` just lifted out.
///
/// The `/dev/uds` driver hands the descriptors over without the flag, and this
/// port has no receive-time equivalent of the kernel's atomic set, so it is
/// applied here, after the body has moved.
fn set_cmsg_cloexec(control: &[u8]) {
    let mut off = 0;
    while off + size_of::<c::cmsghdr>() <= control.len() {
        // SAFETY: `off + size_of::<cmsghdr>()` was just checked against the
        // buffer length, and the chain is the driver's own bytes.
        let hdr = unsafe { &*control.as_ptr().add(off).cast::<c::cmsghdr>() };
        let len = hdr.cmsg_len as usize;
        if len < size_of::<c::cmsghdr>() || off + len > control.len() {
            return;
        }
        if hdr.cmsg_level == c::SOL_SOCKET && hdr.cmsg_type == c::SCM_RIGHTS {
            let data = &control[off + c::CMSG_ALIGN(size_of::<c::cmsghdr>())..off + len];
            for chunk in data.chunks_exact(4) {
                let fd = i32::from_ne_bytes(chunk.try_into().unwrap_or([0; 4]));
                let _ = minix_std::fs::fcntl(fd, c::F_SETFD, c::FD_CLOEXEC);
            }
        }
        off += c::CMSG_ALIGN(len);
    }
}

fn write_body(fd: i32, iov: &[IoSlice<'_>]) -> io::Result<usize> {
    if iov.len() == 1 {
        // SAFETY: the slice is valid for its length.
        let n = unsafe { minix_std::fs::write(fd, &iov[0]) }.map_err(errno)?;
        return usize::try_from(n).map_err(|_| io::Errno::INVAL);
    }
    let mut total = 0;
    for buf in iov {
        if buf.is_empty() {
            continue;
        }
        // SAFETY: the slice is valid for its length.
        let n = unsafe { minix_std::fs::write(fd, buf) }.map_err(errno)?;
        let n = usize::try_from(n).map_err(|_| io::Errno::INVAL)?;
        total += n;
        if n < buf.len() {
            break;
        }
    }
    Ok(total)
}

fn read_body(fd: i32, iov: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
    if iov.len() == 1 {
        let buf = &mut *iov[0];
        return match kind(fd) {
            // A single buffer on an inet socket is the net server's datagram
            // path; the plain read would leave the UDP header in the payload.
            // SAFETY: the slice is valid for its length.
            Kind::Inet => unsafe { minix_std::net::recv(fd, buf) }
                .map_err(errno)
                .and_then(|n| usize::try_from(n).map_err(|_| io::Errno::INVAL)),
            Kind::Local => minix_std::uds::recv(fd, buf)
                .map_err(errno)
                .and_then(|n| usize::try_from(n).map_err(|_| io::Errno::INVAL)),
        };
    }
    if kind(fd) == Kind::Inet {
        // Scattering one datagram over several buffers is not something the
        // net server can do: its write takes the whole datagram at once.
        return Err(io::Errno::OPNOTSUPP);
    }
    let mut total = 0;
    for buf in iov {
        if buf.is_empty() {
            continue;
        }
        let n = minix_std::uds::recv(fd, buf).map_err(errno)?;
        let n = usize::try_from(n).map_err(|_| io::Errno::INVAL)?;
        total += n;
        if n < buf.len() {
            break;
        }
    }
    Ok(total)
}

pub(crate) fn socketpair(
    domain: AddressFamily,
    type_: SocketType,
    flags: SocketFlags,
    protocol: Option<Protocol>,
) -> io::Result<(OwnedFd, OwnedFd)> {
    let _ = (type_, protocol);
    if domain.0 as c::c_int != c::AF_UNIX {
        return Err(io::Errno::AFNOSUPPORT);
    }
    let (a, b) = minix_std::uds::socketpair().map_err(errno)?;
    apply_flags(a, flags)?;
    apply_flags(b, flags)?;
    // SAFETY: `socketpair` hands back two descriptors the caller now owns.
    unsafe { Ok((owned_fd_from_raw(a), owned_fd_from_raw(b))) }
}
