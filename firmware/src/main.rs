#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

#[macro_use]
extern crate cortex_m_semihosting;
use cortex_m_semihosting::{hprintln, hio};
use core::fmt::Write;

extern crate stm32f1xx_hal;

#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code
    hprintln!("Hello, rust world!").unwrap();
    print();

    const UUID: *mut u32 = 0x0009_FC70 as *mut u32;
    dbg!(UUID);
    let mut uuid: [u32; 4] = [0; 4];
    for i in 0..4 {
        dbg!(i);
        uuid[i] = unsafe { dbg!(UUID.offset(i as isize).read_volatile()) };
    }

    loop {
        // your code goes here
    }
}

// This function will be called by the application


// This function will be called by the application
fn print() -> Result<(), core::fmt::Error> {
    let mut stdout = match hio::hstdout() {
        Ok(fd) => fd,
        Err(()) => return Err(core::fmt::Error),
    };

    let language = "Rust";
    let ranking = 1;

    write!(stdout, "{} on embedded is #{}!\n", language, ranking)?;

    Ok(())
}