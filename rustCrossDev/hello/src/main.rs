#![no_std]
#![feature(lang_items, start)]
#![no_main]

// Comments on the above.
// bare embedded:
// - does not use std library
// - implements its own language items e.g. panic_fmt
// - implements its own start function e.g. POR reset handler
// - has no main to be called by start function (instead POR reset jumps)


// crt0 written in Rust
extern crate r0;


// core requires 'language items' usually found in std.
// Since we omit std, we must provide.

// Core invokes panic_fmt() on `panic!`.
// Implement bare minimum: an infinite loop
#[lang = "panic_fmt"]
pub extern fn panic_fmt(_msg: core::fmt::Arguments,
                        _file: &'static str,
                        _line: u32) -> ! {
    loop {}
}

// Core invokes rust_eh_personality() on ???
// Exist compile option to obviate this?
#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {
}



// Entry point for this program.
#[no_mangle] // ensure that this symbol is called `main` in the output
pub extern fn main(_argc: i32, _argv: *const *const u8) -> i32 {  
  // Do stuff here
  loop {}
}


