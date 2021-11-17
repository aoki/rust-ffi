use onigmo_sys::*;

fn main() {
    unsafe {
        let pattern = b"a(.*)b|[e-f]+";
        let s = b"zzzzaffffffffb";
    }
}
