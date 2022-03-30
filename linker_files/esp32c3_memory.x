MEMORY
{
    /*
        https://github.com/espressif/esptool/blob/master/esptool.py#L1919
        MEMORY_MAP = [[0x00000000, 0x00010000, "PADDING"],
                  [0x3C000000, 0x3C800000, "DROM"],
                  [0x3FC80000, 0x3FCE0000, "DRAM"],
                  [0x3FC88000, 0x3FD00000, "BYTE_ACCESSIBLE"],
                  [0x3FF00000, 0x3FF20000, "DROM_MASK"],
                  [0x40000000, 0x40060000, "IROM_MASK"],
                  [0x42000000, 0x42800000, "IROM"],
                  [0x4037C000, 0x403E0000, "IRAM"],
                  [0x50000000, 0x50002000, "RTC_IRAM"],
                  [0x50000000, 0x50002000, "RTC_DRAM"],
                  [0x600FE000, 0x60100000, "MEM_INTERNAL2"]]
    */
    /* 400K of on soc RAM, 16K reserved for cache */
    ICACHE : ORIGIN = 0x4037C000,  LENGTH = 0x4000
    /* Instruction RAM */
    IRAM : ORIGIN = 0x4037C000 + 0x4000, LENGTH = 400K - 0x4000
    /* Data RAM */
    DRAM : ORIGIN = 0x3FC80000, LENGTH = 0x50000
    

    /* External flash */
    /* Instruction ROM */
    IROM : ORIGIN =   0x42000000, LENGTH = 0x400000
    /* Data ROM */
    DROM : ORIGIN = 0x3C000000, LENGTH = 0x400000
}


REGION_ALIAS("REGION_TEXT", IROM);
REGION_ALIAS("REGION_RODATA", DROM);

REGION_ALIAS("REGION_DATA", DRAM);
REGION_ALIAS("REGION_BSS", DRAM);
REGION_ALIAS("REGION_HEAP", DRAM);
REGION_ALIAS("REGION_STACK", DRAM);

SECTIONS
{
  .header : AT(0)
  {
    LONG(0xaedb041d)
    LONG(0xaedb041d)
  } > IROM
}

_stext = ORIGIN(IROM) + 8;

PROVIDE(_stext = ORIGIN(REGION_TEXT));
PROVIDE(_stack_start = ORIGIN(REGION_STACK) + LENGTH(REGION_STACK));
PROVIDE(_max_hart_id = 0);
PROVIDE(_hart_stack_size = 2K);
PROVIDE(_heap_size = 0);

PROVIDE(UserSoft = DefaultHandler);
PROVIDE(SupervisorSoft = DefaultHandler);
PROVIDE(MachineSoft = DefaultHandler);
PROVIDE(UserTimer = DefaultHandler);
PROVIDE(SupervisorTimer = DefaultHandler);
PROVIDE(MachineTimer = DefaultHandler);
PROVIDE(UserExternal = DefaultHandler);
PROVIDE(SupervisorExternal = DefaultHandler);
PROVIDE(MachineExternal = DefaultHandler);

PROVIDE(DefaultHandler = DefaultInterruptHandler);
PROVIDE(ExceptionHandler = DefaultExceptionHandler);

/* # Pre-initialization function */
/* If the user overrides this using the `#[pre_init]` attribute or by creating a `__pre_init` function,
   then the function this points to will be called before the RAM is initialized. */
PROVIDE(__pre_init = default_pre_init);

/* A PAC/HAL defined routine that should initialize custom interrupt controller if needed. */
PROVIDE(_setup_interrupts = default_setup_interrupts);

/* # Multi-processing hook function
   fn _mp_hook() -> bool;

   This function is called from all the harts and must return true only for one hart,
   which will perform memory initialization. For other harts it must return false
   and implement wake-up in platform-dependent way (e.g. after waiting for a user interrupt).
*/
PROVIDE(_mp_hook = default_mp_hook);

/* # Start trap function override
  By default uses the riscv crates default trap handler
  but by providing the `_start_trap` symbol external crates can override.
*/
PROVIDE(_start_trap = default_start_trap);

SECTIONS
{
  .text.dummy (NOLOAD) :
  {
    /* This section is intended to make _stext address work */
    . = ABSOLUTE(_stext);
  } > REGION_TEXT

  .text _stext :
  {
    /* Put reset handler first in .text section so it ends up as the entry */
    /* point of the program. */
    KEEP(*(.init));
    KEEP(*(.init.rust));
    . = ALIGN(4);
    (*(.trap));
    (*(.trap.rust));

    *(.text .text.*);
    _etext = .;
  } > REGION_TEXT

  _text_size = _etext - _stext + 8;
  .rodata ORIGIN(DROM) + _text_size : AT(_text_size)
  {
    _srodata = .;
    *(.srodata .srodata.*);
    *(.rodata .rodata.*);

    /* 4-byte align the end (VMA) of this section.
       This is required by LLD to ensure the LMA of the following .data
       section will have the correct alignment. */
    . = ALIGN(4);
    _erodata = .;
  } > REGION_RODATA

  _rodata_size = _erodata - _srodata + 8;
  .data ORIGIN(DRAM) : AT(_text_size + _rodata_size)
  {
    _sidata = LOADADDR(.data);
    _sdata = .;
    /* Must be called __global_pointer$ for linker relaxations to work. */
    PROVIDE(__global_pointer$ = . + 0x800);
    *(.sdata .sdata.* .sdata2 .sdata2.*);
    *(.data .data.*);
    . = ALIGN(4);
    _edata = .;
  } > REGION_DATA

  .bss (NOLOAD) :
  {
    _sbss = .;
    *(.sbss .sbss.* .bss .bss.*);
    . = ALIGN(4);
    _ebss = .;
  } > REGION_BSS

  /* fictitious region that represents the memory available for the heap */
  .heap (NOLOAD) :
  {
    _sheap = .;
    . += _heap_size;
    . = ALIGN(4);
    _eheap = .;
  } > REGION_HEAP

  /* fictitious region that represents the memory available for the stack */
  .stack (NOLOAD) :
  {
    _estack = .;
    . = ABSOLUTE(_stack_start);
    _sstack = .;
  } > REGION_STACK

  /* fake output .got section */
  /* Dynamic relocations are unsupported. This section is only used to detect
     relocatable code in the input files and raise an error if relocatable code
     is found */
  .got (INFO) :
  {
    KEEP(*(.got .got.*));
  }

  .eh_frame (INFO) : { KEEP(*(.eh_frame)) }
  .eh_frame_hdr (INFO) : { *(.eh_frame_hdr) }
}

/* Do not exceed this mark in the error messages above                                    | */
ASSERT(ORIGIN(REGION_TEXT) % 4 == 0, "
ERROR(riscv-rt): the start of the REGION_TEXT must be 4-byte aligned");

ASSERT(ORIGIN(REGION_RODATA) % 4 == 0, "
ERROR(riscv-rt): the start of the REGION_RODATA must be 4-byte aligned");

ASSERT(ORIGIN(REGION_DATA) % 4 == 0, "
ERROR(riscv-rt): the start of the REGION_DATA must be 4-byte aligned");

ASSERT(ORIGIN(REGION_HEAP) % 4 == 0, "
ERROR(riscv-rt): the start of the REGION_HEAP must be 4-byte aligned");

ASSERT(ORIGIN(REGION_TEXT) % 4 == 0, "
ERROR(riscv-rt): the start of the REGION_TEXT must be 4-byte aligned");

ASSERT(ORIGIN(REGION_STACK) % 4 == 0, "
ERROR(riscv-rt): the start of the REGION_STACK must be 4-byte aligned");

ASSERT(_stext % 4 == 0, "
ERROR(riscv-rt): `_stext` must be 4-byte aligned");

ASSERT(_sdata % 4 == 0 && _edata % 4 == 0, "
BUG(riscv-rt): .data is not 4-byte aligned");

ASSERT(_sidata % 4 == 0, "
BUG(riscv-rt): the LMA of .data is not 4-byte aligned");

ASSERT(_sbss % 4 == 0 && _ebss % 4 == 0, "
BUG(riscv-rt): .bss is not 4-byte aligned");

ASSERT(_sheap % 4 == 0, "
BUG(riscv-rt): start of .heap is not 4-byte aligned");

ASSERT(_stext + SIZEOF(.text) < ORIGIN(REGION_TEXT) + LENGTH(REGION_TEXT), "
ERROR(riscv-rt): The .text section must be placed inside the REGION_TEXT region.
Set _stext to an address smaller than 'ORIGIN(REGION_TEXT) + LENGTH(REGION_TEXT)'");

ASSERT(SIZEOF(.stack) > (_max_hart_id + 1) * _hart_stack_size, "
ERROR(riscv-rt): .stack section is too small for allocating stacks for all the harts.
Consider changing `_max_hart_id` or `_hart_stack_size`.");

ASSERT(SIZEOF(.got) == 0, "
.got section detected in the input files. Dynamic relocations are not
supported. If you are linking to C code compiled using the `gcc` crate
then modify your build script to compile the C code _without_ the
-fPIC flag. See the documentation of the `gcc::Config.fpic` method for
details.");

/* Do not exceed this mark in the error messages above                                    | */

uart_tx_one_char = 0x40000068;
ets_delay_us = 0x40000050;