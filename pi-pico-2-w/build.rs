use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=memory.x");

    #[cfg(not(feature = "skip-cyw43-firmware"))]
    download_cyw43_firmware();

    // Specify linker arguments.

    // `--nmagic` is required if memory section addresses are not aligned to 0x10000,
    // for example the FLASH and RAM sections in your `memory.x`.
    // See https://github.com/rust-embedded/cortex-m-quickstart/pull/95
    println!("cargo:rustc-link-arg=--nmagic");

    // Set the linker script to the one provided by cortex-m-rt.
    println!("cargo:rustc-link-arg=-Tlink.x");

    // Set the linker script of the defmt
    println!("cargo:rustc-link-arg=-Tdefmt.x");
}

#[cfg(not(feature = "skip-cyw43-firmware"))]
fn download_cyw43_firmware() {
    let download_folder = Path::new("cyw43-firmware");
    let url_base = "https://github.com/embassy-rs/embassy/raw/refs/heads/main/cyw43-firmware";
    let file_names = [
        "43439A0.bin",
        "43439A0_btfw.bin",
        "43439A0_clm.bin",
        "LICENSE-permissive-binary-license-1.0.txt",
        "README.md",
    ];

    println!("cargo:rerun-if-changed=cyw43-firmware");
    fs::create_dir_all(download_folder).expect("Failed to create cyw43-firmware directory");

    for file in file_names {
        let path = download_folder.join(file);
        if path.exists() {
            continue;
        }

        let url = format!("{}/{}", url_base, file);
        let response = reqwest::blocking::get(&url)
            .unwrap_or_else(|err| panic!("Failed to download {}: {}", url, err));
        let content = response
            .bytes()
            .unwrap_or_else(|err| panic!("Failed to read {}: {}", url, err));
        fs::write(&path, &content).expect("Failed to write cyw43 firmware file");
    }
}
