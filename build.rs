use std::env;

fn main() {
    let mut cfg = cmake::Config::new("src/cqdb");
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "macos" {
        let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
        if target_arch == "x86_64" {
            cfg.define("CMAKE_OSX_ARCHITECTURES", "x86_64");
        }
    }
    let dst = cfg.build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=cqdb");
    println!("cargo:root={}", dst.to_str().unwrap());
    println!("cargo:include={}/include", dst.to_str().unwrap());
}
