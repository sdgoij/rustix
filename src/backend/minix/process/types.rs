use crate::backend::c;

/// A resource value for use with [`getrlimit`], [`setrlimit`], and
/// [`prlimit`].
///
/// [`getrlimit`]: crate::process::getrlimit
/// [`setrlimit`]: crate::process::setrlimit
/// [`prlimit`]: crate::process::prlimit
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
#[non_exhaustive]
pub enum Resource {
    /// `RLIMIT_CPU`
    Cpu = bitcast!(c::RLIMIT_CPU),
    /// `RLIMIT_FSIZE`
    Fsize = bitcast!(c::RLIMIT_FSIZE),
    /// `RLIMIT_DATA`
    Data = bitcast!(c::RLIMIT_DATA),
    /// `RLIMIT_STACK`
    Stack = bitcast!(c::RLIMIT_STACK),
    /// `RLIMIT_CORE`
    Core = bitcast!(c::RLIMIT_CORE),
    /// `RLIMIT_RSS`
    Rss = bitcast!(c::RLIMIT_RSS),
    /// `RLIMIT_NPROC`
    Nproc = bitcast!(c::RLIMIT_NPROC),
    /// `RLIMIT_NOFILE`
    Nofile = bitcast!(c::RLIMIT_NOFILE),
    /// `RLIMIT_MEMLOCK`
    Memlock = bitcast!(c::RLIMIT_MEMLOCK),
    /// `RLIMIT_AS`
    As = bitcast!(c::RLIMIT_AS),
    /// `RLIMIT_LOCKS`
    Locks = bitcast!(c::RLIMIT_LOCKS),
    /// `RLIMIT_SIGPENDING`
    Sigpending = bitcast!(c::RLIMIT_SIGPENDING),
    /// `RLIMIT_MSGQUEUE`
    Msgqueue = bitcast!(c::RLIMIT_MSGQUEUE),
    /// `RLIMIT_NICE`
    Nice = bitcast!(c::RLIMIT_NICE),
    /// `RLIMIT_RTPRIO`
    Rtprio = bitcast!(c::RLIMIT_RTPRIO),
    /// `RLIMIT_RTTIME`
    Rttime = bitcast!(c::RLIMIT_RTTIME),
}

/// The identifier type `procctl` takes, which the port does not implement.
pub(crate) type RawId = c::id_t;
