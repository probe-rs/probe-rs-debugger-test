#![allow(unused_imports)]
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    #[cfg(any(
        feature = "STM32C031C6Tx",
        feature = "STM32H745ZITx",
        feature = "RP2040",
        feature = "nRF52833_xxAA",
        feature = "esp32c3"
    ))]
    {
        // SECTION: Get `chip_name` from the feature name.
        #[cfg(feature = "STM32C031C6Tx")]
        let chip_name = "STM32C031C6Tx";
        #[cfg(feature = "STM32H745ZITx")]
        let chip_name = "STM32H745ZITx";
        #[cfg(feature = "RP2040")]
        let chip_name = "RP2040";
        #[cfg(feature = "nRF52833_xxAA")]
        let chip_name = "nRF52833_xxAA";
        #[cfg(feature = "esp32c3")]
        let chip_name = "esp32c3";

        // SECTION: Set the target triple environment variable.
        #[cfg(feature = "STM32C031C6Tx")]
        println!("cargo:rustc-env=TARGET=thumbv6m-none-eabi");
        #[cfg(feature = "STM32H745ZITx")]
        println!("cargo:rustc-env=TARGET=thumbv7em-none-eabihf");
        #[cfg(feature = "RP2040")]
        println!("cargo:rustc-env=TARGET=thumbv6m-none-eabi");
        #[cfg(feature = "nRF52833_xxAA")]
        println!("build:target=thumbv7em-none-eabihf");
        #[cfg(feature = "esp32c3")]
        println!("cargo:rustc-env=TARGET=riscv32imac-unknown-none-elf");

        // SECTION: Get the directory where build script livesfrom the environment.
        let workspace_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

        // SECTION: The name of the link script used by the linker.
        #[cfg(any(
            feature = "STM32C031C6Tx",
            feature = "STM32H745ZITx",
            feature = "RP2040",
            feature = "nRF52833_xxAA"
        ))]
        println!("cargo:rustc-link-arg-bins=-Tlink.x");

        #[cfg(feature = "esp32c3")]
        {
            // esp32c3-hal uses a unique linker name to manage the various boot options.
            println!("cargo:rustc-link-arg-bins=-Tlinkall.x");
            println!("")
            // Note: The items below are required for atomic emulation on the esp32c3, when using the `-imc-` target.
            //   # comment the cfgs below if you do _not_ wish to emulate atomics.
            //   # enable the atomic codegen option for RISCV
            // println!("cargo:rustc-link-args=-Ctarget-feature=+a");
            // //   # tell the core library have atomics even though it's not specified in the target definition
            // println!("cargo:rustc-cfg=target_has_atomic_load_store");
            // println!("cargo:rustc-cfg=target_has_atomic_load_store=\"8\"");
            // println!("cargo:rustc-cfg=target_has_atomic_load_store=\"16\"");
            // println!("cargo:rustc-cfg=target_has_atomic_load_store=\"32\"");
            // println!("cargo:rustc-cfg=target_has_atomic_load_store=\"ptr\"");
            // // //   # enable cas
            // println!("cargo:rustc-cfg=target_has_atomic");
            // println!("cargo:rustc-cfg=target_has_atomic=\"8\"");
            // println!("cargo:rustc-cfg=target_has_atomic=\"16\"");
            // println!("cargo:rustc-cfg=target_has_atomic=\"32\"");
            // println!("cargo:rustc-cfg=target_has_atomic=\"ptr\"");
        }

        // SECTION: Instead of copying the linker scripts around, we can now tell rustc where to look for them.
        // This tells cargo where to find additional '.x' files -- usually the "INCLUDE <file>.x" in the above link script.
        let link_search_dir = PathBuf::from(workspace_dir)
            .join("linker_files")
            .join(chip_name);
        // "\"build.rustflags=['-L${workspaceFolder}/linker_files/${fileBasenameNoExtension}/']\""
        println!("cargo:rustc-link-search={}", link_search_dir.display());

        // SECTION: By default, Cargo will re-run a build script whenever
        // any file in the project changes. By specifying `memory.x`
        // here, we ensure the build script is only re-run when
        // `memory.x` is changed.
        println!("cargo:rerun-if-changed=build.rs");
        println!("cargo:rerun-if-changed=Cargo.toml");
        println!("cargo:rerun-if-changed=linker_files/*/memory.x");
    }
}
