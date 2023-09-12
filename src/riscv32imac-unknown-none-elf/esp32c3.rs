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
    // Common testing code.
    let (mut loop_counter, mut binary_rtt_channel) = setup_data_types();
    test_deep_stack(0);

    // Board/Chip specific code.
    let peripherals = peripherals::Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // To ensure we can step through breakpoints/code, we have to disable the watchdog timers.
    // For the ESP32-C3, this includes the Super WDT, the RTC WDT, and the TIMG WDTs.
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(
        peripherals.TIMG1,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt1 = timer_group1.wdt;
    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    // Initialize the Delay peripheral.
    let mut delay = Delay::new(&clocks);

    loop {
        // Common testing code.
        shared_loop_processing(&mut binary_rtt_channel, &mut loop_counter);

        // Board/Chip specific code.
        delay.delay_ms(20u8);
    }
}
