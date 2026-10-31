use crate::backend::c;

/// `struct utsname`, which PM fills one field at a time.
pub(crate) type RawUname = c::utsname;
