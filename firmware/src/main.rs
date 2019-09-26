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

extern crate stm32f1xx_hal;
use stm32f1xx_hal::{
    prelude::*, // auto import of most used stuff
    gpio::*, // gpio hal implementation for stm32f1xx
    timer,
    serial::{Config, Serial},
    pac::{self, interrupt, EXTI, USART2} // peripheral access crate (register access)
};

extern crate embedded_hal;
use embedded_hal::digital::v2::{OutputPin};

use core::{cell::RefCell, ops::DerefMut, cell::UnsafeCell};
use cortex_m::{interrupt::Mutex};

mod encoder;
use encoder::{Encoder, Channel};

mod counter;
use counter::CSCounter;


// workaround for issue https://github.com/stm32-rs/stm32f1xx-hal/issues/110 in stm32f1xx-hal
use core::fmt;
use::core::fmt::*;
struct FormatTx {
    tx :  stm32f1xx_hal::serial::Tx<USART2>,
}

impl fmt::Write for FormatTx {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        s.as_bytes()
            .iter()
            .try_for_each(|c| nb::block!(self.tx.write(*c)))
            .map_err(|_| core::fmt::Error)
    }
}


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
    #[cfg(feature = "use_semihosting")] {
        hprintln!("sysclk freq: {}", clocks.sysclk().0).unwrap();
    }

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    let mut led = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);

    // turn on led (inverted logic)
    led.set_high().unwrap();


    // input pin and interrupt setup
    // PA5 ChA, PA10 = ChB
    let pin_a5 = gpioa.pa5.into_pull_up_input(&mut gpioa.crl);
    let pin_a6 = gpioa.pa6.into_pull_up_input(&mut gpioa.crl);
    let pin_a7 = gpioa.pa7.into_pull_up_input(&mut gpioa.crl);
    let pin_a8 = gpioa.pa8.into_pull_up_input(&mut gpioa.crh);
    let pin_a9 = gpioa.pa9.into_pull_up_input(&mut gpioa.crh);
    let pin_a10 = gpioa.pa10.into_pull_up_input(&mut gpioa.crh);
    let pin_a11 = gpioa.pa11.into_pull_up_input(&mut gpioa.crh);
    let pin_a12 = gpioa.pa12.into_pull_up_input(&mut gpioa.crh);
    let pin_b15 = gpiob.pb15.into_pull_up_input(&mut gpiob.crh);
    let pin_c13 = gpioc.pc13.into_pull_up_input(&mut gpioc.crh);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    // configure correct pins for external interrups
    afio.exticr2.exticr2().modify(|_,w| unsafe {
        w.exti5().bits(0b0000);  //PA5 1A
        w.exti6().bits(0b0000);  //PA6 2A
        w.exti7().bits(0b0000)   //PA7 3A
    });
    afio.exticr3.exticr3().modify(|_,w| unsafe {
        w.exti8().bits(0b0000);  //PA8 4A
        w.exti9().bits(0b0000);  //PA9 5A
        w.exti10().bits(0b0000); //PA10 1B
        w.exti11().bits(0b0000)  //PA11 2B
    });
     afio.exticr4.exticr4().modify(|_,w| unsafe {
        w.exti12().bits(0b0000); //PA12 3B
        w.exti15().bits(0b0001); //PB15 4B
        w.exti13().bits(0b0010)  //PC13 5B
    });

    let encoder1 = Encoder::new(pin_a5, pin_a10);
    let encoder2 = Encoder::new(pin_a6, pin_a11);
    let encoder3 = Encoder::new(pin_a7, pin_a12);
    let encoder4 = Encoder::new(pin_a8, pin_b15);
    let encoder5 = Encoder::new(pin_a9, pin_c13);

    let exti = dp.EXTI;

    // Set interrupt request masks; enable interrupts
    exti.imr.modify(|_, w| {
        w.mr5().set_bit();
        w.mr6().set_bit();
        w.mr7().set_bit();
        w.mr8().set_bit();
        w.mr9().set_bit();
        w.mr10().set_bit();
        w.mr11().set_bit();
        w.mr12().set_bit();
        w.mr15().set_bit();
        w.mr13().set_bit()
    });

    // Set interrupt falling and rising edge triggers
    exti.ftsr.modify(|_, w| {
        w.tr5().set_bit();
        w.tr6().set_bit();
        w.tr7().set_bit();
        w.tr8().set_bit();
        w.tr9().set_bit();
        w.tr10().set_bit();
        w.tr11().set_bit();
        w.tr12().set_bit();
        w.tr15().set_bit();
        w.tr13().set_bit()
    });
    exti.rtsr.modify(|_, w| {
        w.tr5().set_bit();
        w.tr6().set_bit();
        w.tr7().set_bit();
        w.tr8().set_bit();
        w.tr9().set_bit();
        w.tr10().set_bit();
        w.tr11().set_bit();
        w.tr12().set_bit();
        w.tr15().set_bit();
        w.tr13().set_bit()
    });

    // Enable the external interrupts for these lines in the NVIC.
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
        *ENCODER.borrow(cs).borrow_mut() = Some(encoder1);
    });

    let (tx, _) = serial.split();

    block!(timer.wait()).unwrap();
    let mut tx = FormatTx {
        tx: tx
    };
    writeln!(tx, "let's start! {}!", 12).unwrap();

    loop {

        block!(timer.wait()).unwrap();

        cortex_m::interrupt::free(|cs| {
            if let Some(ref mut encoder) = ENCODER.borrow(cs).borrow_mut().deref_mut() {
                if encoder.ready() {
                    let data = encoder.get();
                    for x in data {
                        writeln!(tx, "{}: {}", x.time, x.pos).unwrap();
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
            let _pr = exti.pr.read();

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
