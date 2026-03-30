use core::fmt;
use std::ffi::{CStr, c_char};

use crate::sys::CAN_GetErrorText;

#[derive(Debug)]
pub struct Error(pub(crate) String);

impl Error {
    pub(crate) fn new(error_code: u32) -> Self {
        let mut buffer = vec![0u8; 256];
        unsafe {
            CAN_GetErrorText(error_code, 0, buffer.as_mut_ptr() as *mut c_char);
            let msg = CStr::from_ptr(buffer.as_ptr() as *const c_char);
            Self(msg.to_string_lossy().into_owned())
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self(format!("I/O error: {}", err))
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
