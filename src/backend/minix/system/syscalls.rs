//! minix syscalls supporting `rustix::system`.
//!
//! PM serves `uname` one field at a time through `PM_SYSUNAME`. Everything
//! else in this group is Linux-only in the public layer, so there is nothing
//! else to declare here.

use super::types::RawUname;
use crate::backend::c;
use minix_std::process::{
    self, UTS_MACHINE, UTS_NODENAME, UTS_RELEASE, UTS_SYSNAME, UTS_VERSION,
};

/// The system identification, as PM reports it.
///
/// A field PM does not serve stays empty, which is what the port's C library
/// does too, rather than failing the whole call.
#[inline]
pub(crate) fn uname() -> RawUname {
    let mut uname: RawUname = unsafe {
        // SAFETY: `utsname` is six arrays of `c_char`, so all-zero is a valid
        // value and every field is written or left as an empty string.
        core::mem::zeroed()
    };
    fill(UTS_SYSNAME, &mut uname.sysname);
    fill(UTS_NODENAME, &mut uname.nodename);
    fill(UTS_RELEASE, &mut uname.release);
    fill(UTS_VERSION, &mut uname.version);
    fill(UTS_MACHINE, &mut uname.machine);
    // PM's table has no domain name entry, so `domainname` stays empty.
    uname
}

fn fill(field: i32, dst: &mut [c::c_char; 65]) {
    if unsafe { process::sysuname(field, dst.as_mut_ptr().cast(), dst.len()) }.is_err() {
        dst[0] = 0;
    }
}
