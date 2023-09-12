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
use cortex_m_rt::{entry, exception, ExceptionFrame};

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};
use embedded_hal::digital::v2::ToggleableOutputPin;
use rp_pico as bsp;

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    rprintln!("HardFault at {:#?}", ef);
    unsafe {
        core::arch::asm!("bkpt");
    }
    loop {}
}

// #[panic_handler]
// fn panic(info: &core::panic::PanicInfo) -> ! {
//     rprintln!("Panic: {:?}", info);
//     loop {
//         unsafe {
//             core::arch::asm!("bkpt");
//         }
//         rprintln!("In a panic loop, stepped past the breakpoint");
//     }
// }

#[entry]
fn main() -> ! {
    // Common testing code.
    let (mut loop_counter, mut binary_rtt_channel) = setup_data_types();
    test_deep_stack(0);

    // Board/Chip specific code.
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);
    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    let mut delay = cortex_m_mc::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let mut led_pin = pins.led.into_push_pull_output();

    loop {
        // Common testing code.
        shared_loop_processing(&mut binary_rtt_channel, &mut loop_counter);

        // Board/Chip specific code.
        led_pin.toggle().ok();
        delay.delay_ms(1000_u32);

        // Cause a hardfault, by reading from an invalid address.
        unsafe {
            core::ptr::read_volatile(0x3FFF_FFFE as *const u32);
        }
    }
}

use core::panic::PanicInfo;

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        rtt_target::rprintln!("going to udf");
        cortex_m_mc::asm::udf();
    }
}
