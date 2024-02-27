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
    - To build a specific source file, e.g. `STM32H745ZITx.rs` from the command line, use the following:
    `cargo build --bin STM32H745ZITx --features STM32H745ZITx --target thumbv7em-none-eabi --profile debug-no-opt`

2. Optional: Create coredump files that can be used in automated `probe-rs` debug tests.

    - When building the app in VSCode, choose the `debug-no-opt` build profile. This will build, flash, and run the test application until it reaches the softare breakpoint in the code.
    - Using the "DEBUG CONSOLE" window in VSCode, enter of of the `dump` commands below, in the REPL command line.
    - The current tests and `dump` commands are listed below, with the coredump filename based on the cargo bin name of each binary.
      - Armv6-m:`dump target/RP2040.coredump`
      - Armv7-m:`dump target/NRF52833_xxAA.coredump`
      - RISC-V32:`dump target/esp32c3.coredump`
    - Wait for the message that says "Core dump (Includes memory ranges: <snip> successfully stored at <snip>".
    - This file can now be moved into the `probe-rs/probe-rs` repository, to be used as a test source. Please update the `probe-rs/tests/README.md` in that repository with appropriate information to ensure anyone can accurately recreate the coredump.
      - Note: To do any meaningful testing, you will probably need the appropriate binary from the `target` directory also.
  
## Adding support for new chips

Support for new chips, can be added by making the copying one of the existing source files, and renaming it using the appropriate chip name from `probe-rs-debugger list-chips`. The new file should be placed in a folder named after the target-triple. Finally, `Cargo.toml` and `build.rs` should be updated to include the new chip name and features.
