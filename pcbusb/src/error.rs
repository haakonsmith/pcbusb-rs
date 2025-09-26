use core::fmt;
use std::ffi::CString;

use crate::sys::CAN_GetErrorText;

#[derive(Debug)]
pub struct Error(pub(crate) String);

impl Error {
    pub(crate) fn new(error_code: u32) -> Self {
        unsafe {
            let raw_error_msg = CString::from_vec_unchecked(Vec::with_capacity(256)).into_raw();

            CAN_GetErrorText(error_code, 0, raw_error_msg);
            Self(String::from_utf8_unchecked(
                CString::from_raw(raw_error_msg).into_bytes(),
            ))
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self(format!("I/O error: {}", err))
    }
}

#[cfg(unix)]
impl From<nix::Error> for Error {
    fn from(err: nix::Error) -> Self {
        Self(format!("System error: {}", err))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

impl embedded_can::Error for Error {
    fn kind(&self) -> embedded_can::ErrorKind {
        // TODO update to proper error handling
        embedded_can::ErrorKind::Other
    }
}
