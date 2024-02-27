#![no_std]
#![no_main]
// Because this is not code that doesn't need to serve a purpose other than testing the debugger
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]

// This section contains the declarations that is common to all the tests, and is not specific to any one chip/board.
use common_testing_code::*;
use rtt_target::rprintln;

// Board/Chip specific code.
use esp32c3_hal::{clock::ClockControl, peripherals, prelude::*, timer::TimerGroup, Delay, Rtc};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    rprintln!("Panic: {:?}", info);
    loop {
        unsafe {
            core::arch::asm!("ebreak");
        }
        rprintln!("In a panic loop, stepped past the breakpoint");
    }
}

#[entry]
fn main() -> ! {
    // Board/Chip specific code.
    let peripherals = peripherals::Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Initialize the Delay peripheral.
    let mut delay = Delay::new(&clocks);

    // Common testing code.
    let (mut loop_counter, mut binary_rtt_channel) = setup_data_types();

    loop {
        // Common testing code.
        shared_loop_processing(&mut binary_rtt_channel, &mut loop_counter);

        // Board/Chip specific code.
        delay.delay_ms(20u8);
    }
}
