use crate::onig_error::{Error, Result};
use crate::region::Region;
use onigmo_sys::*;
use std::mem;
use std::ops::Drop;

pub struct Regex(regex_t);
impl Regex {
    pub fn new(pattern: &str) -> Result<Self> {
        unsafe {
            // メモリをスタックに確保
            let mut reg: regex_t = mem::MaybeUninit::uninit().assume_init();
            let mut einfo: OnigErrorInfo = mem::MaybeUninit::uninit().assume_init();
            let pattern = pattern.as_bytes();

            let r = onig_new_without_alloc(
                &mut reg as *mut _,
                pattern.as_ptr() as *const OnigUChar,
                (pattern.as_ptr() as *const OnigUChar).offset(pattern.len() as isize),
                ONIG_OPTION_NONE,
                &OnigEncodingUTF_8,
                OnigDefaultSyntax,
                &mut einfo,
            );

            if (r as ::std::os::raw::c_uint) == ONIG_NORMAL {
                Ok(Regex(reg))
            } else {
                Err(Error::new(r as OnigPosition, Some(einfo)))
            }
        }
    }

    pub fn search(&mut self, s: &str) -> Option<Region> {
        unsafe {
            let s = s.as_bytes();
            let start = s.as_ptr();
            let end = start.offset(s.len() as isize);
            let range = end;
            let mut region = Region::new()?;

            let pos = onig_search(
                &mut self.0,
                start,
                end,
                start,
                range,
                region.as_ptr_mut(),
                ONIG_OPTION_NONE,
            );

            if 0 <= pos {
                Some(region)
            } else {
                debug_assert!(pos as std::os::raw::c_int == ONIG_MISMATCH);
                None
            }
        }
    }
}

impl Drop for Regex {
    fn drop(&mut self) {
        unsafe { onig_free_body(&mut self.0) }
    }
}
