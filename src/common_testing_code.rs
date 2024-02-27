#![no_std]
#![no_main]
// Because this is not code that doesn't need to serve a purpose other than testing the debugger
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use core::num::Wrapping;
use core::usize;
use heapless::Vec;
use rtt_target::{rprintln, rtt_init, set_print_channel, ChannelMode::NoBlockTrim};

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
    "This global `const` value will only show up in the debugger in the variables where it is referenced";
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

#[derive(core::cmp::PartialEq, Debug)]
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

    let mut f32_val: f32 = 2.5;
    let f32_ref: &f32 = &f32_val;
    f32_val = 3.5;

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

pub struct Matrix<T, const X: usize, const Y: usize, const Z: usize> {
    pub raw: [[[T; Y]; X]; Z],
}

fn assoc_struct<T: TraitWithAssocType>(arg: Struct<T>) -> Struct<T> {
    arg
}

#[inline(always)]
fn assoc_local<T: TraitWithAssocType>(x: T) -> T::Type {
    let inferred = x.get_value();
    let explicitly: T::Type = x.get_value();
    inferred
}

fn assoc_arg<T: TraitWithAssocType>(arg: T::Type) -> T::Type {
    arg
}

fn assoc_return_value<T: TraitWithAssocType>(arg: T) -> T::Type {
    arg.get_value()
}

fn assoc_tuple<T: TraitWithAssocType>(arg: (T, T::Type)) -> (T, T::Type) {
    arg
}

fn assoc_enum<T: TraitWithAssocType>(arg: Enum<T>) -> Enum<T> {
    match arg {
        Enum::Variant1(a, b) => Enum::Variant2(b, a),
        Enum::Variant2(a, b) => Enum::Variant1(b, a),
    }
}

#[inline(never)]
fn create_complex_struct() -> ComplexStruct {
    rprintln!("Creating a new complex struct");
    ComplexStruct { x: 10, y: 8, z: 4 }
}

#[inline(never)]
fn create_short_lived(copy_me_from: ComplexStruct) -> ComplexStruct {
    let mut change_me = copy_me_from;
    change_me.x *= 2;
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

#[inline(never)]
pub fn setup_data_types() -> (Wrapping<u8>, rtt_target::UpChannel) {
    let int8_minus_twenty_three: i8 = -23;
    static mut LOCAL_STATIC: &str =
        "A 'local' to main() static variable ...will be optimized out if not used in the code.";
    let local_reference_to_global_const = GLOBAL_CONSTANT;
    let local_reference_to_global_static = unsafe { GLOBAL_STATIC };
    let local_reference_to_global_static_struct = unsafe { &REGULAR_STRUCT };
    let ghosted_variable = 0_usize;
    let ghosted_variable = "New value and type for a different name";
    let int8_twenty_six: i8 = 26;
    let int128: i128 = -196710231994021419720322;
    let u_int128: u128 = int128 as u128;
    let float64: f64 = 56.7 / 32.2; //1.760869565217391
    let float64_ptr = &float64;
    let emoji = '💩';
    let emoji_ptr = &emoji;
    let mut true_bool = false;
    true_bool = true;
    let any_old_string_slice = "How long is a piece of String.";
    let function_result = basic_types_with_err_result();
    let global_types = unsafe { (B, I, C, I8, I16, I32, I64, U, U8, U16, U32, U64, F32, F64) };
    let three_d_usize_array = Matrix {
        raw: [
            [[0, 1, 2], [3, 4, 5]],
            [[6, 7, 8], [9, 10, 11]],
            [[12, 13, 14], [15, 16, 17]],
            [[18, 19, 20], [21, 22, 23]],
        ],
    };
    let three_d_string_array = Matrix {
        raw: [
            [["Apple", "Banana", "Cherry"], ["Dog", "Elephant", "Fish"]],
            [
                ["Guitar", "Horse", "Ice Cream"],
                ["Jaguar", "Kangaroo", "Lion"],
            ],
            [["Moon", "Newton", "Owl"], ["Pencil", "Queen", "Rainbow"]],
            [
                ["Sun", "Tree", "Umbrella"],
                ["Violin", "Watch", "Xylophone"],
            ],
            [["Yellow", "Zebra", "Alpha"], ["Bravo", "Charlie", "Delta"]],
            [["Echo", "Foxtrot", "Golf"], ["Hotel", "India", "Juliet"]],
        ],
    };

    let three = SimpleEnum::Two;
    let simple_enum_pointer = &three;

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
    let stuct_with_one_variant_pointer = &struct_with_one_variant;

    let long_lived = ComplexStruct { x: 10, y: 8, z: 4 };
    // The next line has two step-in targets. The first is the `create_complex_struct` function. The second is the `create_short_lived` function.
    let short_lived = create_short_lived(create_complex_struct());
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
    let loop_counter = Wrapping(0u8);
    let rtt_channels = rtt_init! { up: {
            0: {
                size: 1024,
                mode: NoBlockTrim,  // NoBlockSkip //BlockIfFull
                name: "String RTT Channel"
            }
            1: {
                size: 1024,
                mode: NoBlockTrim,
                name: "BinaryLE RTT Channel"
            }
        }
    };
    set_print_channel(rtt_channels.up.0);
    rprintln!("Forcing use of : {:?}", simple_enum_pointer);
    unsafe {
        rprintln!("Forcing use of :{}", LOCAL_STATIC);
    }

    test_deep_stack(0);

    (loop_counter, rtt_channels.up.1)
}

#[inline(never)]
pub fn test_deep_stack(stack_depth: usize) {
    let internal_depth_measure = stack_depth + 1;
    rprintln!(
        "Recursive call # {} in `test_deep_stack`",
        internal_depth_measure
    );
    if internal_depth_measure <= 5 {
        test_deep_stack(internal_depth_measure);
        rprintln!("Returning from call # {} ", internal_depth_measure);
    } else {
        // We force a software breakpoint here, so that our tests can rely on the target state being stopped exactly here.
        #[cfg(not(feature = "esp32c3"))]
        unsafe {
            core::arch::asm!("bkpt");
        }
        #[cfg(feature = "esp32c3")]
        unsafe {
            core::arch::asm!("ebreak");
        }
        rprintln!("Dropping out of the deep recursive stack test");
    }
}

#[inline(always)]
pub fn shared_loop_processing(
    binary_rtt_channel: &mut rtt_target::UpChannel,
    loop_counter: &mut core::num::Wrapping<u8>,
) {
    let bytes_written = binary_rtt_channel.write(&u8::to_le_bytes(loop_counter.0));
    rprintln!(
        "Loop count # {}, wrote {}  bytes to the BinaryLE channel #1",
        loop_counter.0,
        bytes_written,
    );
    *loop_counter += Wrapping(1u8);
}
