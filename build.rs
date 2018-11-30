use std::process::Command;

fn main() {
    Command::new("make")
        .args(&["-C", "quirc/"])
        .spawn()
        .expect("couldn't build quirc C library");

    println!("cargo:rustc-link-lib=static=quirc");
    println!("cargo:rustc-link-search=native=quirc/");
}
