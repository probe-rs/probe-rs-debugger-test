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
        // Cause a hardfault, by reading from an invalid address.
        unsafe {
            core::ptr::read_volatile(0x3FFF_FFFE as *const u32);
        }
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

use nrf52833_hal::{clocks::Clocks, pac::CorePeripherals, pac::Peripherals};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    cortex_m::asm::udf();
}

#[entry]
fn main() -> ! {
    // Board/Chip specific code.
    let pac = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let clocks = Clocks::new(pac.CLOCK).enable_ext_hfosc();

    // configures the system timer to trigger a SysTick exception every second
    let mut syst = core.SYST;
    syst.set_clock_source(SystClkSource::Core);
    // this is configured for the NRF52833 which has a default CPU clock of 64 MHz
    syst.set_reload(64_000_000);
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();

    // Common testing code.
    let (mut loop_counter, mut binary_rtt_channel) = setup_data_types();

    loop {
        // Common testing code.
        shared_loop_processing(&mut binary_rtt_channel, &mut loop_counter);

        // Board/Chip specific code.

        if loop_counter.0 > 3 {
            panic!("Loop counter exceeded 3")
        }
        // Board/Chip specific code.
        cortex_m::asm::delay(1_000_000);
    }
}
