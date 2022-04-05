//! This build script copies the `Core0.x` and "Core1.x" file from the crate /src into
//! a directory where the linker can always find it at build time.
#![allow(unused_imports)]
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    #[cfg(feature = "STM32H745ZITx")]
    let link_bytes = include_bytes!("linker_files/STM32H745ZITx_memory.x");
    #[cfg(feature = "RP2040")]
    let link_bytes = include_bytes!("linker_files/RP2040_memory.x");
    #[cfg(feature = "nRF52833_xxAA")]
    let link_bytes = include_bytes!("linker_files/nRF52833_xxAA_memory.x");
    #[cfg(any(
        feature = "STM32H745ZITx",
        feature = "RP2040",
        feature = "nRF52833_xxAA"
    ))]
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(link_bytes)
        .unwrap();
    #[cfg(any(
        feature = "STM32H745ZITx",
        feature = "RP2040",
        feature = "nRF52833_xxAA"
    ))]
    println!("cargo:rustc-link-arg-bins=-Tlink.x");

    #[cfg(feature = "esp32c3")]
    let link_bytes = include_bytes!("linker_files/esp32c3_memory.x");
    #[cfg(feature = "esp32c3")]
    File::create(out.join("linkall.x"))
        .unwrap()
        .write_all(link_bytes)
        .unwrap();
    #[cfg(feature = "esp32c3")]
    println!("cargo:rustc-link-arg-bins=-Tlinkall.x");

    println!("cargo:rustc-link-search={}", out.display());

    // println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=linker_files/esp32c3_memory.x");
    println!("cargo:rerun-if-changed=linker_files/STM32H745ZITx_memory.x");
    println!("cargo:rerun-if-changed=linker_files/RP2040_memory.x");
    println!("cargo:rerun-if-changed=linker_files/nRF52833_xxAA_memory.x");
}
