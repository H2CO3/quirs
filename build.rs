use std::fs;
use std::env;
use std::process::Command;

fn main() {

    Command::new("make")
        .args(&["libquirc.a", "-C", "quirc/"])
        .status()
        .expect("couldn't build quirc C library");

    let out_dir = env::var("OUT_DIR")
        .expect("missing OUT_DIR env var");

    fs::copy("quirc/libquirc.a", out_dir.clone() + "/libquirc.a")
        .expect("couldn't copy libquirc.a to OUT_DIR");

    Command::new("make")
        .args(&["clean", "-C", "quirc/"])
        .status()
        .expect("couldn't make clean");

    println!("cargo:rustc-link-lib=static=quirc");
    println!("cargo:rustc-link-search=native={}", out_dir);
}
