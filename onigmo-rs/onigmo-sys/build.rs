use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=onigmo");
    let bindings = bindgen::Builder::default()
        // wrapper.h:1:10: fatal error: 'onigmo.h' file not found
        // というエラーが出るので、includeパスを指定してあげれば良さそう
        // https://users.rust-lang.org/t/bindgen-cant-find-included-file/62687/2
        .clang_args(&["-I/usr/local/include"])
        .header("wrapper.h")
        .rust_target(bindgen::RustTarget::Stable_1_28)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!")
}
