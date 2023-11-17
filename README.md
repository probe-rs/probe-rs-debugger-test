# Some sample applications to test `probe-rs-debugger`

The application in this repo are designed to allow consistent testing of `probe-rs-debugger`, and also to show some default configurations that users can template to enable `probe-rs-debugger` accross different hardware platforms/architectures.

The application can be run on mulitple architectures, and is controlled by conditional compile features, which are keyed on the chip name as reported by `probe-rs-debugger list-chips`. The following chips / features are pre-configured for use:

- `STM32H745ZITx`
  - ARM Cortex-M7 / Armv7-M core on a ST Nucleo H745ZI-Q board
  - TODO: Create multi-core debug examples.
  - TODO: Add `rtic` async to example.
- `STM32C031C6Tx`
  - ARM Cortex-M0 / Armv6-M core on a STM32C0316-DK board
- `nRF52833_xxAA`
  - ARM Cortex-M4 / Armv6-M on a Micro:Bit v2 board
- `esp32c3`
  - RISC-V on an Espressif ESP32-C3 board
  - TODO: Requires manual editing of 'launch.json' to enable EDF boot loader.
- `RP2040`
  - ARM Cortex-M0 / Armv6-M on Raspberry PICO RP2040, that uses a second PICO as a probe.
  - TODO: Add `embassy` async to example.
- TODO: `STM32U5A9` as a ARM Cortex-M33 / Armv8-M architecute.
- TODO: Armv7-A
- TODO: Armv8-A

## Usage notes:

1. Use the **VSCode probe-rs-debug extension** for `probe-rs-debugger`

    - The `.vscode/launch.json` and `.vscode/tasks.json` are preconfigured, and will adjust behaviour based on the active source file in the editor. The configuration uses VSCode variables referencing both the file name and the parent folder name, to determine the correct values in the configuration to use.
    - The `.vscode/launch.json` prompts for levels of optimization required. The intention is to simplify the creation of various binaries for automated testing of `probe-rs` debug api.

2. Optional: Create coredump files that can be used in automated `probe-rs` debug tests.

    - When launching the app in VSCode, choose the `debug-no-opt` launch profile. This will build, flash, and run the test application until it reaches the softare breakpoint in the code.
    - The memory regions required in the coredump as follows:
      - text: Check the `cargo-size` terminal window to see the memory locations and sizes of the `.rodata` location.
      - data: Check the appropriate linker file for the region that corresponds to the `.data` secion of the binary, and use the memory location and size from the linker file.
    - The current tests and `dump` commands are listed below, with the coredump filename based on the cargo bin name of each binary.
      - Armv6-m:`dump 0x20000000 0x4000 0x1000b150 0x1b00 target/RP2040.coredump`
      - Armv7-m:`dump 0x20000000 0x4000 0x4cf0 0x1070 target/NRF52833_xxAA.coredump`
      - RISC-V32: Currently experiencing issues ... WIP
        - `dump 0x3FC80000 0x18010 single_read_bad.coredump`
        - `dump 0x3FC80000 0x18000 0x3FC98000 0x10 two_part_read_ok.coredump`
    - This file can now be moved into the `probe-rs/probe-rs` repository, to be used as a test source. Please update the `probe-rs/tests/README.md` in that repository with appropriate information to ensure anyone can accurately recreate the coredump.
      - Note: To do any meaningful testing, you will probably need the appropriate binary from the `target` directory also.
  


## Adding support for new chips

Support for new chips, can be added by making the copying one of the existing source files, and renaming it using the appropriate chip name from `probe-rs-debugger list-chips`. The new file should be placed in a folder named after the target-triple. Finally, `Cargo.toml` and `build.rs` should be updated to include the new chip name and features.
