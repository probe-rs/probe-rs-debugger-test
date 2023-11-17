#!/usr/bin/env bash
TARGET="riscv32imac-unknown-none-elf"
FEATURE="esp32c3"

# cargo install probe-rs --features cli --git https://github.com/probe-rs/probe-rs --branch debug_tests_for_riscv --force

set -ex
cargo size --target $TARGET --features ${FEATURE} --bin ${FEATURE} --profile "debug-no-opt" -- -A -x  && \
probe-rs download --chip ${FEATURE}  --format "idf" ./target/${TARGET}/debug-no-opt/${FEATURE} && \
set +x && \
echo Once the program is running, try "reset" followed by "run" followed by "status" && \
echo The response should be  && \
echo      Status Halted\(Breakpoint\(Software\)\) && \
echo      Core halted at address 0x4200a738 && \
echo Then enter the following two commands one at a time ... && \
echo dump 0x3FC80000 0x18010 single_read_bad.coredump && \
echo dump 0x3FC80000 0x18000 0x3FC98000 0x10 two_part_read_ok.coredump && \
set -x && \
probe-rs debug --chip ${FEATURE} --exe ./target/${TARGET}/debug-no-opt/${FEATURE}

