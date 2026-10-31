use crate::backend::c;
use bitflags::bitflags;

bitflags! {
    /// `EFD_*` flags for use with [`eventfd`].
    ///
    /// [`eventfd`]: crate::event::eventfd
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct EventfdFlags: u32 {
        /// `EFD_CLOEXEC`
        const CLOEXEC = bitcast!(c::EFD_CLOEXEC);
        /// `EFD_NONBLOCK`
        const NONBLOCK = bitcast!(c::EFD_NONBLOCK);
        /// `EFD_SEMAPHORE`
        const SEMAPHORE = bitcast!(c::EFD_SEMAPHORE);

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}
