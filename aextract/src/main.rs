#[cfg(target_family = "unix")]
include!(concat!(env!("OUT_DIR"), "/binaries.rs"));

#[cfg(target_family = "windows")]
include!(concat!(env!("OUT_DIR"), "\\binaries.rs"));

fn main() {
    println!("Hello, world!");
}
