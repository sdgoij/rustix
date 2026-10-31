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
