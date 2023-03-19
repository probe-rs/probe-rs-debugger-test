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

use nrf52833_hal::{
    clocks::Clocks, gpio, pac::CorePeripherals, pac::Peripherals, prelude::*, Delay,
};
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
    let pac = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let clocks = Clocks::new(pac.CLOCK).enable_ext_hfosc();
    let pins = gpio::p0::Parts::new(pac.P0);
    // To light an LED, set the Row bit, and clear the Col bit.
    // Col 1 =  p0_28, Row 1 = p0_21
    let _ = pins.p0_28.into_push_pull_output(gpio::Level::Low).degrade();

    let mut delay = Delay::new(core.SYST);
    let mut led_pin = pins
        .p0_21
        .into_push_pull_output(gpio::Level::High)
        .degrade();

    loop {
        // Common testing code.
        shared_loop_processing(&mut binary_rtt_channel, &mut loop_counter);

        // Board/Chip specific code.
        if led_pin.is_set_high().unwrap() {
            led_pin.set_low()
        } else {
            led_pin.set_high()
        }
        .unwrap();
        delay.delay_ms(20_u32);
    }
}
