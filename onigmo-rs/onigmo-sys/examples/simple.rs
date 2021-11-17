use onigmo_sys::*;
use std::mem;
use std::str::from_utf8_unchecked;

fn main() {
    unsafe {
        let pattern = b"a(.*)b|[e-f]+";
        let s = b"zzzzaffffffffb";
        // let s = b"aaaaaaaaaaaaa";
        // let s = b"azzbaaaaaaaaa";

        // メモリをスタックに確保
        let mut reg: regex_t = mem::MaybeUninit::uninit().assume_init();
        let mut einfo: OnigErrorInfo = mem::MaybeUninit::uninit().assume_init();

        let r = onig_new_without_alloc(
            &mut reg as *mut _,
            pattern as *const OnigUChar,
            (pattern as *const OnigUChar).offset(pattern.len() as isize),
            ONIG_OPTION_NONE,
            &OnigEncodingUTF_8,
            OnigDefaultSyntax,
            &mut einfo,
        );

        if (r as ::std::os::raw::c_uint) != ONIG_NORMAL {
            let s: &mut [OnigUChar] = &mut [0; ONIG_MAX_ERROR_MESSAGE_LEN as usize];
            onig_error_code_to_str(s as *mut _ as *mut _, r as OnigPosition, &einfo);
            println!("ERROR: {}\n", from_utf8_unchecked(s));
            return;
        }

        let region = onig_region_new();

        // マッチ対象文字列の終端
        let end = (s as *const OnigUChar).offset(s.len() as isize);
        // マッチ開始位置
        let start = s as *const _;
        // マッチ終了位置
        let range = end;

        let mut r = onig_search(
            &mut reg,
            s as *const _,
            end,
            start,
            range,
            region,
            ONIG_OPTION_NONE,
        );

        if 0 <= r {
            println!("match at {}", r);
            let region = region.as_ref().unwrap();
            for i in 0..(region.num_regs) {
                println!(
                    "{}: ({}-{})",
                    i,
                    *region.beg.offset(i as isize),
                    *region.end.offset(i as isize)
                );
            }
            r = 0;
        } else if (r as ::std::os::raw::c_int) == ONIG_MISMATCH {
            println!("search fail");
            r = -1;
        } else {
            let s: &mut [OnigUChar] = &mut [0; ONIG_MAX_ERROR_MESSAGE_LEN as usize];
            onig_error_code_to_str(s as *mut _ as *mut _, r as OnigPosition, &einfo);
            println!("ERROR: {}\n", from_utf8_unchecked(s));
            std::process::exit(-1);
        }

        onig_region_free(region, 1);
        onig_free_body(&mut reg);
        onig_end();
        std::process::exit(r as i32);
    }
}
