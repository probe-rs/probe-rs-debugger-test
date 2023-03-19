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
use esp32c3_hal::{
    clock::ClockControl,
    peripherals::Peripherals,
    prelude::*,
    pulse_control::ClockSource,
    timer::TimerGroup,
    utils::{smartLedAdapter, SmartLedsAdapter},
    Delay, PulseControl, Rtc, IO,
};
use smart_leds::{
    brightness, gamma,
    hsv::{hsv2rgb, Hsv},
    SmartLedsWrite,
};
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
    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    // Disable the watchdog timers. For the ESP32-C3, this includes the Super WDT,
    // the RTC WDT, and the TIMG WDTs.
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;
    // Disable watchdog timers
    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    //This is more complex than other boards because of the LED installed on this board.
    // Configure RMT peripheral globally
    let pulse = PulseControl::new(
        peripherals.RMT,
        &mut system.peripheral_clock_control,
        ClockSource::APB,
        0,
        0,
        0,
    )
    .unwrap();
    // We use one of the RMT channels to instantiate a `SmartLedsAdapter` which can
    // be used directly with all `smart_led` implementations
    let mut led = <smartLedAdapter!(1)>::new(pulse.channel0, io.pins.gpio8);
    // Initialize the Delay peripheral, and use it to toggle the LED state in a
    // loop.
    let mut delay = Delay::new(&clocks);
    let mut color = Hsv {
        hue: 255,
        sat: 255,
        val: 255,
    };
    let mut data = [hsv2rgb(color)];

    loop {
        // Common testing code.
        shared_loop_processing(&mut binary_rtt_channel, &mut loop_counter);

        // Board/Chip specific code.
        color.hue = loop_counter.0;
        // Convert from the HSV color space (where we can easily transition from one
        // color to the other) to the RGB color space that we can then send to the LED
        data = [hsv2rgb(color)];
        // When sending to the LED, we do a gamma correction first (see smart_leds
        // documentation for details) and then limit the brightness to 10 out of 255 so
        // that the output it's not too bright.
        led.write(brightness(gamma(data.iter().cloned()), 10))
            .unwrap();
        delay.delay_ms(20u8);
    }
}
