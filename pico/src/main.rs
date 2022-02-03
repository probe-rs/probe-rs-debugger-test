#![no_std]
#![no_main]
// Because this is not code that doesn't need to serve a purpose other than testing the debugger
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use heapless::Vec;
use panic_probe as _;
use rtt_target::{rprintln, rtt_init_print};

// TODO: Code specific to Cortex-M
// use cortex_m::asm;
use cortex_m_rt::entry;

// Raspberry PICO board
use embedded_hal::digital::v2::OutputPin;
use embedded_time::fixed_point::FixedPoint;

use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

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

static GLOBAL_STATIC: &str = "A 'global' static variable";
const GLOBAL_CONSTANT: &str =
    "This value will only show up in the debugger in the variables where it is referenced";
use self::ComplexEnum::{Case1, Case2};
use self::Univariant::TupleOfComplexStruct;

struct RecursiveStruct<'a> {
    depth: u32,
    next_self: Option<&'a mut RecursiveStruct<'a>>,
}

struct ComplexStruct {
    x: i64,
    y: i32,
    z: i16,
}

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
static REGULAR_STRUCT: ComplexEnum = Case1(
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

#[entry]
fn main() -> ! {
    static mut LOCAL_STATIC: &str = "A 'local' to main() static variable";
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
    let second_case_of_struct_variants = Case2(0, 1023, 1967);
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

    rtt_init_print!();
    // TODO: Code specific to this board - BEGIN
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

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.led.into_push_pull_output();
    // TODO: Code specific to this board - END
    loop {
        rprintln!("on!");
        led_pin.set_high().unwrap();
        delay.delay_ms(500);
        rprintln!("off!");
        led_pin.set_low().unwrap();
        delay.delay_ms(500);
    }
}
