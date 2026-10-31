//! The C-level names the shared layer above the backend uses.
//!
//! None of this is an FFI declaration: there is no C function behind any of it.
//! The types and constants are the ones `tools/c-include` publishes, so a value
//! that crosses this module means to a server exactly what it means to C.
//!
//! The set is larger than any one feature needs, because it mirrors the header
//! surface rather than the current call graph.
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub(crate) use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_ushort, c_void};

pub(crate) type c_schar = i8;
pub(crate) type c_short = i16;
pub(crate) type c_longlong = i64;
pub(crate) type size_t = usize;
pub(crate) type ssize_t = isize;
pub(crate) type off_t = i64;
pub(crate) type mode_t = u32;
pub(crate) type pid_t = i32;
pub(crate) type uid_t = u32;
pub(crate) type gid_t = u32;
pub(crate) type dev_t = u64;
pub(crate) type ino_t = u64;
pub(crate) type nlink_t = u32;
pub(crate) type blksize_t = i64;
pub(crate) type blkcnt_t = i64;
pub(crate) type time_t = i64;
pub(crate) type clockid_t = i32;
pub(crate) type __fsword_t = i64;
pub(crate) type fsid_t = u64;

/// The width of an `ioctl` request, which `rustix::ioctl::Opcode` is.
pub(crate) type OpcodeValue = c_ulong;

/// The VFS `stat` reply. rustix's `Stat` is an alias for this, so the field
/// names the shared code reads (`st_mode`, `st_size`, ...) are the port's own.
pub(crate) type stat = minix_std::fs::Stat;
pub(crate) type stat64 = minix_std::fs::Stat;

/// The VFS `statvfs` reply, laid out in `tools/c-include/sys/statvfs.h`.
pub(crate) type statfs = minix_std::fs::Statvfs;
pub(crate) type statfs64 = minix_std::fs::Statvfs;

/// The timespec layout `src/timespec.rs` names.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: c_long,
}

/// `struct itimerspec`, two `timespec`s in the order the VFS timerfd messages
/// carry them.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}

/// `PIPE_BUF`: the largest write POSIX guarantees is atomic, which is the
/// figure `tools/c-include/limits.h` publishes.
pub(crate) const PIPE_BUF: usize = 4096;

/// `struct utsname` from `tools/c-include/sys/utsname.h`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct utsname {
    pub sysname: [c_char; 65],
    pub nodename: [c_char; 65],
    pub release: [c_char; 65],
    pub version: [c_char; 65],
    pub machine: [c_char; 65],
    pub domainname: [c_char; 65],
}

/// The three standard descriptors, which the port keeps at these numbers.
pub(crate) const STDIN_FILENO: c_int = 0;
pub(crate) const STDOUT_FILENO: c_int = 1;
pub(crate) const STDERR_FILENO: c_int = 2;

/// `id_t`, which `procctl` names.
pub(crate) type id_t = u32;

/// `struct flock`, from `tools/c-include/fcntl.h`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct flock {
    pub l_type: c_short,
    pub l_whence: c_short,
    pub l_start: off_t,
    pub l_len: off_t,
    pub l_pid: pid_t,
}

/// Lock types for `struct flock`.
pub(crate) const F_RDLCK: c_short = 0;
pub(crate) const F_WRLCK: c_short = 1;
pub(crate) const F_UNLCK: c_short = 2;

/// `TIOCSCTTY`, the BSD `_IO('t', 37)` encoding.
pub(crate) const TIOCSCTTY: c_ulong = 0x7425;

/// `struct pollfd`, from `tools/c-include/poll.h`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}

// `poll` flags. The port publishes the first six; the rest are the standard
// values, which is what the VFS's epoll/poll layer compares against.
pub(crate) const POLLIN: c_short = 0x001;
pub(crate) const POLLPRI: c_short = 0x002;
pub(crate) const POLLOUT: c_short = 0x004;
pub(crate) const POLLERR: c_short = 0x008;
pub(crate) const POLLHUP: c_short = 0x010;
pub(crate) const POLLNVAL: c_short = 0x020;
pub(crate) const POLLRDNORM: c_short = 0x040;
pub(crate) const POLLRDBAND: c_short = 0x080;
pub(crate) const POLLWRNORM: c_short = 0x100;
pub(crate) const POLLWRBAND: c_short = 0x200;

// epoll flags, from `tools/c-include/sys/epoll.h`.
pub(crate) const EPOLL_CLOEXEC: c_int = O_CLOEXEC;
pub(crate) const EPOLL_CTL_ADD: c_int = 1;
pub(crate) const EPOLL_CTL_DEL: c_int = 2;
pub(crate) const EPOLL_CTL_MOD: c_int = 3;
pub(crate) const EPOLLIN: c_uint = 0x001;
pub(crate) const EPOLLPRI: c_uint = 0x002;
pub(crate) const EPOLLOUT: c_uint = 0x004;
pub(crate) const EPOLLERR: c_uint = 0x008;
pub(crate) const EPOLLHUP: c_uint = 0x010;
pub(crate) const EPOLLNVAL: c_uint = 0x020;
pub(crate) const EPOLLRDNORM: c_uint = 0x040;
pub(crate) const EPOLLRDBAND: c_uint = 0x080;
pub(crate) const EPOLLWRNORM: c_uint = 0x100;
pub(crate) const EPOLLWRBAND: c_uint = 0x200;
pub(crate) const EPOLLMSG: c_uint = 0x400;
pub(crate) const EPOLLRDHUP: c_uint = 0x2000;
pub(crate) const EPOLLEXCLUSIVE: c_uint = 1 << 28;
pub(crate) const EPOLLWAKEUP: c_uint = 1 << 29;
pub(crate) const EPOLLONESHOT: c_uint = 1 << 30;
pub(crate) const EPOLLET: c_uint = 1 << 31;

// `eventfd` flags.
pub(crate) const EFD_SEMAPHORE: c_int = 1;
pub(crate) const EFD_CLOEXEC: c_int = O_CLOEXEC;
pub(crate) const EFD_NONBLOCK: c_int = O_NONBLOCK;

// Clock ids. PM serves `CLOCK_REALTIME` and `CLOCK_MONOTONIC`; the rest keep
// the standard numbering so the `ClockId` enum has distinct values, and PM
// answers `EINVAL` for them.
pub(crate) const CLOCK_REALTIME: clockid_t = 0;
pub(crate) const CLOCK_MONOTONIC: clockid_t = 1;
pub(crate) const CLOCK_PROCESS_CPUTIME_ID: clockid_t = 2;
pub(crate) const CLOCK_THREAD_CPUTIME_ID: clockid_t = 3;
pub(crate) const CLOCK_MONOTONIC_RAW: clockid_t = 4;
pub(crate) const CLOCK_REALTIME_COARSE: clockid_t = 5;
pub(crate) const CLOCK_MONOTONIC_COARSE: clockid_t = 6;
pub(crate) const CLOCK_BOOTTIME: clockid_t = 7;
pub(crate) const CLOCK_REALTIME_ALARM: clockid_t = 8;
pub(crate) const CLOCK_BOOTTIME_ALARM: clockid_t = 9;

// `timerfd_create`/`timerfd_settime` flags, from `tools/c-include/sys/timerfd.h`.
pub(crate) const TFD_CLOEXEC: c_int = O_CLOEXEC;
pub(crate) const TFD_NONBLOCK: c_int = O_NONBLOCK;
pub(crate) const TFD_TIMER_ABSTIME: c_int = 1;
pub(crate) const TFD_TIMER_CANCEL_ON_SET: c_int = 2;

// ---- sys/socket.h ----

pub(crate) type socklen_t = u32;
pub(crate) type sa_family_t = u16;

pub(crate) const AF_UNSPEC: c_int = 0;
pub(crate) const AF_UNIX: c_int = 1;
pub(crate) const AF_LOCAL: c_int = AF_UNIX;
pub(crate) const AF_INET: c_int = 2;
pub(crate) const AF_INET6: c_int = 10;

pub(crate) const SOCK_STREAM: c_int = 1;
pub(crate) const SOCK_DGRAM: c_int = 2;
pub(crate) const SOCK_RAW: c_int = 3;
pub(crate) const SOCK_RDM: c_int = 4;
pub(crate) const SOCK_SEQPACKET: c_int = 5;
/// `SOCK_CLOEXEC`/`SOCK_NONBLOCK` are ORed into the type, as on Linux.
pub(crate) const SOCK_CLOEXEC: c_int = 0x80000;
pub(crate) const SOCK_NONBLOCK: c_int = 0x800;

pub(crate) const SHUT_RD: c_int = 0;
pub(crate) const SHUT_WR: c_int = 1;
pub(crate) const SHUT_RDWR: c_int = 2;

pub(crate) const IPPROTO_IP: c_int = 0;
pub(crate) const IPPROTO_ICMP: c_int = 1;
pub(crate) const IPPROTO_TCP: c_int = 6;
pub(crate) const IPPROTO_UDP: c_int = 17;
pub(crate) const IPPROTO_RAW: c_int = 255;

/// MINIX's `SOL_SOCKET` is `0xffff` (`sys/sys/socket.h`), not Linux's 1.
pub(crate) const SOL_SOCKET: c_int = 0xffff;
pub(crate) const SO_REUSEADDR: c_int = 2;
pub(crate) const SO_TYPE: c_int = 3;
pub(crate) const SO_ERROR: c_int = 4;
pub(crate) const SO_BROADCAST: c_int = 6;
pub(crate) const SO_SNDBUF: c_int = 7;
pub(crate) const SO_RCVBUF: c_int = 8;
pub(crate) const SO_KEEPALIVE: c_int = 9;
pub(crate) const SO_OOBINLINE: c_int = 10;
pub(crate) const SO_LINGER: c_int = 13;
pub(crate) const SO_REUSEPORT: c_int = 15;
pub(crate) const SO_PEERCRED: c_int = 17;
pub(crate) const SO_RCVTIMEO: c_int = 20;
pub(crate) const SO_SNDTIMEO: c_int = 21;
pub(crate) const SO_ACCEPTCONN: c_int = 30;
pub(crate) const SO_DOMAIN: c_int = 39;

pub(crate) const SCM_RIGHTS: c_int = 0x01;
pub(crate) const SCM_CREDS: c_int = 0x04;

pub(crate) const MSG_OOB: c_int = 0x0001;
pub(crate) const MSG_PEEK: c_int = 0x0002;
pub(crate) const MSG_DONTROUTE: c_int = 0x0004;
pub(crate) const MSG_EOR: c_int = 0x0008;
pub(crate) const MSG_TRUNC: c_int = 0x0010;
pub(crate) const MSG_CTRUNC: c_int = 0x0020;
pub(crate) const MSG_WAITALL: c_int = 0x0040;
pub(crate) const MSG_DONTWAIT: c_int = 0x0080;
pub(crate) const MSG_CONFIRM: c_int = 0x0800;
pub(crate) const MSG_ERRQUEUE: c_int = 0x2000;
pub(crate) const MSG_NOSIGNAL: c_int = 0x4000;
pub(crate) const MSG_MORE: c_int = 0x8000;
pub(crate) const MSG_CMSG_CLOEXEC: c_int = 0x4000_0000;

// Address families the port does not open. The shared layer names every one
// its `AddressFamily` constants expose, so each needs a value; `socket`
// refuses all but `AF_UNIX` and `AF_INET` with `EPROTONOSUPPORT`.
pub(crate) const AF_AX25: c_int = 3;
pub(crate) const AF_IPX: c_int = 4;
pub(crate) const AF_APPLETALK: c_int = 5;
pub(crate) const AF_NETROM: c_int = 6;
pub(crate) const AF_BRIDGE: c_int = 7;
pub(crate) const AF_ATMPVC: c_int = 8;
pub(crate) const AF_X25: c_int = 9;
pub(crate) const AF_ROSE: c_int = 11;
pub(crate) const AF_DECnet: c_int = 12;
pub(crate) const AF_NETBEUI: c_int = 13;
pub(crate) const AF_SECURITY: c_int = 14;
pub(crate) const AF_KEY: c_int = 15;
pub(crate) const AF_NETLINK: c_int = 16;
pub(crate) const AF_PACKET: c_int = 17;
pub(crate) const AF_ASH: c_int = 18;
pub(crate) const AF_ECONET: c_int = 19;
pub(crate) const AF_ATMSVC: c_int = 20;
pub(crate) const AF_RDS: c_int = 21;
pub(crate) const AF_SNA: c_int = 22;
pub(crate) const AF_IRDA: c_int = 23;
pub(crate) const AF_PPPOX: c_int = 24;
pub(crate) const AF_WANPIPE: c_int = 25;
pub(crate) const AF_LLC: c_int = 26;
pub(crate) const AF_CAN: c_int = 29;
pub(crate) const AF_TIPC: c_int = 30;
pub(crate) const AF_BLUETOOTH: c_int = 31;
pub(crate) const AF_IUCV: c_int = 32;
pub(crate) const AF_RXRPC: c_int = 33;
pub(crate) const AF_ISDN: c_int = 34;
pub(crate) const AF_PHONET: c_int = 35;
pub(crate) const AF_IEEE802154: c_int = 36;

// The same for the `IPPROTO_*` constants: the port serves TCP, UDP and ICMP
// only, and `socket` refuses the rest.
pub(crate) const IPPROTO_IGMP: c_int = 2;
pub(crate) const IPPROTO_IPIP: c_int = 4;
pub(crate) const IPPROTO_EGP: c_int = 8;
pub(crate) const IPPROTO_PUP: c_int = 12;
pub(crate) const IPPROTO_IDP: c_int = 22;
pub(crate) const IPPROTO_TP: c_int = 29;
pub(crate) const IPPROTO_DCCP: c_int = 33;
pub(crate) const IPPROTO_IPV6: c_int = 41;
pub(crate) const IPPROTO_ROUTING: c_int = 43;
pub(crate) const IPPROTO_FRAGMENT: c_int = 44;
pub(crate) const IPPROTO_RSVP: c_int = 46;
pub(crate) const IPPROTO_GRE: c_int = 47;
pub(crate) const IPPROTO_ESP: c_int = 50;
pub(crate) const IPPROTO_AH: c_int = 51;
pub(crate) const IPPROTO_ICMPV6: c_int = 58;
pub(crate) const IPPROTO_MTP: c_int = 92;
pub(crate) const IPPROTO_BEETPH: c_int = 94;
pub(crate) const IPPROTO_ENCAP: c_int = 98;
pub(crate) const IPPROTO_PIM: c_int = 103;
pub(crate) const IPPROTO_COMP: c_int = 108;
pub(crate) const IPPROTO_SCTP: c_int = 132;
pub(crate) const IPPROTO_MH: c_int = 135;
pub(crate) const IPPROTO_UDPLITE: c_int = 136;
pub(crate) const IPPROTO_MPLS: c_int = 137;
pub(crate) const IPPROTO_MPTCP: c_int = 262;

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct in_addr {
    /// The four octets in network order.
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct sockaddr_in {
    pub sin_family: sa_family_t,
    /// Network order.
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    /// Network order.
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [c_char; 108],
}

/// `struct sockaddr_storage`: large and aligned enough for any address above.
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub(crate) struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __data: [u8; 126],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: size_t,
}

/// `struct cmsghdr`: a 32-bit `socklen_t` and an alignment of 4, so the
/// header is 12 bytes and the data starts at 12 (`tools/c-include/sys/socket.h`).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct cmsghdr {
    pub cmsg_len: socklen_t,
    pub cmsg_level: c_int,
    pub cmsg_type: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct msghdr {
    pub msg_name: *mut c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: c_int,
    pub msg_control: *mut c_void,
    pub msg_controllen: socklen_t,
    pub msg_flags: c_int,
}

pub(crate) const fn CMSG_ALIGN(len: usize) -> usize {
    (len + 3) & !3
}

pub(crate) const unsafe fn CMSG_DATA(cmsg: *const cmsghdr) -> *mut u8 {
    (cmsg as *const u8).wrapping_add(CMSG_ALIGN(core::mem::size_of::<cmsghdr>())) as *mut u8
}

pub(crate) const unsafe fn CMSG_SPACE(len: u32) -> u32 {
    (CMSG_ALIGN(core::mem::size_of::<cmsghdr>()) + CMSG_ALIGN(len as usize)) as u32
}

pub(crate) const unsafe fn CMSG_LEN(len: u32) -> u32 {
    (CMSG_ALIGN(core::mem::size_of::<cmsghdr>()) + len as usize) as u32
}

pub(crate) unsafe fn CMSG_FIRSTHDR(mhdr: *const msghdr) -> *mut cmsghdr {
    unsafe {
        if ((*mhdr).msg_controllen as usize) < core::mem::size_of::<cmsghdr>() {
            core::ptr::null_mut()
        } else {
            (*mhdr).msg_control as *mut cmsghdr
        }
    }
}

pub(crate) unsafe fn CMSG_NXTHDR(mhdr: *const msghdr, cmsg: *const cmsghdr) -> *mut cmsghdr {
    unsafe {
        let next = (cmsg as *const u8).wrapping_add(CMSG_ALIGN((*cmsg).cmsg_len as usize));
        let end = ((*mhdr).msg_control as *const u8).wrapping_add((*mhdr).msg_controllen as usize);
        if next.wrapping_add(CMSG_ALIGN(core::mem::size_of::<cmsghdr>())) > end {
            core::ptr::null_mut()
        } else {
            next as *mut cmsghdr
        }
    }
}

// Exit statuses, from tools/c-include/stdlib.h.
pub(crate) const EXIT_SUCCESS: c_int = 0;
pub(crate) const EXIT_FAILURE: c_int = 1;
/// `128 + SIGABRT`, which is how a shell reports an abort.
pub(crate) const EXIT_SIGNALED_SIGABRT: c_int = 128 + 6;

// Signals. The port's own set is the one `tools/c-include/signal.h` publishes;
// the rest are numbered above it because PM never delivers them, and they
// exist so the `Signal` constants have distinct values.
pub(crate) const SIGHUP: c_int = 1;
pub(crate) const SIGINT: c_int = 2;
pub(crate) const SIGQUIT: c_int = 3;
pub(crate) const SIGILL: c_int = 4;
pub(crate) const SIGTRAP: c_int = 5;
pub(crate) const SIGABRT: c_int = 6;
pub(crate) const SIGBUS: c_int = 7;
pub(crate) const SIGFPE: c_int = 8;
pub(crate) const SIGKILL: c_int = 9;
pub(crate) const SIGUSR1: c_int = 10;
pub(crate) const SIGSEGV: c_int = 11;
pub(crate) const SIGUSR2: c_int = 12;
pub(crate) const SIGPIPE: c_int = 13;
pub(crate) const SIGALRM: c_int = 14;
pub(crate) const SIGTERM: c_int = 15;
pub(crate) const SIGCHLD: c_int = 20;
pub(crate) const SIGWINCH: c_int = 28;
pub(crate) const SIGSYS: c_int = 31;
pub(crate) const SIGSTKFLT: c_int = 16;
pub(crate) const SIGCONT: c_int = 18;
pub(crate) const SIGSTOP: c_int = 19;
pub(crate) const SIGTTIN: c_int = 21;
pub(crate) const SIGTTOU: c_int = 22;
pub(crate) const SIGURG: c_int = 23;
pub(crate) const SIGXCPU: c_int = 24;
pub(crate) const SIGXFSZ: c_int = 25;
pub(crate) const SIGVTALRM: c_int = 26;
pub(crate) const SIGPROF: c_int = 27;
pub(crate) const SIGIO: c_int = 29;
pub(crate) const SIGPOLL: c_int = SIGIO;
pub(crate) const SIGPWR: c_int = 30;
pub(crate) const SIGTSTP: c_int = 32;

// Resource numbers, from tools/c-include/sys/resource.h.
pub(crate) const RLIMIT_CPU: c_int = 0;
pub(crate) const RLIMIT_FSIZE: c_int = 1;
pub(crate) const RLIMIT_DATA: c_int = 2;
pub(crate) const RLIMIT_STACK: c_int = 3;
pub(crate) const RLIMIT_CORE: c_int = 4;
pub(crate) const RLIMIT_RSS: c_int = 5;
pub(crate) const RLIMIT_NPROC: c_int = 6;
pub(crate) const RLIMIT_NOFILE: c_int = 7;
pub(crate) const RLIMIT_MEMLOCK: c_int = 8;
pub(crate) const RLIMIT_AS: c_int = 9;
pub(crate) const RLIMIT_LOCKS: c_int = 10;
pub(crate) const RLIMIT_SIGPENDING: c_int = 11;
pub(crate) const RLIMIT_MSGQUEUE: c_int = 12;
pub(crate) const RLIMIT_NICE: c_int = 13;
pub(crate) const RLIMIT_RTPRIO: c_int = 14;
pub(crate) const RLIMIT_RTTIME: c_int = 15;

// `getpriority`/`setpriority` targets.
pub(crate) const PRIO_PROCESS: c_int = 0;
pub(crate) const PRIO_PGRP: c_int = 1;
pub(crate) const PRIO_USER: c_int = 2;

// `wait` options and the `siginfo_t` codes, from tools/c-include/sys/wait.h.
pub(crate) const WNOHANG: c_int = 1;
pub(crate) const WUNTRACED: c_int = 2;
pub(crate) const WCONTINUED: c_int = 8;
pub(crate) const WEXITED: c_int = 0x0000_0004;
pub(crate) const WSTOPPED: c_int = 0x0000_0002;
pub(crate) const WNOWAIT: c_int = 0x0100_0000;
pub(crate) const CLD_EXITED: c_int = 1;
pub(crate) const CLD_KILLED: c_int = 2;
pub(crate) const CLD_DUMPED: c_int = 3;
pub(crate) const CLD_TRAPPED: c_int = 4;
pub(crate) const CLD_STOPPED: c_int = 5;
pub(crate) const CLD_CONTINUED: c_int = 6;

pub(crate) const fn WIFEXITED(status: c_int) -> bool {
    status & 0x7f == 0
}

pub(crate) const fn WIFSIGNALED(status: c_int) -> bool {
    status & 0x7f != 0 && status & 0x7f != 0x7f
}

pub(crate) const fn WIFSTOPPED(status: c_int) -> bool {
    status & 0xff == 0x7f
}

/// PM has no stop or continue encoding, so a child never reports one.
pub(crate) const fn WIFCONTINUED(_status: c_int) -> bool {
    false
}

pub(crate) const fn WEXITSTATUS(status: c_int) -> c_int {
    (status >> 8) & 0xff
}

pub(crate) const fn WTERMSIG(status: c_int) -> c_int {
    status & 0x7f
}

pub(crate) const fn WSTOPSIG(status: c_int) -> c_int {
    WEXITSTATUS(status)
}

/// The `waitid` reply. The port never builds one, but `WaitIdStatus` wraps it
/// and reads these fields.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct siginfo_t {
    pub si_signo: c_int,
    pub si_errno: c_int,
    pub si_code: c_int,
    pub si_pid: pid_t,
    pub si_uid: uid_t,
    pub si_status: c_int,
}

impl siginfo_t {
    /// POSIX guarantees `si_status` is valid for a status `waitid` reports.
    ///
    /// # Safety
    ///
    /// Only call this on a `siginfo_t` a `waitid` call filled.
    pub(crate) unsafe fn si_status(&self) -> crate::ffi::c_int {
        self.si_status
    }
}

// `open` flags (tools/c-include/fcntl.h).
pub(crate) const O_RDONLY: c_int = 0o00;
pub(crate) const O_WRONLY: c_int = 0o01;
pub(crate) const O_RDWR: c_int = 0o02;
pub(crate) const O_ACCMODE: c_int = 0o03;
pub(crate) const O_CREAT: c_int = 0o100;
pub(crate) const O_EXCL: c_int = 0o200;
pub(crate) const O_NOCTTY: c_int = 0o400;
pub(crate) const O_TRUNC: c_int = 0o1000;
pub(crate) const O_APPEND: c_int = 0o2000;
pub(crate) const O_NONBLOCK: c_int = 0o4000;
pub(crate) const O_ASYNC: c_int = 0o20000;
pub(crate) const O_DIRECT: c_int = 0o40000;
pub(crate) const O_CLOEXEC: c_int = 0o200000;
pub(crate) const O_DIRECTORY: c_int = 0o400000;
pub(crate) const O_NOFOLLOW: c_int = 0o1000000;
pub(crate) const O_SYNC: c_int = 0o2000000;
pub(crate) const O_DSYNC: c_int = 0o4000000;

// fcntl commands and descriptor flags.
pub(crate) const F_DUPFD: c_int = 0;
pub(crate) const F_GETFD: c_int = 1;
pub(crate) const F_SETFD: c_int = 2;
pub(crate) const F_GETFL: c_int = 3;
pub(crate) const F_SETFL: c_int = 4;
pub(crate) const F_GETLK: c_int = 5;
pub(crate) const F_SETLK: c_int = 6;
pub(crate) const F_SETLKW: c_int = 7;
pub(crate) const F_DUPFD_CLOEXEC: c_int = 12;
pub(crate) const FD_CLOEXEC: c_int = 1;

// `flock` operations.
pub(crate) const LOCK_SH: c_int = 1;
pub(crate) const LOCK_EX: c_int = 2;
pub(crate) const LOCK_NB: c_int = 4;
pub(crate) const LOCK_UN: c_int = 8;

// `lseek` origins.
pub(crate) const SEEK_SET: c_int = 0;
pub(crate) const SEEK_CUR: c_int = 1;
pub(crate) const SEEK_END: c_int = 2;

// `*at` sentinels and flags (tools/c-include/fcntl.h).
pub(crate) const AT_FDCWD: c_int = -100;
pub(crate) const AT_EACCESS: c_int = 0x100;
pub(crate) const AT_SYMLINK_NOFOLLOW: c_int = 0x200;
pub(crate) const AT_SYMLINK_FOLLOW: c_int = 0x400;
pub(crate) const AT_REMOVEDIR: c_int = 0x800;
pub(crate) const AT_EMPTY_PATH: c_int = 0x1000;
pub(crate) const AT_NO_AUTOMOUNT: c_int = 0x800;
pub(crate) const AT_RESOLVE_BENEATH: c_int = 0x2000;
pub(crate) const AT_STATX_SYNC_AS_STAT: c_int = 0x0000;
pub(crate) const AT_STATX_FORCE_SYNC: c_int = 0x2000;
pub(crate) const AT_STATX_DONT_SYNC: c_int = 0x4000;

// `access` modes.
pub(crate) const F_OK: c_int = 0;
pub(crate) const X_OK: c_int = 1;
pub(crate) const W_OK: c_int = 2;
pub(crate) const R_OK: c_int = 4;

// File type and permission bits (tools/c-include/sys/stat.h).
pub(crate) const S_IFMT: mode_t = 0o170000;
pub(crate) const S_IFSOCK: mode_t = 0o140000;
pub(crate) const S_IFLNK: mode_t = 0o120000;
pub(crate) const S_IFREG: mode_t = 0o100000;
pub(crate) const S_IFBLK: mode_t = 0o060000;
pub(crate) const S_IFDIR: mode_t = 0o040000;
pub(crate) const S_IFCHR: mode_t = 0o020000;
pub(crate) const S_IFIFO: mode_t = 0o010000;
pub(crate) const S_ISUID: mode_t = 0o4000;
pub(crate) const S_ISGID: mode_t = 0o2000;
pub(crate) const S_ISVTX: mode_t = 0o1000;
pub(crate) const S_IRWXU: mode_t = 0o700;
pub(crate) const S_IRUSR: mode_t = 0o400;
pub(crate) const S_IWUSR: mode_t = 0o200;
pub(crate) const S_IXUSR: mode_t = 0o100;
pub(crate) const S_IRWXG: mode_t = 0o070;
pub(crate) const S_IRGRP: mode_t = 0o040;
pub(crate) const S_IWGRP: mode_t = 0o020;
pub(crate) const S_IXGRP: mode_t = 0o010;
pub(crate) const S_IRWXO: mode_t = 0o007;
pub(crate) const S_IROTH: mode_t = 0o004;
pub(crate) const S_IWOTH: mode_t = 0o002;
pub(crate) const S_IXOTH: mode_t = 0o001;

// `d_type` values. The VFS writes the BSD numbering, which is what `rustix`
// reports from `DirEntry::file_type`.
pub(crate) const DT_UNKNOWN: u8 = 0;
pub(crate) const DT_FIFO: u8 = 1;
pub(crate) const DT_CHR: u8 = 2;
pub(crate) const DT_DIR: u8 = 4;
pub(crate) const DT_BLK: u8 = 6;
pub(crate) const DT_REG: u8 = 8;
pub(crate) const DT_LNK: u8 = 10;
pub(crate) const DT_SOCK: u8 = 12;

// `statvfs` flags.
pub(crate) const ST_RDONLY: c_ulong = 0x0001;
pub(crate) const ST_NOSUID: c_ulong = 0x0002;
pub(crate) const ST_NODEV: c_ulong = 0x0004;
pub(crate) const ST_NOEXEC: c_ulong = 0x0008;
pub(crate) const ST_SYNCHRONOUS: c_ulong = 0x0010;
pub(crate) const ST_MANDLOCK: c_ulong = 0x0040;
pub(crate) const ST_NOATIME: c_ulong = 0x0400;
pub(crate) const ST_NODIRATIME: c_ulong = 0x0800;
pub(crate) const ST_RELATIME: c_ulong = 0x1000;

// Filesystem magic numbers. The port mounts one filesystem type, so these
// exist for the callers that compare against them.
pub(crate) const PROC_SUPER_MAGIC: __fsword_t = 0x9fa0;
pub(crate) const NFS_SUPER_MAGIC: __fsword_t = 0x6969;

// `posix_fadvise` advice values.
pub(crate) const POSIX_FADV_NORMAL: c_int = 0;
pub(crate) const POSIX_FADV_RANDOM: c_int = 1;
pub(crate) const POSIX_FADV_SEQUENTIAL: c_int = 2;
pub(crate) const POSIX_FADV_WILLNEED: c_int = 3;
pub(crate) const POSIX_FADV_DONTNEED: c_int = 4;
pub(crate) const POSIX_FADV_NOREUSE: c_int = 5;

// `fallocate` mode bits.
pub(crate) const FALLOC_FL_KEEP_SIZE: c_int = 0x01;
pub(crate) const FALLOC_FL_PUNCH_HOLE: c_int = 0x02;
pub(crate) const FALLOC_FL_NO_HIDE_STALE: c_int = 0x04;
pub(crate) const FALLOC_FL_COLLAPSE_RANGE: c_int = 0x08;
pub(crate) const FALLOC_FL_ZERO_RANGE: c_int = 0x10;
pub(crate) const FALLOC_FL_INSERT_RANGE: c_int = 0x20;
pub(crate) const FALLOC_FL_UNSHARE_RANGE: c_int = 0x40;

// `renameat2` flags.
pub(crate) const RENAME_NOREPLACE: c_uint = 1;
pub(crate) const RENAME_EXCHANGE: c_uint = 2;
pub(crate) const RENAME_WHITEOUT: c_uint = 4;
pub(crate) const RENAME_EXCL: c_uint = 8;
pub(crate) const RENAME_SWAP: c_uint = 0x10;

// `memfd_create` flags.
pub(crate) const MFD_CLOEXEC: c_uint = 0x0001;
pub(crate) const MFD_ALLOW_SEALING: c_uint = 0x0002;
pub(crate) const MFD_HUGETLB: c_uint = 0x0004;
pub(crate) const MFD_NOEXEC_SEAL: c_uint = 0x0008;
pub(crate) const MFD_EXEC: c_uint = 0x0010;
pub(crate) const MFD_HUGE_64KB: c_uint = 16 << 26;
pub(crate) const MFD_HUGE_512KB: c_uint = 19 << 26;
pub(crate) const MFD_HUGE_1MB: c_uint = 20 << 26;
pub(crate) const MFD_HUGE_2MB: c_uint = 21 << 26;
pub(crate) const MFD_HUGE_8MB: c_uint = 23 << 26;
pub(crate) const MFD_HUGE_16MB: c_uint = 24 << 26;
pub(crate) const MFD_HUGE_32MB: c_uint = 25 << 26;
pub(crate) const MFD_HUGE_256MB: c_uint = 28 << 26;
pub(crate) const MFD_HUGE_512MB: c_uint = 29 << 26;
pub(crate) const MFD_HUGE_1GB: c_uint = 30 << 26;
pub(crate) const MFD_HUGE_2GB: c_uint = 31 << 26;
pub(crate) const MFD_HUGE_16GB: c_uint = (34 << 26) as c_uint;

// `memfd` seals.
pub(crate) const F_SEAL_SEAL: c_int = 0x0001;
pub(crate) const F_SEAL_SHRINK: c_int = 0x0002;
pub(crate) const F_SEAL_GROW: c_int = 0x0004;
pub(crate) const F_SEAL_WRITE: c_int = 0x0008;
pub(crate) const F_SEAL_FUTURE_WRITE: c_int = 0x0010;
pub(crate) const F_SEAL_EXEC: c_int = 0x0020;

// `utimensat` `tv_nsec` sentinels.
pub(crate) const UTIME_NOW: c_long = 0x3fff_ffff;
pub(crate) const UTIME_OMIT: c_long = 0x3fff_fffe;

// The two `ioctl` requests `rustix::io` issues. They are the BSD `_IOW`/`_IOR`
// encodings `tools/c-include/sys/ioctl.h` builds.
pub(crate) const FIONBIO: OpcodeValue = 0x8004_667e;
pub(crate) const FIONREAD: OpcodeValue = 0x4004_667f;

// Error numbers, from tools/c-include/errno.h. The port follows the MINIX
// numbering for the errors its servers return and the Linux numbering for the
// rest, which is the convention that header records.
pub(crate) const EPERM: c_int = 1;
pub(crate) const ENOENT: c_int = 2;
pub(crate) const ESRCH: c_int = 3;
pub(crate) const EINTR: c_int = 4;
pub(crate) const EIO: c_int = 5;
pub(crate) const ENXIO: c_int = 6;
pub(crate) const EBADF: c_int = 9;
pub(crate) const EAGAIN: c_int = 11;
pub(crate) const ENOMEM: c_int = 12;
pub(crate) const EACCES: c_int = 13;
pub(crate) const EFAULT: c_int = 14;
pub(crate) const EBUSY: c_int = 16;
pub(crate) const EEXIST: c_int = 17;
pub(crate) const ENODEV: c_int = 19;
pub(crate) const ENOTDIR: c_int = 20;
pub(crate) const EISDIR: c_int = 21;
pub(crate) const EINVAL: c_int = 22;
pub(crate) const ENOSPC: c_int = 28;
pub(crate) const EDOM: c_int = 33;
pub(crate) const ERANGE: c_int = 34;
pub(crate) const ENOSYS: c_int = 78;
pub(crate) const EOPNOTSUPP: c_int = 95;
pub(crate) const EAFNOSUPPORT: c_int = 97;

pub(crate) const E2BIG: c_int = 7;
pub(crate) const ENOEXEC: c_int = 8;
pub(crate) const ECHILD: c_int = 10;
pub(crate) const ENOTBLK: c_int = 15;
pub(crate) const EXDEV: c_int = 18;
pub(crate) const ENFILE: c_int = 23;
pub(crate) const EMFILE: c_int = 24;
pub(crate) const ENOTTY: c_int = 25;
pub(crate) const ETXTBSY: c_int = 26;
pub(crate) const EFBIG: c_int = 27;
pub(crate) const ESPIPE: c_int = 29;
pub(crate) const EROFS: c_int = 30;
pub(crate) const EMLINK: c_int = 31;
pub(crate) const EPIPE: c_int = 32;
pub(crate) const EDEADLK: c_int = 35;
pub(crate) const ENAMETOOLONG: c_int = 36;
pub(crate) const ELOOP: c_int = 40;
pub(crate) const EOVERFLOW: c_int = 75;
pub(crate) const EILSEQ: c_int = 84;
pub(crate) const EWOULDBLOCK: c_int = EAGAIN;

pub(crate) const ENOTSOCK: c_int = 88;
pub(crate) const EDESTADDRREQ: c_int = 89;
pub(crate) const EMSGSIZE: c_int = 90;
pub(crate) const ENOPROTOOPT: c_int = 92;
pub(crate) const EPROTONOSUPPORT: c_int = 93;
pub(crate) const EADDRINUSE: c_int = 98;
pub(crate) const EADDRNOTAVAIL: c_int = 99;
pub(crate) const ENETDOWN: c_int = 100;
pub(crate) const ENETUNREACH: c_int = 101;
pub(crate) const ENETRESET: c_int = 102;
pub(crate) const ECONNABORTED: c_int = 103;
pub(crate) const ECONNRESET: c_int = 104;
pub(crate) const ENOBUFS: c_int = 105;
pub(crate) const EISCONN: c_int = 106;
pub(crate) const ENOTCONN: c_int = 107;
pub(crate) const ESHUTDOWN: c_int = 108;
pub(crate) const ETOOMANYREFS: c_int = 109;
pub(crate) const ETIMEDOUT: c_int = 110;
pub(crate) const ECONNREFUSED: c_int = 111;
pub(crate) const EHOSTDOWN: c_int = 112;
pub(crate) const EHOSTUNREACH: c_int = 113;
pub(crate) const EALREADY: c_int = 114;
pub(crate) const EINPROGRESS: c_int = 115;

// The remainder of the Linux-numbered set the `Errno` constants name. The
// port never returns these, so their numbers are the Linux ones, which keeps
// them aligned with the values in `tools/c-include/errno.h`. `EREMCHG` takes a
// number above the range in use because Linux spells it 78, and 78 is the
// port's `ENOSYS`.
pub(crate) const EADV: c_int = 68;
pub(crate) const EBADE: c_int = 52;
pub(crate) const EBADFD: c_int = 77;
pub(crate) const EBADMSG: c_int = 74;
pub(crate) const EBADR: c_int = 53;
pub(crate) const EBADRQC: c_int = 56;
pub(crate) const EBADSLT: c_int = 57;
pub(crate) const EBFONT: c_int = 59;
pub(crate) const ECANCELED: c_int = 125;
pub(crate) const ECHRNG: c_int = 44;
pub(crate) const ECOMM: c_int = 70;
pub(crate) const EDEADLOCK: c_int = EDEADLK;
pub(crate) const EDOTDOT: c_int = 73;
pub(crate) const EDQUOT: c_int = 122;
pub(crate) const EHWPOISON: c_int = 133;
pub(crate) const EIDRM: c_int = 43;
pub(crate) const EISNAM: c_int = 120;
pub(crate) const EKEYEXPIRED: c_int = 127;
pub(crate) const EKEYREJECTED: c_int = 129;
pub(crate) const EKEYREVOKED: c_int = 128;
pub(crate) const EL2HLT: c_int = 51;
pub(crate) const EL2NSYNC: c_int = 45;
pub(crate) const EL3HLT: c_int = 46;
pub(crate) const EL3RST: c_int = 47;
pub(crate) const ELIBACC: c_int = 79;
pub(crate) const ELIBBAD: c_int = 80;
pub(crate) const ELIBEXEC: c_int = 83;
pub(crate) const ELIBMAX: c_int = 82;
pub(crate) const ELIBSCN: c_int = 81;
pub(crate) const ELNRNG: c_int = 48;
pub(crate) const EMEDIUMTYPE: c_int = 124;
pub(crate) const EMULTIHOP: c_int = 72;
pub(crate) const ENAVAIL: c_int = 119;
pub(crate) const ENOANO: c_int = 55;
pub(crate) const ENOCSI: c_int = 50;
pub(crate) const ENODATA: c_int = 61;
pub(crate) const ENOKEY: c_int = 126;
pub(crate) const ENOLCK: c_int = 37;
pub(crate) const ENOLINK: c_int = 67;
pub(crate) const ENOMEDIUM: c_int = 123;
pub(crate) const ENOMSG: c_int = 42;
pub(crate) const ENONET: c_int = 64;
pub(crate) const ENOPKG: c_int = 65;
pub(crate) const ENOSR: c_int = 63;
pub(crate) const ENOSTR: c_int = 60;
pub(crate) const ENOTEMPTY: c_int = 39;
pub(crate) const ENOTNAM: c_int = 118;
pub(crate) const ENOTRECOVERABLE: c_int = 131;
pub(crate) const ENOTSUP: c_int = EOPNOTSUPP;
pub(crate) const ENOTUNIQ: c_int = 76;
pub(crate) const EOWNERDEAD: c_int = 130;
pub(crate) const EPFNOSUPPORT: c_int = 96;
pub(crate) const EPROTO: c_int = 71;
pub(crate) const EPROTOTYPE: c_int = 91;
pub(crate) const EREMCHG: c_int = 200;
pub(crate) const EREMOTE: c_int = 66;
pub(crate) const EREMOTEIO: c_int = 121;
pub(crate) const ERESTART: c_int = 85;
pub(crate) const ERFKILL: c_int = 132;
pub(crate) const ESOCKTNOSUPPORT: c_int = 94;
pub(crate) const ESRMNT: c_int = 69;
pub(crate) const ESTALE: c_int = 116;
pub(crate) const ESTRPIPE: c_int = 86;
pub(crate) const ETIME: c_int = 62;
pub(crate) const EUCLEAN: c_int = 117;
pub(crate) const EUNATCH: c_int = 49;
pub(crate) const EUSERS: c_int = 87;
pub(crate) const EXFULL: c_int = 54;
