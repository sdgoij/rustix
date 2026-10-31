//! `Dir` and `DirEntry`, reading the VFS `getdents` reply.
//!
//! The VFS writes one record per entry: `d_fileno` (u64) at 0, `d_reclen`
//! (u16) at 8, `d_namlen` (u16) at 10, `d_type` at 12, and the name bytes at
//! 13 with no terminator. There is no `DIR` object to hand to a C library, so
//! the stream is the descriptor plus the buffer it was last filled with.

use crate::backend::c;
use crate::backend::conv::{errno, owned_fd_from_raw};
use crate::fd::{AsFd, AsRawFd as _, BorrowedFd, OwnedFd};
use crate::ffi::{CStr, CString};
use crate::fs::{FileType, Stat, StatFs, StatVfs};
use crate::io;
use core::fmt;

/// One `getdents` reply.
const BUF_LEN: usize = 4096;

/// The offset of the name inside a record.
const NAME_OFF: usize = 13;

/// `d_name` is 256 bytes in the C struct, so a name that fills it has no room
/// for a terminator.
const MAX_NAME: usize = 255;

/// A directory stream.
pub struct Dir {
    fd: OwnedFd,
    buf: [u8; BUF_LEN],
    buf_len: usize,
    off: usize,
    any_errors: bool,
}

impl Dir {
    /// Take ownership of `fd` and read entries from it.
    pub fn new<Fd: Into<OwnedFd>>(fd: Fd) -> io::Result<Self> {
        Ok(Self {
            fd: fd.into(),
            buf: [0; BUF_LEN],
            buf_len: 0,
            off: 0,
            any_errors: false,
        })
    }

    /// The descriptor the stream reads from.
    pub fn fd(&self) -> io::Result<BorrowedFd<'_>> {
        Ok(self.fd.as_fd())
    }

    /// Borrow `fd` and read entries from an independent copy of it, so a
    /// `dup` the caller holds cannot move this stream's position.
    pub fn read_from<Fd: AsFd>(fd: Fd) -> io::Result<Self> {
        let raw = minix_std::fs::fcntl(fd.as_fd().as_raw_fd(), c::F_DUPFD, 0).map_err(errno)?;
        // SAFETY: `F_DUPFD` hands back a descriptor the caller now owns.
        Self::new(unsafe { owned_fd_from_raw(raw) })
    }

    /// Move back to the first entry.
    pub fn rewind(&mut self) {
        let _ = minix_std::fs::lseek(self.fd.as_raw_fd(), 0, c::SEEK_SET);
        self.buf_len = 0;
        self.off = 0;
        self.any_errors = false;
    }

    /// Move to the entry at `offset`, which `DirEntry::offset` reports.
    pub fn seek(&mut self, offset: i64) -> io::Result<()> {
        minix_std::fs::lseek(self.fd.as_raw_fd(), offset, c::SEEK_SET).map_err(errno)?;
        self.buf_len = 0;
        self.off = 0;
        self.any_errors = false;
        Ok(())
    }

    /// The next entry, or `None` at the end of the directory.
    pub fn read(&mut self) -> Option<io::Result<DirEntry>> {
        if self.any_errors {
            return None;
        }

        if self.off >= self.buf_len {
            let filled = match minix_std::fs::getdents(self.fd.as_raw_fd(), &mut self.buf) {
                Ok(n) => n as usize,
                Err(e) => {
                    self.any_errors = true;
                    return Some(Err(errno(e)));
                }
            };
            if filled == 0 {
                return None;
            }
            self.buf_len = filled;
            self.off = 0;
        }

        let off = self.off;
        if off + NAME_OFF > self.buf_len {
            self.off = self.buf_len;
            return None;
        }
        let reclen = usize::from(u16::from_ne_bytes([self.buf[off + 8], self.buf[off + 9]]));
        if reclen == 0 || off + reclen > self.buf_len {
            self.any_errors = true;
            return Some(Err(io::Errno::IO));
        }
        let namlen = usize::from(u16::from_ne_bytes([self.buf[off + 10], self.buf[off + 11]]));
        let mut ino = [0u8; 8];
        ino.copy_from_slice(&self.buf[off..off + 8]);
        let d_type = self.buf[off + 12];
        let name_len = namlen.min(MAX_NAME).min(reclen - NAME_OFF);
        let name = &self.buf[off + NAME_OFF..off + NAME_OFF + name_len];
        self.off = off + reclen;

        Some(
            CString::new(name)
                .map(|name| DirEntry {
                    d_type,
                    d_ino: u64::from_ne_bytes(ino),
                    name,
                })
                .map_err(|_| io::Errno::ILSEQ),
        )
    }

    /// `fstat(self)`
    pub fn stat(&self) -> io::Result<Stat> {
        super::syscalls::fstat(self.fd.as_fd())
    }

    /// `fstatfs(self)`
    pub fn statfs(&self) -> io::Result<StatFs> {
        super::syscalls::fstatfs(self.fd.as_fd())
    }

    /// `fstatvfs(self)`
    pub fn statvfs(&self) -> io::Result<StatVfs> {
        super::syscalls::fstatvfs(self.fd.as_fd())
    }

    /// `fchdir(self)`
    #[cfg(feature = "process")]
    pub fn chdir(&self) -> io::Result<()> {
        minix_std::fs::fchdir(self.fd.as_raw_fd()).map_err(errno)
    }
}

/// `Dir` holds no state that is shared implicitly, and every method that
/// changes it takes `&mut self`.
unsafe impl Send for Dir {}
unsafe impl Sync for Dir {}

impl Drop for Dir {
    fn drop(&mut self) {
        // Closing is the drop of `fd` below; there is no separate stream
        // object to release.
    }
}

impl Iterator for Dir {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        Self::read(self)
    }
}

impl fmt::Debug for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Dir").field("fd", &self.fd.as_raw_fd()).finish()
    }
}

/// One directory entry.
#[derive(Debug)]
pub struct DirEntry {
    d_type: u8,
    d_ino: u64,
    name: CString,
}

impl DirEntry {
    /// The entry's name.
    pub fn file_name(&self) -> &CStr {
        &self.name
    }

    /// The entry's type, as the filesystem reported it.
    pub fn file_type(&self) -> FileType {
        FileType::from_dirent_d_type(self.d_type)
    }

    /// The entry's inode number.
    pub fn ino(&self) -> u64 {
        self.d_ino
    }
}
