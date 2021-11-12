fn main() {
    let x = 1;
    let xptr: *const i32 = &x;
    println!("&x = {}", &x);
    println!("x = {}", x);

    unsafe {
        let x = *xptr;
        println!("xptr = {:?}", xptr);
        println!("*xptr = {}", x);
    }

    println!("---------------------------------");

    let mut y = 2;
    let yptr: *mut i32 = &mut y;
    println!("&y = {}", &y);
    println!("y = {}", y);
    unsafe {
        println!("yptr = {:?}", yptr);
        *yptr = 3;
        println!("*yptr = {:?}", *yptr);
    }

    println!("---------------------------------");

    let z = Box::new(4);
    let bzptr: *const Box<i32> = &z;
    let zptr: *const i32 = &*z;
    println!("Boxed z = {}", z);
    println!("*z = {}", *z);
    println!("&*z = {}", &*z);
    println!("bzptr = {:?}", bzptr);
    unsafe {
        println!("*bzptr = {:?}", *bzptr);
    }
    println!("zptr = {:?}", zptr);

    println!("---------------------------------");

    let s: &[u8] = b"abc";
    let sptr: *const u8 = s.as_ptr();
    println!("s {:?}", s);
    println!("sptr {:?}", sptr);
    unsafe {
        let st = std::slice::from_raw_parts(sptr, s.len());
        println!("sptr {:?}", st);
    }

    println!("---------------------------");

    let boxed = Box::new(true);
    println!("boxed {:?}", boxed);
    let ptr: *mut bool = Box::into_raw(boxed);
    // println!("boxed {:?}", boxed); // 所有権を渡しているのでここで valuew borrowed になる
    println!("boxed-ptr {:?}", ptr);
    unsafe {
        let boxed = Box::from_raw(ptr);
        println!("boxed {:?}", boxed);
    }
    unsafe {
        // 気をつけないと同じポインタから複数のBoxが作れてしまう。
        // 以下のエラーが出る。
        // ptr(10178,0x10856ee00) malloc: *** error for object 0x7ff8aec05da0: pointer being freed was not allocated
        // ptr(10178,0x10856ee00) malloc: *** set a breakpoint in malloc_error_break to debug
        let boxed2 = Box::from_raw(ptr);
        println!("boxed2 {:?}", boxed2);
    }
}
