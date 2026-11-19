//! minix syscalls supporting `rustix::termios`.
//!
//! The tty server keeps a NetBSD-shaped `struct termios` behind the
//! `TIOCGETA`/`TIOCSETA` family, and `minix_std::termios` owns both the request
//! codes and a struct whose size the codes are derived from. This module
//! converts between that struct and rustix's `Termios` field by field, so the
//! two layouts are never assumed to be interchangeable.

use crate::backend::conv::{borrowed_fd, errno};
use crate::fd::BorrowedFd;
use crate::io;
use crate::pid::Pid;
use crate::termios::{
    Action, ControlModes, InputModes, LocalModes, OptionalActions, OutputModes, QueueSelector,
    SpecialCodeIndex, SpecialCodes, Termios, Winsize,
};
use minix_std::termios as tty;

fn from_tty(raw: &tty::Termios) -> Termios {
    Termios {
        input_modes: InputModes::from_bits_truncate(raw.c_iflag),
        output_modes: OutputModes::from_bits_truncate(raw.c_oflag),
        control_modes: ControlModes::from_bits_truncate(raw.c_cflag),
        local_modes: LocalModes::from_bits_truncate(raw.c_lflag),
        special_codes: SpecialCodes(raw.c_cc),
        input_speed: raw.c_ispeed as u32,
        output_speed: raw.c_ospeed as u32,
    }
}

fn to_tty(termios: &Termios) -> tty::Termios {
    tty::Termios {
        c_iflag: termios.input_modes.bits(),
        c_oflag: termios.output_modes.bits(),
        c_cflag: termios.control_modes.bits(),
        c_lflag: termios.local_modes.bits(),
        c_cc: termios.special_codes.0,
        c_ispeed: termios.input_speed as i32,
        c_ospeed: termios.output_speed as i32,
    }
}

pub(crate) fn tcgetattr(fd: BorrowedFd<'_>) -> io::Result<Termios> {
    let mut raw = tty::Termios::zeroed();
    // SAFETY: `raw` is a live `struct termios` and `fd` is a borrowed, open
    // descriptor.
    unsafe { tty::tcgetattr(borrowed_fd(fd), &mut raw) }.map_err(errno)?;
    Ok(from_tty(&raw))
}

pub(crate) fn tcsetattr(
    fd: BorrowedFd<'_>,
    optional_actions: OptionalActions,
    termios: &Termios,
) -> io::Result<()> {
    let request = match optional_actions {
        OptionalActions::Now => tty::TIOCSETA,
        OptionalActions::Drain => tty::TIOCSETAW,
        OptionalActions::Flush => tty::TIOCSETAF,
    };
    let raw = to_tty(termios);
    let arg = (&raw as *const tty::Termios).cast_mut().cast::<u8>();
    // SAFETY: `arg` points at a live `struct termios` for the duration of the
    // call, which is what the request reads.
    unsafe { minix_std::fs::ioctl(borrowed_fd(fd), request, arg) }
        .map(|_| ())
        .map_err(errno)
}

pub(crate) fn tcgetpgrp(fd: BorrowedFd<'_>) -> io::Result<Pid> {
    let mut pgrp: i32 = 0;
    // SAFETY: `pgrp` is a live `pid_t` for the request to write.
    unsafe {
        minix_std::fs::ioctl(
            borrowed_fd(fd),
            tty::TIOCGPGRP,
            (&mut pgrp as *mut i32).cast(),
        )
    }
    .map_err(errno)?;
    // A terminal with no foreground group answers 0, and `Pid` cannot hold it.
    if pgrp <= 0 {
        return Err(io::Errno::OPNOTSUPP);
    }
    // SAFETY: the check above holds `pgrp` positive.
    Ok(unsafe { Pid::from_raw_unchecked(pgrp) })
}

pub(crate) fn tcsetpgrp(fd: BorrowedFd<'_>, pid: Pid) -> io::Result<()> {
    let mut pgrp = pid.as_raw_nonzero().get();
    // SAFETY: `pgrp` is a live `pid_t` for the request to read.
    unsafe {
        minix_std::fs::ioctl(
            borrowed_fd(fd),
            tty::TIOCSPGRP,
            (&mut pgrp as *mut i32).cast(),
        )
    }
    .map(|_| ())
    .map_err(errno)
}

pub(crate) fn tcsendbreak(_fd: BorrowedFd<'_>) -> io::Result<()> {
    // A virtual console has no line to send a break down, and the tty server
    // has no request for one.
    Err(io::Errno::NOSYS)
}

pub(crate) fn tcdrain(fd: BorrowedFd<'_>) -> io::Result<()> {
    // SAFETY: `TIOCDRAIN` takes no argument.
    unsafe {
        minix_std::fs::ioctl(borrowed_fd(fd), tty::TIOCDRAIN, core::ptr::null_mut())
    }
    .map(|_| ())
    .map_err(errno)
}

pub(crate) fn tcflush(fd: BorrowedFd<'_>, queue_selector: QueueSelector) -> io::Result<()> {
    let mut mask = queue_selector as i32;
    // SAFETY: `TIOCFLUSH` reads the `FREAD`/`FWRITE` mask through the
    // argument, which points at a live `int` for the duration of the call.
    unsafe {
        minix_std::fs::ioctl(
            borrowed_fd(fd),
            tty::TIOCFLUSH,
            (&mut mask as *mut i32).cast(),
        )
    }
    .map(|_| ())
    .map_err(errno)
}

pub(crate) fn tcflow(fd: BorrowedFd<'_>, action: Action) -> io::Result<()> {
    let request = match action {
        Action::OOff => tty::TIOCSTOP,
        Action::OOn => tty::TIOCSTART,
        // `TCIOFF`/`TCION` transmit a STOP/START character, which a virtual
        // console has no line for.
        Action::IOff | Action::IOn => return Err(io::Errno::NOSYS),
    };
    // SAFETY: neither request takes an argument.
    unsafe { minix_std::fs::ioctl(borrowed_fd(fd), request, core::ptr::null_mut()) }
        .map(|_| ())
        .map_err(errno)
}

pub(crate) fn tcgetsid(_fd: BorrowedFd<'_>) -> io::Result<Pid> {
    // The tty server tracks a foreground group but not a session leader.
    Err(io::Errno::NOSYS)
}

pub(crate) fn tcsetwinsize(fd: BorrowedFd<'_>, winsize: Winsize) -> io::Result<()> {
    let raw = tty::WinSize {
        ws_row: winsize.ws_row,
        ws_col: winsize.ws_col,
        ws_xpixel: winsize.ws_xpixel,
        ws_ypixel: winsize.ws_ypixel,
    };
    let arg = (&raw as *const tty::WinSize).cast_mut().cast::<u8>();
    // SAFETY: `arg` points at a live `struct winsize` for the duration of the
    // call.
    unsafe { minix_std::fs::ioctl(borrowed_fd(fd), tty::TIOCSWINSZ, arg) }
        .map(|_| ())
        .map_err(errno)
}

pub(crate) fn tcgetwinsize(fd: BorrowedFd<'_>) -> io::Result<Winsize> {
    let mut raw = tty::WinSize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    // SAFETY: `raw` is a live `struct winsize` for the request to fill.
    unsafe {
        minix_std::fs::ioctl(
            borrowed_fd(fd),
            tty::TIOCGWINSZ,
            (&mut raw as *mut tty::WinSize).cast(),
        )
    }
    .map_err(errno)?;
    Ok(Winsize {
        ws_row: raw.ws_row,
        ws_col: raw.ws_col,
        ws_xpixel: raw.ws_xpixel,
        ws_ypixel: raw.ws_ypixel,
    })
}

pub(crate) fn set_speed(termios: &mut Termios, arbitrary_speed: u32) -> io::Result<()> {
    speed_field(arbitrary_speed)?;
    termios.input_speed = arbitrary_speed;
    termios.output_speed = arbitrary_speed;
    Ok(())
}

pub(crate) fn set_output_speed(termios: &mut Termios, arbitrary_speed: u32) -> io::Result<()> {
    speed_field(arbitrary_speed)?;
    termios.output_speed = arbitrary_speed;
    Ok(())
}

pub(crate) fn set_input_speed(termios: &mut Termios, arbitrary_speed: u32) -> io::Result<()> {
    speed_field(arbitrary_speed)?;
    termios.input_speed = arbitrary_speed;
    Ok(())
}

/// The tty server records a rate in an `i32` field, so a value that does not
/// fit is refused rather than wrapped into a different speed.
fn speed_field(arbitrary_speed: u32) -> io::Result<i32> {
    i32::try_from(arbitrary_speed).map_err(|_| io::Errno::INVAL)
}

pub(crate) fn cfmakeraw(termios: &mut Termios) {
    termios.input_modes -= InputModes::IGNBRK
        | InputModes::BRKINT
        | InputModes::PARMRK
        | InputModes::ISTRIP
        | InputModes::INLCR
        | InputModes::IGNCR
        | InputModes::ICRNL
        | InputModes::IXON;
    termios.output_modes -= OutputModes::OPOST;
    termios.local_modes -=
        LocalModes::ECHO | LocalModes::ECHONL | LocalModes::ICANON | LocalModes::ISIG | LocalModes::IEXTEN;
    termios.control_modes -= ControlModes::CSIZE | ControlModes::PARENB;
    termios.control_modes |= ControlModes::CS8;
    termios.special_codes[SpecialCodeIndex::VMIN] = 1;
    termios.special_codes[SpecialCodeIndex::VTIME] = 0;
}

pub(crate) fn isatty(fd: BorrowedFd<'_>) -> bool {
    // VFS answers an ioctl it does not route with success, so `tcgetattr`
    // succeeding says nothing about the descriptor being a terminal. The C
    // library's `isatty` uses the file type for the same reason.
    match minix_std::fs::fstat(borrowed_fd(fd)) {
        Ok(st) => st.st_mode & minix_std::fs::S_IFMT == minix_std::fs::S_IFCHR,
        Err(_) => false,
    }
}

#[cfg(feature = "alloc")]
#[cfg(feature = "fs")]
pub(crate) fn ttyname(
    fd: BorrowedFd<'_>,
    buf: &mut [core::mem::MaybeUninit<u8>],
) -> io::Result<usize> {
    // The port has one console terminal per process, so the name is always
    // `/dev/tty`; the C library's `ttyname` answers the same.
    const NAME: &[u8] = b"/dev/tty";

    if !isatty(fd) {
        return Err(io::Errno::NOTTY);
    }
    if buf.len() < NAME.len() + 1 {
        return Err(io::Errno::RANGE);
    }
    for (dst, src) in buf.iter_mut().zip(NAME) {
        dst.write(*src);
    }
    buf[NAME.len()].write(0);
    Ok(NAME.len())
}
