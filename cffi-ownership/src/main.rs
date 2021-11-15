use std::os::raw::{c_int, c_void};

#[link(name = "ownership", kind = "static")]
extern "C" {
    fn take_ownership(i: *const c_int, dtor: unsafe extern "C" fn(i: *mut c_int)) -> c_void;
    fn make_memory() -> *mut c_int;
}

unsafe extern "C" fn drop_pointer(i: *mut c_int) {
    // Cから所有権を取り戻して
    Box::from_raw(i);
    // ライフタイムが尽きるので開放される
}

fn main() {
    println!("Rsut から値とメモリ開放する関数を渡してCで開放");
    let i = Box::new(1);
    unsafe { take_ownership(Box::into_raw(i), drop_pointer) };

    println!("C で確保した値をRustで利用して、Rustでメモリ開放");
    unsafe {
        let i = make_memory();
        println!("got address {:?}", i);
        println!("got {}", *i);

        // Cから渡されたメモリは手で開放する必要がある
        libc::free(i as *mut _);
    }
}
