extern crate cc;

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    if target.starts_with("thumbv") {
        if env::var_os("CARGO_FEATURE_INLINE_ASM").is_none() {
            cc::Build::new().file("asm.s").compile("asm");
        }

        println!("cargo:rustc-cfg=thumb");
    }
}
