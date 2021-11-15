use std::os::raw::{c_char, c_int};

enum File {}

extern "C" {
    // https://devdocs.io/c/io/fopen
    fn fopen(fname: *const c_char, mode: *const c_char) -> *mut File;

    // https://devdocs.io/c/io/fgetc
    fn fgetc(stream: *mut File) -> c_int;

    // https://devdocs.io/c/io/fclose
    fn fclose(stream: *mut File) -> c_int;
}

fn main() {
    unsafe {
        // NULL終端したバイト列を作成し、キャストを行いCの文字列を作成する
        let fname: *const c_char = b"Cargo.toml\0".as_ptr() as *const _;
        let mode: *const c_char = b"r\0".as_ptr() as *const _;

        let file = fopen(fname, mode);
        if file.is_null() {
            println!("open file failed");
            return;
        }
        loop {
            let c = fgetc(file);
            if c == -1 {
                break;
            } else {
                let c = c as u8 as char;
                print!("{}", c);
            }
        }

        if fclose(file) == -1 {
            println!("close file failed");
        }
    }
}
