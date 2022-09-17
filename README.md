# Some sample applications to test `probe-rs-debugger`

The application in this repo are designed to allow consistent testing of `probe-rs-debugger`, and also to show some default configurations that users can template to enable `probe-rs-debugger` accross different hardware platforms/architectures.

The application can be run on mulitple architectures, and is controlled by conditional compile features, which are keyed on the chip name as reported by `probe-rs-debugger list-chips`. The following chips / features are pre-configured for use:

- `STM32H745ZITx`
  - ARM Cortex-M7 core on a ST Nucleo H745ZI-Q board
- `nRF52833_xxAA`
  - ARM Cortex-M4 on a Micro:Bit v2 board
- `esp32c3`
  - RISCV on an Espressif ESP32-C3 board
- `RP2040`
  - ARM Cortex-M0 on Raspberry PICO RP2040, that uses a second PICO as a probe.

## Usage notes:

Use the **VSCode probe-rs-debug extension** for `probe-rs-debugger`

- Edit the applicable `<chipname>.code-workspace` file, and adjust the locations marked with a **// CONFIGURE:** comment.
- The `.vscode/launch.json` and `.vscode/tasks.json` are preconfigured, and will adjust to the values configured in the `<chipname>.code-workspace` file.
- Open the applicable workspace file with 'Open Workspace from File ...', and you should be ready to debug.

## Adding support for new chips

Support for new chips, can be added by making the following modifications, using the appropriate chip name from `probe-rs-debugger list-chips`

1. Copy one of the existing VSCode workspace files to an new file named: `<chipname>.code_workspace` - Update all locations marked with a **// CONFIGURE:** comment.
   2.Edit the `Cargo.toml` file: - Add a [feature] for your chip. - If your feature uses a different [target.\<triple>] , then create a new dependency for it, using the existing target platforms as a template.
2. Edit the `src/main.rs` file:
   - Add your chip [feature] to the existing conditional compile structures, or add new ones if your chip requires custom crates.
3. Add the appropriate linker file to the `linker_files` folder.
   - Note, it must be named "<chip_name>\_memory.x"
4. Add the appropriate CMSIS-SVD file to the `svd_files` folder.
   - Note, it must be named "<chip_name>.svd"
