use onigmo_sys::*;
use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Error(OnigPosition, Option<OnigErrorInfo>, String);
pub type Result<T> = ::std::result::Result<T, Error>;

impl Error {
    pub fn new(pos: OnigPosition, error_info: Option<OnigErrorInfo>) -> Self {
        use std::str::from_utf8;
        let s: &mut [OnigUChar] = &mut [0; ONIG_MAX_ERROR_MESSAGE_LEN as usize];
        unsafe {
            let size = match error_info {
                Some(ei) => onig_error_code_to_str(s as *mut _ as *mut _, pos, ei),
                None => onig_error_code_to_str(s as *mut _ as *mut _, pos),
            };
            let size = size as usize;
            let s = from_utf8(&s[0..size]).unwrap().to_string();
            Error(pos, error_info, s)
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ERROR: {}\n", self.2)
    }
}

impl error::Error for Error {}
