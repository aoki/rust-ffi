extern crate onigmo as onig;
use std::str::from_utf8;

fn main() {
    let mut reg = onig::regex::Regex::new("a(.*)b|[e-f]+").unwrap();
    let s = "zzzzafffffffb";
    match reg.search(s) {
        Some(ret) => {
            for (beg, end) in ret.positions() {
                println!("{}", from_utf8(&s.as_bytes()[beg..end]).unwrap());
            }
        }
        None => println!("not match"),
    }
}
