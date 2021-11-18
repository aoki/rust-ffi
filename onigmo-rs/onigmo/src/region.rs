use onigmo_sys::*;
use std::{mem, ops::Range, ptr::NonNull};

#[derive(Debug, Clone)]
pub struct PositionIter<'a>(&'a Region, Range<i32>);

#[derive(Debug)]
pub struct Region(NonNull<OnigRegion>);

impl Region {
    pub fn new() -> Option<Self> {
        unsafe {
            let region: *mut OnigRegion = onig_region_new();
            Some(Region(NonNull::new(region)?))
        }
    }

    pub fn as_ptr_mut(&mut self) -> *mut OnigRegion {
        self.0.as_ptr()
    }

    fn as_ptr(&self) -> *const OnigRegion {
        self.0.as_ptr()
    }

    /// 位置情報のイテレータを取り出す
    pub fn positions(&self) -> PositionIter {
        let num_regs;
        unsafe {
            num_regs = (*self.as_ptr()).num_regs;
        }
        PositionIter(self, 0..num_regs)
    }
}

impl Drop for Region {
    fn drop(&mut self) {
        unsafe { onig_region_free(self.0.as_ptr(), 1) }
    }
}

impl Clone for Region {
    fn clone(&self) -> Self {
        unsafe {
            let to: *mut OnigRegion = mem::MaybeUninit::uninit().assume_init();
            onig_region_copy(to, self.0.as_ptr());
            Region(NonNull::new_unchecked(to))
        }
    }
}

impl<'a> Iterator for PositionIter<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let region = *(self.0).as_ptr();
            self.1.next().map(|i| {
                (
                    *region.beg.offset(i as isize) as usize,
                    *region.end.offset(i as isize) as usize,
                )
            })
        }
    }
}
