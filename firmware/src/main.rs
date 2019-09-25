#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use cortex_m_rt::entry;
use nb::block;

#[cfg(feature = "use_semihosting")]
#[macro_use]
#[cfg(feature = "use_semihosting")]
extern crate cortex_m_semihosting;
#[cfg(feature = "use_semihosting")]
use cortex_m_semihosting::{hprintln, hio};
#[cfg(feature = "use_semihosting")]
use core::fmt::Write;

extern crate stm32f1xx_hal;
use stm32f1xx_hal::{
    prelude::*, // auto import of most used stuff
    gpio::*, // gpio hal implementation for stm32f1xx
    timer,
    serial::{Config, Serial},
    pac::{self, interrupt, EXTI} // peripheral access crate (register access)
};

extern crate embedded_hal;
use embedded_hal::digital::v2::{OutputPin};
use embedded_hal::{serial};

use core::{cell::RefCell, ops::DerefMut, cell::UnsafeCell};
use cortex_m::{interrupt::Mutex};

mod encoder;
use encoder::{Encoder, Channel};

mod counter;
use counter::CSCounter;


// Make external interrupt registers globally available
static INT: Mutex<RefCell<Option<EXTI>>> = Mutex::new(RefCell::new(None));

// Make our LED globally available
static LED: Mutex<RefCell<Option<gpiob::PB12<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));

static TIMER_UP: Mutex<RefCell<Option<timer::Timer<stm32f1xx_hal::pac::TIM1>>>> = Mutex::new(RefCell::new(None));

static TIME_MS: CSCounter<u32> = CSCounter(UnsafeCell::new(0));



static ENCODER: Mutex<RefCell<Option<
    Encoder<
        gpioa::PA5<Input<PullUp>>,
        gpioa::PA10<Input<PullUp>>
        >>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    // #[cfg(feature = "use_semihosting")]
    // semihosting_print_example().ok();
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();

    let mut rcc = dp.RCC.constrain();

    // configure clocks
    let clocks = rcc.cfgr.use_hse(8.mhz()).sysclk(72.mhz()).pclk1(36.mhz()).freeze(&mut flash.acr);
    // let clocks = rcc.cfgr.freeze(&mut flash.acr);
    #[cfg(feature = "use_semihosting")] {
        hprintln!("sysclk freq: {}", clocks.sysclk().0).unwrap();
    }

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let mut led = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);

    // turn on led (inverted logic)
    led.set_high().unwrap();


    // input pin and interrupt setup
    // PA5 ChA, PA10 = ChB
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let pin_a5 = gpioa.pa5.into_pull_up_input(&mut gpioa.crl);
    let pin_a10 = gpioa.pa10.into_pull_up_input(&mut gpioa.crh);

    let encoder = Encoder::new(pin_a5, pin_a10);


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
    exti.rtsr.modify(|_, w| {
        w.tr5().set_bit();
        w.tr10().set_bit()
    });

    // Enable the external interrupt in the NVIC.
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::EXTI15_10);
        pac::NVIC::unmask(pac::Interrupt::EXTI9_5);
    }


    let mut timer = timer::Timer::syst(cp.SYST, 1000.hz(), clocks);
    let mut tim1 = timer::Timer::tim1(dp.TIM1, 1.khz(), clocks, &mut rcc.apb2);

    tim1.listen(timer::Event::Update);
    tim1.clear_update_interrupt_flag();

    unsafe {
        pac::NVIC::unmask(pac::Interrupt::TIM1_UP);
        let mut nvic = cp.NVIC;
        nvic.set_priority(pac::Interrupt::TIM1_UP, 0); // prio 0
        nvic.set_priority(pac::Interrupt::EXTI15_10, 32); // prio 1
        nvic.set_priority(pac::Interrupt::EXTI9_5, 32); // prio 1
    }



    //USART2_TX PA2
    //USART2_RX PA3
    let tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let rx = gpioa.pa3;


    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let serial = Serial::usart2(
        dp.USART2,
        (tx, rx),
        &mut afio.mapr,
        Config::default().baudrate(115200.bps()),
        clocks,
        &mut rcc.apb1,
    );



    // Move control over LED and DELAY and EXTI into global mutexes
    cortex_m::interrupt::free(|cs| {
        *LED.borrow(cs).borrow_mut() = Some(led);
        *INT.borrow(cs).borrow_mut() = Some(exti);
        *TIMER_UP.borrow(cs).borrow_mut() = Some(tim1);
        *ENCODER.borrow(cs).borrow_mut() = Some(encoder);
    });

    let (mut tx, _) = serial.split();
    let enter_r = b'\r';
    let enter_n = b'\n';
    let comma = b',';
    let ready = "ready:\r\n";

    let splus = "counter = ";
    let smin = "counter = -";

    let mut prev_val: i32 = -1;

    loop {

        block!(timer.wait()).unwrap();


        // let val: i32 = COUNTER.get();

        // if prev_val != val {

        //     if val < 0 {
        //         write_string(&mut tx, &smin);
        //         print_int(&mut tx, -val as u32);
        //     } else {
        //         write_string(&mut tx, &splus);
        //         print_int(&mut tx, val as u32);
        //     }
        //     block!(tx.write(enter_r)).ok();
        //     block!(tx.write(enter_n)).ok();
        //     prev_val = val;
        // }

        cortex_m::interrupt::free(|cs| {
            if let Some(ref mut encoder) = ENCODER.borrow(cs).borrow_mut().deref_mut() {
                if encoder.ready() {
                    write_string(&mut tx, &ready);
                    let data = encoder.get();
                    for x in data {
                        print_int(&mut tx, x.time as u32);
                        block!(tx.write(comma)).ok();
                        print_int(&mut tx, -x.pos as u32);
                        block!(tx.write(enter_r)).ok();
                        block!(tx.write(enter_n)).ok();
                    }
                    encoder.reset();

                }
            }
        });


    }
}


#[interrupt]
fn TIM1_UP() {
    cortex_m::interrupt::free(|cs| {
        if let Some(ref mut tim1) = TIMER_UP.borrow(cs).borrow_mut().deref_mut() {
            tim1.clear_update_interrupt_flag();
            // count 1 millisecond
            TIME_MS.increment(cs);
        }
    });
}

/**
 *Ch A interrupt
 */
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

            if let Some(ref mut encoder) = ENCODER.borrow(cs).borrow_mut().deref_mut() {
                encoder.update(Channel::A, TIME_MS.get() as u32);
            }
        }
    });
}


/**
 *Ch B interrupt
 */
#[interrupt]
fn EXTI15_10() {
 cortex_m::interrupt::free(|cs| {
        if let Some(ref mut exti) = INT.borrow(cs).borrow_mut().deref_mut() {
            // Clear the interrupt flag.
            exti.pr.write(|w| w.pr10().set_bit());
            let pr = exti.pr.read();

            // Change the LED state on each interrupt.
            if let Some(ref mut led) = LED.borrow(cs).borrow_mut().deref_mut() {
                led.toggle().unwrap();
            }

            if let Some(ref mut encoder) = ENCODER.borrow(cs).borrow_mut().deref_mut() {
                encoder.update(Channel::B, TIME_MS.get() as u32);
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


pub fn print_int (tx: &mut impl serial::Write<u8>, i : u32) {
    if i == 0 { block!(tx.write('0' as u8)).ok(); return; };

    let mut i = i;
    let mut s = [0 as u8; 10];
    let mut j = 0;
    while i != 0 {
        let rem = (i % 10) as u8;
        s[j] = '0' as u8 + rem;
        j += 1;
        i = i / 10;
    }

    for x in 0..j {
        block!(tx.write(s[j-x-1])).ok();
    }
}

fn write_string(tx: &mut impl serial::Write<u8>, s: &str) {
    for a in s.chars() {
        block!(tx.write(a as u8)).ok();
    }
}
