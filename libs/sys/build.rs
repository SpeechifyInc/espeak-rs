extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

static STATIC_LIBRARIES: &[&str] = &["stdc++", "espeak-ng"];

fn add_arch(arch: &str) {
    println!("cargo:rustc-link-search=native=/usr/lib/{arch}");
    println!("cargo:rustc-link-search=native=/usr/lib/gcc/{arch}/10");
    println!("cargo:rustc-link-search=native=/usr/lib/gcc/{arch}/11");
}

fn add_search_paths() {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    match target_arch.as_str() {
        "aarch64" => {
            add_arch("aarch64-linux-gnu");
        }
        "x86" | "x86_64" => {
            add_arch("x86_64-linux-gnu");
        }
        _ => panic!("Unsupported architecture: {}", target_arch),
    }
    println!("cargo:rustc-link-search=native=/usr/lib");
    println!("cargo:rustc-link-search=native=/usr/local/lib");
}

fn build_static_espeak_ng() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let espeak_dir = format!("{out_dir}/espeak-ng");
    let espeak_build_dir = format!("{espeak_dir}/build");
    let espeak_lib_dir = format!("{espeak_build_dir}/lib");

    Command::new("git")
        .args(&[
            "clone",
            "https://github.com/espeak-ng/espeak-ng",
            &espeak_dir,
        ])
        .status()
        .expect("Failed to clone espeak-ng");
    Command::new("./autogen.sh")
        .current_dir(&espeak_dir)
        .status()
        .expect("Failed to run autogen for espeak-ng");
    Command::new("./configure")
        .args(&[
            "--prefix",
            &espeak_build_dir,
            "--without-sonic",
            "--without-pcaudiolib",
            "--without-speechplayer",
            "--enable-static",
            "--disable-shared",
        ])
        .current_dir(&espeak_dir)
        .status()
        .expect("Failed to configure espeak-ng");
    Command::new("make")
        .current_dir(&espeak_dir)
        .status()
        .expect("Failed to build espeak-ng");
    Command::new("make")
        .args(&["install"])
        .current_dir(&espeak_dir)
        .status()
        .expect("Failed to install espeak-ng");

    println!("cargo:rustc-link-search=native={espeak_lib_dir}");
}

fn main() {
    if cfg!(feature = "static") {
        build_static_espeak_ng();
        add_search_paths();
        for lib in STATIC_LIBRARIES {
            println!("cargo:rustc-link-lib=static={lib}");
        }
    } else {
        println!("cargo:rustc-link-lib=espeak-ng");
    }

    println!("cargo:rerun-if-changed=headers/wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("headers/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
