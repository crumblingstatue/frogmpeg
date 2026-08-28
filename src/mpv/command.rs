use std::ffi::{CStr, CString};

/// A command to make mpv perform some kind of operation
///
/// # Safety
///
/// Must make sure `ARGS_COUNT` is correct
pub unsafe trait Command {
    const NAME: &'static CStr;
    type const ARGS_COUNT: usize;
    const ARGS_ARRAY_LEN: usize = const { Self::ARGS_COUNT + 2 };
    fn args(&self) -> [CString; Self::ARGS_COUNT];
}
