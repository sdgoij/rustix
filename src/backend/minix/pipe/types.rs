use crate::backend::c;
use bitflags::bitflags;

bitflags! {
    /// `O_*` constants for use with [`pipe_with`].
    ///
    /// [`pipe_with`]: crate::pipe::pipe_with
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct PipeFlags: u32 {
        /// `O_CLOEXEC`
        const CLOEXEC = bitcast!(c::O_CLOEXEC);
        /// `O_DIRECT`
        const DIRECT = bitcast!(c::O_DIRECT);
        /// `O_NONBLOCK`
        const NONBLOCK = bitcast!(c::O_NONBLOCK);

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}
