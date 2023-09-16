#![no_std]
#![no_main]
// We want our binary names the same as the chipname.
#![allow(non_snake_case)]
// Because this is not code that doesn't need to serve a purpose other than testing the debugger
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]

// This section contains the declarations that is common to all the tests, and is not specific to any one chip/board.
use common_testing_code::*;
use rtt_target::rprintln;

// Board/Chip specific code.
use cortex_m_rt::entry;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    rprintln!("Panic: {:?}", info);
    loop {
        unsafe {
            core::arch::asm!("bkpt");
        }
        rprintln!("In a panic loop, stepped past the breakpoint");
    }
}

#[entry]
fn main() -> ! {
    // Common testing code.
    let (mut loop_counter, mut binary_rtt_channel) = setup_data_types();
    test_deep_stack(0);

    // Board/Chip specific code.

    loop {
        // Common testing code.
        shared_loop_processing(&mut binary_rtt_channel, &mut loop_counter);

        // Board/Chip specific code.
        cortex_m::asm::delay(1_000_000);
    }
}
