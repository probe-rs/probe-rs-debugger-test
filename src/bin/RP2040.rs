#![no_std]
#![no_main]
// We want our binary names the same as the chipname.
#![allow(non_snake_case)]
// Because this is not code that doesn't need to serve a purpose other than testing the debugger
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use cortex_m::{peripheral::syst::SystClkSource, Peripherals};
// This section contains the declarations that is common to all the tests, and is not specific to any one chip/board.
use probe_rs_debugger_test::*;

// Board/Chip specific code.
use cortex_m_rt::{entry, exception};

use bsp::hal::{clocks::init_clocks_and_plls, pac, watchdog::Watchdog};

use rp_pico as bsp;

#[entry]
fn main() -> ! {
    // Board/Chip specific code.
    let core = pac::CorePeripherals::take().unwrap();
    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

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

    #[cfg(feature = "systick")]
    enable_systick(core);

    #[cfg(feature = "hardfault_from_usagefault")]
    trigger_hardfault_from_usagefault();

    #[cfg(feature = "hardfault_from_busfault")]
    trigger_hardfault_from_busfault();

    #[cfg(feature = "svcall")]
    trigger_supervisor_call();

    // Common testing code.
    let (mut loop_counter, mut binary_rtt_channel) = setup_data_types();

    loop {
        // Common testing code.
        shared_loop_processing(&mut binary_rtt_channel, &mut loop_counter);
    }
}

/// Cause an escalated hardfault, by reading from an invalid address.
fn trigger_hardfault_from_busfault() {
    unsafe {
        core::ptr::read_volatile(0x3FFF_FFFC as *const u32);
    }
}

/// Cause a UDF, by reading executing an invalid instruction.
fn trigger_hardfault_from_usagefault() {
    cortex_m::asm::udf();
}

/// Cause a SVC, by executing an SVC instruction.
fn trigger_supervisor_call() {
    unsafe {
        core::arch::asm!("svc 0");
    }
}

/// Configures the system timer to trigger a SysTick exception every second
fn enable_systick(core: Peripherals) {
    let mut syst = core.SYST;
    syst.set_clock_source(SystClkSource::Core);
    // this is configured for the RP2040 with external crystal at 12 MHz
    syst.set_reload(12_000_000);
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();
    // Hang out here for long enough to let the Systick interrupt fire
    loop {
        cortex_m::asm::delay(1_000_000);
    }
}

#[exception]
fn SysTick() {
    #[cfg(feature = "hardfault_from_busfault")]
    trigger_hardfault_from_busfault();
    software_breakpoint();
}

// `cortex-m-rt` has a default trampoline, that overwrites the ARM EXC_RETURN value,
// and makes it impossible for the debugger to identify an exception frame,
// and to identify where it was called from. Disable with `trampoline = false`.
#[exception(trampoline = false)]
unsafe fn HardFault() -> ! {
    loop {
        software_breakpoint();
    }
}

#[exception]
fn SVCall() {
    software_breakpoint();
}
