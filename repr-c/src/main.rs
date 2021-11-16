use libc::{suseconds_t, time_t};
use std::mem;
use std::os::raw::c_int;

// #[repr(C)] をつけるとCと相互運用できる方になる
// メモリ上の表現がC互換になり、それ以外は普通のRustの構造体として扱える
#[repr(C)]
#[derive(Debug)]
struct Timeval {
    tv_sec: time_t,
    tv_usec: suseconds_t,
}

#[repr(C)]
#[derive(Debug)]
struct Timezone {
    tz_minuteswest: c_int,
    tz_dsttime: c_int,
}

extern "C" {
    // 定義した方をFFIの方に使える
    // fn gettimeofday(tv: *mut Timeval, tz: *mut Timezone) -> c_int;
    fn gettimeofday(tv: Option<&mut Timeval>, tz: Option<&mut Timezone>) -> c_int;
}

fn main() {
    unsafe {
        // https://gkuga.hatenablog.com/entry/2020/02/16/213058
        // https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#initializing-an-array-element-by-element
        let mut tv: Timeval = mem::MaybeUninit::<Timeval>::uninit().assume_init();
        // let tz: *mut Timezone = ptr::null_mut();
        // let ret = gettimeofday(&mut tv as *mut _, tz);
        let ret = gettimeofday(Some(&mut tv), None);
        if ret == -1 {
            println!("failuer");
            return;
        }
        println!("{:?}", tv);
    }
}
