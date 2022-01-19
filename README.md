# Some sample applications to test `probe-rs-debugger`

The applications in this repo are all very similar in their `main.rs`. The purpose is to allow consistent testing of `probe-rs-debugger`, and also to show some default configurations that users can template to enable `probe-rs-debugger` accross different hardware platforms/architectures.
- `stm32h745`: A sample application for Cortex-M7 on a ST Nucleo H745ZI-Q board
- `stm32h745-defmt-rtt`: Similar to above, except it uses `defmt` for RTT
- `nrf52833`: A sample applciation for Cortex-M4 on a Micro:Bit v2 board
- `esp32c3` : A sample configuration for RISCV platform

To customize or create additional application, simply copy the contents of a similar architecture, and look for the `TODO:` comment tag in all the files to identify the changes needed to support a specific architecture.

## Usage notes: 
- **VSCode debug extension** for `probe-rs-debugger`: Look at the `.vscode/launch.json` file for typical setup values.
- **Command line (CLI)** invocation of `probe-rs-debugger`: Look at the `.cargo/config.toml` file for typical setup values to enable `probe-rs-debugger` as the runner for `cargo run`

## TODO:
The subfolders contain a lot of duplicated code. I am open to suggestions on how to share more code between the various targets without tripping up `rust-analyzer`.