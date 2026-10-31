use crate::backend::c;
use crate::backend::conv::borrowed_fd;
use crate::backend::fd::{AsFd, AsRawFd as _, BorrowedFd, LibcFd};
use bitflags::bitflags;
use core::fmt;
use core::marker::PhantomData;

pub(crate) type RawPollFlags = core::ffi::c_short;

bitflags! {
    /// `POLL*` flags for use with [`poll`].
    ///
    /// [`poll`]: crate::event::poll
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct PollFlags: RawPollFlags {
        /// `POLLIN`
        const IN = c::POLLIN;
        /// `POLLPRI`
        const PRI = c::POLLPRI;
        /// `POLLOUT`
        const OUT = c::POLLOUT;
        /// `POLLRDNORM`
        const RDNORM = c::POLLRDNORM;
        /// `POLLWRNORM`
        const WRNORM = c::POLLWRNORM;
        /// `POLLRDBAND`
        const RDBAND = c::POLLRDBAND;
        /// `POLLWRBAND`
        const WRBAND = c::POLLWRBAND;
        /// `POLLERR`
        const ERR = c::POLLERR;
        /// `POLLHUP`
        const HUP = c::POLLHUP;
        /// `POLLNVAL`
        const NVAL = c::POLLNVAL;

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

/// `struct pollfd`: a descriptor and the flags for use with [`poll`].
///
/// [`poll`]: crate::event::poll
#[doc(alias = "pollfd")]
#[derive(Clone)]
#[repr(transparent)]
pub struct PollFd<'fd> {
    pollfd: c::pollfd,
    _phantom: PhantomData<BorrowedFd<'fd>>,
}

impl fmt::Debug for PollFd<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PollFd")
            .field("fd", &self.pollfd.fd)
            .field("events", &self.pollfd.events)
            .field("revents", &self.pollfd.revents)
            .finish()
    }
}

impl<'fd> PollFd<'fd> {
    /// Constructs a new `PollFd` holding `fd` and `events`.
    #[inline]
    pub fn new<Fd: AsFd>(fd: &'fd Fd, events: PollFlags) -> Self {
        Self::from_borrowed_fd(fd.as_fd(), events)
    }

    /// Sets the contained file descriptor to `fd`.
    #[inline]
    pub fn set_fd<Fd: AsFd>(&mut self, fd: &'fd Fd) {
        self.pollfd.fd = fd.as_fd().as_raw_fd() as LibcFd;
    }

    /// Clears the ready events.
    #[inline]
    pub fn clear_revents(&mut self) {
        self.pollfd.revents = 0;
    }

    /// Constructs a new `PollFd` holding `fd` and `events`.
    ///
    /// This is the same as `new`, but takes the descriptor by value, which
    /// avoids borrowing it in situations where the `BorrowedFd` is a temporary.
    #[inline]
    pub fn from_borrowed_fd(fd: BorrowedFd<'fd>, events: PollFlags) -> Self {
        Self {
            pollfd: c::pollfd {
                fd: borrowed_fd(fd),
                events: events.bits(),
                revents: 0,
            },
            _phantom: PhantomData,
        }
    }

    /// Returns the ready events.
    #[inline]
    pub fn revents(&self) -> PollFlags {
        // The port only ever reports the flags declared above, so this cannot
        // fail in practice, and an unknown bit is dropped rather than panic.
        PollFlags::from_bits_retain(self.pollfd.revents)
    }
}

impl<'fd> AsFd for PollFd<'fd> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        // SAFETY: the constructors and `set_fd` require `pollfd.fd` to be
        // valid for the `'fd` lifetime.
        unsafe { BorrowedFd::borrow_raw(self.pollfd.fd) }
    }
}
