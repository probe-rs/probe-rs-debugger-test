# Some sample applications to test `probe-rs-debugger`

The application in this repo are designed to allow consistent testing of `probe-rs-debugger`, and also to show some default configurations that users can template to enable `probe-rs-debugger` accross different hardware platforms/architectures.

The application can be run on mulitple architectures, and is controlled by conditional compile features, which are keyed on the chip name as reported by `probe-rs-debugger list-chips`. The following chips / features are pre-configured for use:

- `RP2040`
  - ARM Cortex-M0 / Armv6-M on Raspberry PICO RP2040, that uses a second PICO as a probe.
- `nRF52833_xxAA`
  - ARM Cortex-M4 / Armv7-M on a Micro:Bit v2 board
- `esp32c3`
  - RISC-V on an Espressif ESP32-C3 board

- TODO: `STM32U5A9` as a ARM Cortex-M33 / Armv8-M architecute.
- TODO: Armv7-A
- TODO: Armv8-A
- TODO: Add `embassy` async example.
- TODO: Add `rtic` async example

## Usage notes

1. Use the **VSCode probe-rs-debug extension** for `probe-rs-debugger`

    - The `.vscode/launch.json` and `.vscode/tasks.json` are preconfigured, and will adjust behaviour based on the active workspace file that is open in VSCode.
    - Open the VSCode Workspace, with the workspace file name that starts with the name of the chip you want to test. e.g. `armv7-probe-rs-debugger-test.code-workspace`.
    - In `Cargo.toml`, enable one of the features to select from the available unwind tests, e.g. `full_unwind`.

2. Optional: Create coredump files that can be used in automated `probe-rs` debug tests.

    - When building the app in VSCode, the defaul is to build, flash, and run the test application until it reaches the softare breakpoint in the code (the  Cargo.toml` feature for this is `full_unwind`)
    - Using the "DEBUG CONSOLE" window in VSCode, enter of of the `dump` commands below, in the REPL command line.
    - The current tests and `dump` commands are listed below, with the coredump filename based on the cargo bin name of each binary.
      - Armv6-m:`dump tests/RP2040_<unwind feature>.coredump`, where `<unwind feature>` is one of:
        - `full_unwind`
        - `systick`
        - `svcall`
        - `hardfault_from_usagefault` (TODO: unable to do coredump, due to multidrop error)
        - `hardfault_from_busfault` (TODO: unable to do coredump, due to multidrop error)
        - `hardfault-in-systick` (TODO: unable to do coredump, due to multidrop error)
      - Armv7-m:`dump target/NRF52833_xxAA_<unwind feature>.coredump`, where `<unwind feature>` is one of:
        - `full_unwind`
        - `systick`
        - `svcall`
        - `hardfault_from_usagefault`
        - `hardfault_from_busfault`
        - `hardfault-in-systick`
      - RISC-V32:`dump target/esp32c3_<unwind feature>.coredump`, where `<unwind feature>` is one of:
        - `full_unwind`
    - Wait for the message that says "Core dump (Includes memory ranges: ... successfully stored at ...)".
    - This file can now be moved into the `probe-rs/probe-rs` repository, to be used as a test source. Please update the `probe-rs/tests/README.md` in that repository with appropriate information to ensure anyone can accurately recreate the coredump.
      - Note: To do any meaningful testing, you will probably need the appropriate binary from the `target` directory also, and the convention is to rename it to the same base file name as the corresponding `.coredump` file,
      and to add a `.elf` extension.
  
## Adding support for new chips

Support for new chips, can be added by:

- Copy one of the existing source files, and renaming it using the appropriate chip name from `probe-rs-debugger list-chips`.
- Create a `.code-workspace` file for the new chip, and edit all sections indicated with a `//CONFIGURE:` tag in the other examples.
- Finally, `Cargo.toml` and `build.rs` should be updated to include the new chip name and features.
