#![no_std]
#![no_main]
// Because this is not code that doesn't need to serve a purpose other than testing the debugger
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use core::num::Wrapping;
use core::usize;

use heapless::Vec;
use rtt_target::{rprintln, rtt_init, set_print_channel};

#[cfg(any(feature = "STM32H745ZITx", feature = "nRF52833_xxAA"))]
use cortex_m::asm;

#[cfg(any(
    feature = "STM32H745ZITx",
    feature = "RP2040",
    feature = "nRF52833_xxAA"
))]
#[cfg(any(
    feature = "STM32H745ZITx",
    feature = "RP2040",
    feature = "nRF52833_xxAA"
))]
use cortex_m_rt::entry;
#[cfg(any(
    feature = "STM32H745ZITx",
    feature = "RP2040",
    feature = "nRF52833_xxAA"
))]
use panic_probe as _;

#[cfg(feature = "nRF52833_xxAA")]
use nrf52833_hal::{
    clocks::Clocks, gpio, pac::CorePeripherals, pac::Peripherals, prelude::*, Delay,
};

#[cfg(feature = "esp32c3")]
use esp32c3_hal::{gpio::IO, pac::Peripherals, prelude::*, Delay, RtcCntl, Timer};
#[cfg(feature = "esp32c3")]
use panic_halt as _;
// #[cfg(feature = "esp32c3")]
// use riscv_atomic_emulation_trap as _;
#[cfg(feature = "esp32c3")]
use riscv_rt::entry;

#[cfg(feature = "RP2040")]
use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};
#[cfg(feature = "RP2040")]
use embedded_hal::digital::v2::ToggleableOutputPin;
#[cfg(feature = "RP2040")]
use embedded_time::fixed_point::FixedPoint;
#[cfg(feature = "RP2040")]
use rp_pico as bsp;

// Definitions for testing debugger with various datatypes
// N.B. These are `mut` only so they don't constant fold away.
static mut B: bool = false;
static mut I: isize = -1;
static mut C: char = 'a';
static mut I8: i8 = 68;
static mut I16: i16 = -16;
static mut I32: i32 = -32;
static mut I64: i64 = -64;
static mut U: usize = 1;
static mut U8: u8 = 100;
static mut U16: u16 = 16;
static mut U32: u32 = 32;
static mut U64: u64 = 64;
static mut F32: f32 = 2.5;
static mut F64: f64 = 3.5;

static mut GLOBAL_STATIC: &str = "A 'global' static variable";
const GLOBAL_CONSTANT: &str =
    "This value will only show up in the debugger in the variables where it is referenced";
use self::ComplexEnum::{Case1, Case2};
use self::Univariant::TupleOfComplexStruct;

struct RecursiveStruct<'a> {
    depth: u32,
    next_self: Option<&'a mut RecursiveStruct<'a>>,
}

#[derive(Debug)]
struct ComplexStruct {
    x: i64,
    y: i32,
    z: i16,
}

#[derive(core::cmp::PartialEq)]
enum SimpleEnum {
    One,
    Two,
    Three,
    Four,
    Five,
}
enum ComplexEnum {
    Case1(u64, ComplexStruct),
    Case2(u64, u64, i16),
}
enum Univariant {
    TupleOfComplexStruct(ComplexStruct, ComplexStruct),
}

static mut REGULAR_STRUCT: ComplexEnum = Case1(
    0,
    ComplexStruct {
        x: 24,
        y: 25,
        z: 26,
    },
);

#[inline(always)]
fn basic_types_with_err_result() -> Result<(), &'static str> {
    let bool_val: bool = true;
    let bool_ref: &bool = &bool_val;

    let int_val: isize = -1;
    let int_ref: &isize = &int_val;

    let char_val: char = 'a';
    let char_ref: &char = &char_val;

    let i8_val: i8 = 68;
    let i8_ref: &i8 = &i8_val;

    let i16_val: i16 = -16;
    let i16_ref: &i16 = &i16_val;

    let i32_val: i32 = -32;
    let i32_ref: &i32 = &i32_val;

    let i64_val: i64 = -64;
    let i64_ref: &i64 = &i64_val;

    let uint_val: usize = 1;
    let uint_ref: &usize = &uint_val;

    let u8_val: u8 = 100;
    let u8_ref: &u8 = &u8_val;

    let u16_val: u16 = 16;
    let u16_ref: &u16 = &u16_val;

    let u32_val: u32 = 32;
    let u32_ref: &u32 = &u32_val;

    let u64_val: u64 = 64;
    let u64_ref: &u64 = &u64_val;

    let f32_val: f32 = 2.5;
    let f32_ref: &f32 = &f32_val;

    let f64_val: f64 = 3.5;
    let f64_ref: &f64 = &f64_val;

    let nested_inline_function = assoc_local(2);

    Err("Forcing the return of an Error variant")
}
trait TraitWithAssocType {
    type Type;

    fn get_value(&self) -> Self::Type;
}
impl TraitWithAssocType for i32 {
    type Type = i64;

    fn get_value(&self) -> i64 {
        *self as i64
    }
}

struct Struct<T: TraitWithAssocType> {
    b: T,
    b1: T::Type,
}

enum Enum<T: TraitWithAssocType> {
    Variant1(T, T::Type),
    Variant2(T::Type, T),
}

fn assoc_struct<T: TraitWithAssocType>(arg: Struct<T>) {}

#[inline(always)]
fn assoc_local<T: TraitWithAssocType>(x: T) {
    let inferred = x.get_value();
    let explicitly: T::Type = x.get_value();
}

fn assoc_arg<T: TraitWithAssocType>(arg: T::Type) {}

fn assoc_return_value<T: TraitWithAssocType>(arg: T) -> T::Type {
    arg.get_value()
}

fn assoc_tuple<T: TraitWithAssocType>(arg: (T, T::Type)) {}

fn assoc_enum<T: TraitWithAssocType>(arg: Enum<T>) {
    match arg {
        Enum::Variant1(a, b) => {}
        Enum::Variant2(a, b) => {}
    }
}

fn create_short_lived(copy_me_from: &mut ComplexStruct) -> ComplexStruct {
    let mut change_me = copy_me_from;
    change_me.x *= 2;
    rprintln!(
        "Our incoming arg has a LocationList to compute memory location : {:?}",
        change_me
    );
    ComplexStruct {
        x: change_me.x,
        y: change_me.y,
        z: change_me.z,
    }
}

fn consume_enum_parameter(complex_enum: ComplexEnum) -> ComplexEnum {
    let mut consume_argument = complex_enum;
    consume_argument = Case2(0, 1023, 1967);
    consume_argument
}

#[entry]
fn main() -> ! {
    static mut LOCAL_STATIC: &str = "A 'local' to main() static variable";
    let ghosted_variable = 0_usize;
    let ghosted_variable = "New value and type for a different name";
    let int8: i8 = 23;
    let int128: i128 = -196710231994021419720322;
    let u_int128: u128 = 196710231994021419720322;
    let float64: f64 = 56.7 / 32.2; //1.760869565217391
    let float64_ptr = &float64;
    let emoji = '💩';
    let emoji_ptr = &emoji;
    let mut true_bool = false;
    true_bool = true;
    let any_old_string_slice = "How long is a piece of String.";
    let function_result = basic_types_with_err_result();
    let global_types = unsafe { (B, I, C, I8, I16, I32, I64, U, U8, U16, U32, U64, F32, F64) };
    let three = SimpleEnum::Three;

    let three_level_recursive_struct = RecursiveStruct {
        depth: 1,
        next_self: Some(&mut RecursiveStruct {
            depth: 2,
            next_self: Some(&mut RecursiveStruct {
                depth: 3,
                next_self: None,
            }),
        }),
    };

    let first_case_of_struct_variants = Case1(
        0,
        ComplexStruct {
            x: 24,
            y: 25,
            z: 26,
        },
    );
    let second_case_of_struct_variants = consume_enum_parameter(first_case_of_struct_variants);

    drop(second_case_of_struct_variants);

    let struct_with_one_variant = Some(TupleOfComplexStruct(
        ComplexStruct {
            x: 24,
            y: 25,
            z: 26,
        },
        ComplexStruct {
            x: -3,
            y: -2,
            z: -1,
        },
    ));

    let mut long_lived = ComplexStruct { x: 10, y: 8, z: 4 };
    let short_lived = create_short_lived(&mut long_lived);
    let a1 = assoc_struct(Struct { b: -1, b1: 0 });
    let a2 = assoc_local(1);
    let a3 = assoc_arg::<i32>(2);
    let a4 = assoc_return_value(3);
    let a5 = assoc_tuple((4, 5));
    let a6 = assoc_enum(Enum::Variant1(6, 7));
    let a7 = assoc_enum(Enum::Variant2(8, 9));

    let my_array = [55; 10];
    let my_array_ptr = &my_array;
    let my_array_of_i8: [i8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let mut heapless_vec = Vec::<i8, 10>::new(); //Needs a Cargo.toml dependency for `heapless = "*"`
    heapless_vec.push(1).ok();
    heapless_vec.push(2).ok();
    heapless_vec.push(3).ok();
    let mut loop_counter = Wrapping(0u8);

    let rtt_channels = rtt_init! {
        up: {
            0: {
                size: 1024
                mode: BlockIfFull
                name: "String RTT Channel"
            }
            1: {
                size: 1024
                mode: BlockIfFull
                name: "BinaryLE RTT Channel"
            }
        }
    };
    // Setup to use rprintln to channel 0
    set_print_channel(rtt_channels.up.0);
    let mut binary_rtt_channel: rtt_target::UpChannel = rtt_channels.up.1;

    #[cfg(feature = "esp32c3")]
    let (mut delay, mut led_pin) = {
        let peripherals = Peripherals::take().unwrap();

        // Disable the watchdog timers. For the ESP32-C3, this includes the Super WDT,
        // the RTC WDT, and the TIMG WDTs.
        let mut rtc_cntl = RtcCntl::new(peripherals.RTC_CNTL);
        let mut timer0 = Timer::new(peripherals.TIMG0);
        let mut timer1 = Timer::new(peripherals.TIMG1);

        rtc_cntl.set_super_wdt_enable(false);
        rtc_cntl.set_wdt_enable(false);
        timer0.disable();
        timer1.disable();

        let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

        //TODO: The LED on gpio8 is "addressable" and won't flash with a simple on-off. The code in the main loop works, but doesn't flash the LED.

        (
            Delay::new(peripherals.SYSTIMER),
            io.pins.gpio8.into_push_pull_output(),
        )
    };

    #[cfg(feature = "RP2040")]
    let (mut delay, mut led_pin) = {
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

        (
            cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer()),
            pins.led.into_readable_output(),
        )
    };

    #[cfg(feature = "nRF52833_xxAA")]
    let (mut delay, mut led_pin) = {
        let pac = Peripherals::take().unwrap();
        let core = CorePeripherals::take().unwrap();
        let clocks = Clocks::new(pac.CLOCK).enable_ext_hfosc();
        let pins = gpio::p0::Parts::new(pac.P0);
        // To light an LED, set the Row bit, and clear the Col bit.
        // Col 1 =  p0_28, Row 1 = p0_21
        let _ = pins.p0_28.into_push_pull_output(gpio::Level::Low).degrade();

        (
            Delay::new(core.SYST),
            pins.p0_21
                .into_push_pull_output(gpio::Level::High)
                .degrade(),
        )
    };

    test_deep_stack(0);
    loop {
        loop_counter += Wrapping(1u8);
        let bytes_written = binary_rtt_channel.write(&u8::to_le_bytes(loop_counter.0)); // Raw byte level output to Channel 1
        rprintln!(
            "Loop count # {}, wrote {}  bytes to the BinaryLE channel #1",
            loop_counter,
            bytes_written
        ); // Text Output line on Channel 0

        #[cfg(feature = "nRF52833_xxAA")]
        {
            if led_pin.is_set_high().unwrap() {
                led_pin.set_low()
            } else {
                led_pin.set_high()
            }
            .unwrap();
            delay.delay_ms(250_u32);
        }

        #[cfg(any(feature = "RP2040", feature = "esp32c3"))]
        {
            led_pin.toggle().ok();
            delay.delay_ms(250_u32);
        }

        #[cfg(any(feature = "STM32H745ZITx", feature = "nRF52833_xxAA"))]
        asm::delay(10_000_000);
    }

    fn test_deep_stack(stack_depth: usize) {
        let internal_depth_measure = stack_depth + 1;
        rprintln!(
            "Recursive call # {} in `test_deep_stack`",
            internal_depth_measure
        );
        if internal_depth_measure <= 35 {
            test_deep_stack(internal_depth_measure);
        } else {
            rprintln!("Dropping out of the deep recursive stack test");
        }
    }
}
