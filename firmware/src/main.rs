#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use cortex_m_rt::entry;

#[cfg(feature = "use_semihosting")]
#[macro_use]
#[cfg(feature = "use_semihosting")]
extern crate cortex_m_semihosting;
#[cfg(feature = "use_semihosting")]
use cortex_m_semihosting::{hprintln, hio};
#[cfg(feature = "use_semihosting")]
use core::fmt::Write;

// extern crate stm32f1xx_hal;
use stm32f1xx_hal::{
    prelude::*, // auto import of most used stuff
    gpio::*, // gpio hal implementation for stm32f1xx
    pac::{self, interrupt, EXTI} // peripheral access crate (register access)
};
use embedded_hal::digital::v2::OutputPin;
use core::{cell::RefCell, ops::DerefMut};
use cortex_m::{interrupt::Mutex};

// Make external interrupt registers globally available
static INT: Mutex<RefCell<Option<EXTI>>> = Mutex::new(RefCell::new(None));

// Make our LED globally available
static LED: Mutex<RefCell<Option<gpiob::PB12<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));


#[entry]
fn main() -> ! {
    #[cfg(feature = "use_semihosting")]
    semihosting_print_example().ok();

    // Get access to the core peripherals from the cortex-m crate
    // let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let mut led = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);

    // turn on led (inverted logic)
    led.set_low().unwrap();


    // input pin and interrupt setup
    // PA5 ChA, PA10 = ChB
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let _pin_a5 = gpioa.pa0.into_pull_up_input(&mut gpioa.crl);
    let _pin_a10 = gpioa.pa7.into_pull_up_input(&mut gpioa.crl);


    let exti = dp.EXTI;

    // Set interrupt request masks
    exti.imr.modify(|_, w| {
        w.mr5().set_bit();
        w.mr10().set_bit()
    });

    // Set interrupt falling triggers
    exti.ftsr.modify(|_, w| {
        w.tr5().set_bit();
        w.tr10().set_bit()
    });

    // Enable the external interrupt in the NVIC.
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::EXTI15_10);
        pac::NVIC::unmask(pac::Interrupt::EXTI9_5);
    }

    // Move control over LED and DELAY and EXTI into global mutexes
    cortex_m::interrupt::free(|cs| {
        *LED.borrow(cs).borrow_mut() = Some(led);
        *INT.borrow(cs).borrow_mut() = Some(exti);
    });


    loop {
        // your code goes here
    }
}

#[interrupt]
fn EXTI9_5() {
    cortex_m::interrupt::free(|cs| {
        if let Some(ref mut exti) = INT.borrow(cs).borrow_mut().deref_mut() {
            // Clear the interrupt flag.
            exti.pr.modify(|_, w| w.pr5().set_bit());

            // Change the LED state on each interrupt.
            if let Some(ref mut led) = LED.borrow(cs).borrow_mut().deref_mut() {
                led.toggle().unwrap();
            }

        }
    });
}


#[interrupt]
fn EXTI15_10() {
 cortex_m::interrupt::free(|cs| {
        if let Some(ref mut exti) = INT.borrow(cs).borrow_mut().deref_mut() {
            // Clear the interrupt flag.
            exti.pr.write(|w| w.pr10().set_bit());

            // Change the LED state on each interrupt.
            if let Some(ref mut led) = LED.borrow(cs).borrow_mut().deref_mut() {
                led.toggle().unwrap();
            }

        }
    });
}


#[cfg(feature = "use_semihosting")]
fn semihosting_print_example() -> Result<(), core::fmt::Error> {

    hprintln!("Hello, rust world!").unwrap();

    const UUID: *mut u32 = 0x1FFF_F7E8 as *mut u32;
    dbg!(UUID);

    let mut uuid: [u32; 4] = [0; 4];
    for i in 0..4 {
        dbg!(i);
        uuid[i] = unsafe { dbg!(UUID.offset(i as isize).read_volatile()) };
    }

    let mut stdout = match hio::hstdout() {
        Ok(fd) => fd,
        Err(()) => return Err(core::fmt::Error),
    };

    let language = "Rust";
    let ranking = 1;

    write!(stdout, "{} on embedded is #{}!\n", language, ranking)?;

    Ok(())
}