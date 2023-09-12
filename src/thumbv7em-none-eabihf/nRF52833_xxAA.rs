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
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception, ExceptionFrame};

#[exception]
fn SysTick() {
    static mut COUNT: u32 = 0;

    *COUNT += 1;

    if *COUNT == 5 {
        // If `UsageFault` is enabled, we disable that first, since otherwise `udf` will cause that
        // exception instead of `HardFault`.
        // const SHCSR: *mut u32 = 0xE000ED24usize as _;
        // const USGFAULTENA: usize = 18;
        // unsafe {
        //     let mut shcsr = core::ptr::read_volatile(SHCSR);
        //     shcsr &= !(1 << USGFAULTENA);
        //     core::ptr::write_volatile(SHCSR, shcsr);
        // }
        // Cause a UDF, by reading executing an invalid instruction.
        // core::intrinsics::abort()

        // Cause a hardfault, by reading from an invalid address.
        // unsafe {
        //     core::ptr::read_volatile(0x3FFF_FFFE as *const u32);
        // }
    }
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    rprintln!("HardFault at {:#?}", ef);
    unsafe {
        core::arch::asm!("bkpt");
    }
    panic!("Exiting after HardFault");
}

use nrf52833_hal::{
    clocks::Clocks, gpio, pac::CorePeripherals, pac::Peripherals, prelude::*, Delay,
};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    cortex_m::asm::udf();
    // core::arch::asm(udf());
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

    // let mut delay = Delay::new(core.SYST);
    let mut syst = core.SYST;

    // configures the system timer to trigger a SysTick exception every second
    syst.set_clock_source(SystClkSource::Core);
    // this is configured for the NRF52833 which has a default CPU clock of 64 MHz
    syst.set_reload(64_000_000);
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();
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

        if loop_counter.0 > 3 {
            panic!("Loop counter exceeded 3")
        }
        // Board/Chip specific code.
        cortex_m::asm::delay(1_000_000);
        // delay.delay_ms(1000_u32);
    }
}
