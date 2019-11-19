
#![no_std]
#![no_main]

extern crate panic_halt;

use core::fmt::Write;
// use core::fmt;

extern crate embedded_hal;
use embedded_hal::digital::v2::OutputPin;

use stm32f1xx_hal::{
    prelude::*,
    pac::{self, EXTI},
    serial::{self, Config, Serial},
    timer::{ Timer, Event, CountDownTimer },
    gpio::{*, gpiob::PB12, Output, PushPull },
};

mod encoder;
use encoder::{Encoder, Channel, EncoderPair};
extern crate heapless;
use heapless::Vec;
use heapless::consts::*;

use heapless::{
    consts::*,
    i,
    spsc::{Consumer, Producer, Queue},
};

use arrayvec::ArrayString;

#[rtfm::app(device = stm32f1xx_hal::pac, peripherals = true)]
const APP: () = {

    struct Resources {
        time_ms: u32,
        led: PB12<Output<PushPull>>,
        timer: CountDownTimer<pac::TIM1>,
        rx2: serial::Rx<pac::USART2>,
        tx2: serial::Tx<pac::USART2>,
        tx3: serial::Tx<pac::USART3>,
        exti: EXTI,
        encoder1: Encoder<
                    gpioa::PA5<Input<PullUp>>,
                    gpioc::PC13<Input<PullUp>>,
                    gpiob::PB8<Output<PushPull>>>,
        encoder_vector: Vec<EncoderPair, U100>,
        p: Producer<'static, u8, U8192>,
        c: Consumer<'static, u8, U8192>,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {

        static mut Q: Queue<u8, U8192> = Queue(i::Queue::new());
        let (p, c) = Q.split();

        // Cortex-M peripherals
        let _core: cortex_m::Peripherals = cx.core;

        // Device specific peripherals
        let _device = cx.device;

        let mut flash = _device.FLASH.constrain();
        let mut rcc = _device.RCC.constrain();
        let clocks = rcc.cfgr.use_hse(8.mhz()).sysclk(72.mhz()).pclk1(36.mhz()).freeze(&mut flash.acr);

        let mut gpioa = _device.GPIOA.split(&mut rcc.apb2);
        let mut gpiob = _device.GPIOB.split(&mut rcc.apb2);
        let mut gpioc = _device.GPIOC.split(&mut rcc.apb2);
        let mut afio = _device.AFIO.constrain(&mut rcc.apb2);

        let mut led = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
        led.set_low().unwrap();

        // input pin and interrupt setup
        // PA5 ChA, PA10 = ChB
        let pin_a5 = gpioa.pa5.into_pull_up_input(&mut gpioa.crl);
        // let pin_a6 = gpioa.pa6.into_pull_up_input(&mut gpioa.crl);
        // let pin_a7 = gpioa.pa7.into_pull_up_input(&mut gpioa.crl);
        // let pin_a8 = gpioa.pa8.into_pull_up_input(&mut gpioa.crh);
        // let pin_a9 = gpioa.pa9.into_pull_up_input(&mut gpioa.crh);
        // let pin_a10 = gpioa.pa10.into_pull_up_input(&mut gpioa.crh);
        // let pin_a11 = gpioa.pa11.into_pull_up_input(&mut gpioa.crh);
        // let pin_a12 = gpioa.pa12.into_pull_up_input(&mut gpioa.crh);
        // let pin_b15 = gpiob.pb15.into_pull_up_input(&mut gpiob.crh);
        let pin_c13 = gpioc.pc13.into_pull_up_input(&mut gpioc.crh);

    // configure correct pins for external interrups
        afio.exticr2.exticr2().modify(|_,w| unsafe {
            w.exti5().bits(0b0000)  //PA5 1A
            // w.exti6().bits(0b0000);  //PA6 2A
            // w.exti7().bits(0b0000)   //PA7 3A
        });
        // afio.exticr3.exticr3().modify(|_,w| unsafe {
            // w.exti8().bits(0b0000);  //PA8 5A
            // w.exti9().bits(0b0000);  //PA9 4A
            // w.exti10().bits(0b0000); //PA10 4B
            // w.exti11().bits(0b0000)  //PA11 3B
        // });
        afio.exticr4.exticr4().modify(|_,w| unsafe {
            // w.exti12().bits(0b0000); //PA12 2B
            // w.exti15().bits(0b0001); //PB15 5B
            w.exti13().bits(0b0010)  //PC13 1B
        });



        //A bits 5,6,7,8,9
        //B bits 10,11,12,15,13

        let exti = _device.EXTI;

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


        let ch1_led = gpiob.pb8.into_push_pull_output(&mut gpiob.crh);
        let encoder1 = Encoder::new(pin_a5, pin_c13, ch1_led);

        //USART2_TX PA2
        //USART2_RX PA3
        let uart2_tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
        let uart2_rx = gpioa.pa3;

        let mut serial2 = Serial::usart2(
            _device.USART2,
            (uart2_tx, uart2_rx),
            &mut afio.mapr,
            Config::default().baudrate(2000000.bps()),
            clocks,
            &mut rcc.apb1,
        );

        serial2.listen(serial::Event::Rxne);

        let (mut tx2, rx2) = serial2.split();

        writeln!(tx2, "Start {} App!", "RTFM").unwrap();

        //USART3_TX PB10
        //USART3_RX PB11
        let uart3_tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
        let uart3_rx = gpiob.pb11;

        let serial3 = Serial::usart3(
            _device.USART3,
            (uart3_tx, uart3_rx),
            &mut afio.mapr,
            Config::default().baudrate(2000000.bps()),
            clocks,
            &mut rcc.apb1,
        );

        let (tx3, _) = serial3.split();

        // Configure the syst timer to trigger an update every second and enables interrupt
        let mut timer = Timer::tim1(_device.TIM1, &clocks, &mut rcc.apb2)
            .start_count_down(1000.hz());
        timer.listen(Event::Update);


        // Return the initialised resources.
        init::LateResources {
            time_ms: 0,
            led: led,
            timer: timer,
            rx2: rx2,
            tx2: tx2,
            tx3: tx3,
            exti: exti,
            encoder1,
            encoder_vector: Vec::new(),
            c,
            p
            // uart_string: ArrayString::new(),
            // uart_string2: ArrayString::new(),
        }
    }

    // #[idle]
    // fn idle(cx: idle::Context) -> ! {
    //     loop {}
    // }

    #[task(binds = USART2, resources = [rx2], priority = 1)]
    fn usart2(cx: usart2::Context) {

        let usart2::Resources {
            rx2
        } = cx.resources;

        match rx2.read() {
            Ok(b) => {
                // tx2.write(b).unwrap();
            }
            Err(_e) => {
                // writeln!(tx2, "Serial Error: {:?}", _e).unwrap();
            }
        }

    }

    #[task(binds = TIM1_UP, resources = [timer, time_ms], priority = 4)]
    fn tim1_up(cx: tim1_up::Context) {
        *cx.resources.time_ms += 1;

        // Clear the interrupt flag.
        cx.resources.timer.clear_update_interrupt_flag();
    }

    #[task(resources = [tx2, c], priority = 3)]
    fn send(cx: send::Context) {

        let send::Resources {
            tx2,
            c
        } = cx.resources;

        writeln!(tx2, "test: OK1").unwrap();

        while c.ready() {
            if let Some(byte) = c.dequeue() {
                nb::block!(tx2.write(byte)).unwrap()
            }
        }

    }


    #[task(priority = 2, resources=[encoder_vector, p], spawn = [send], capacity = 10)]
    fn enc_buffer(cx: enc_buffer::Context, data_point: EncoderPair, ready: bool) {

        let enc_buffer::Resources {
            encoder_vector,
            p,
        } = cx.resources;

        encoder_vector.push(data_point).ok();

        if ready {
            let mut count = 0;
            for x in encoder_vector.iter() {
                let t = x.get_time();
                let v = x.get_position();
                let mut uart_string: ArrayString::<[u8; 20]> = ArrayString::new();
                writeln!(uart_string, "{}:{}", count, v).unwrap();
                for byte in uart_string.as_str().bytes() {
                    p.enqueue(byte).unwrap();
                }
                count += 1;
            }
            encoder_vector.clear();
            cx.spawn.send().ok();
        }
    }

    /**
     *Ch A interrupt
    */
    #[task(binds = EXTI9_5, resources = [encoder1, time_ms, exti], priority = 4, spawn = [enc_buffer])]
    fn encoder_a(cx: encoder_a::Context) {


        // encoder_isr(cx, Channel::A);

        let encoder_a::Resources {
            encoder1,
            time_ms,
            exti
        } = cx.resources;

        let channel = Channel::A;

        let pr = exti.pr.read();
        if pr.pr5().bit_is_set() || pr.pr13().bit_is_set() {
            // Clear the interrupt flagw.
            exti.pr.write(|w| {
                w.pr5().set_bit();
                w.pr13().set_bit()
            });
            let data_point = encoder1.update(&channel, 10);
            cx.spawn.enc_buffer(data_point, encoder1.ready()).ok();
        }
    }

    /**
     *Ch B interrupt
    */
    #[task(binds = EXTI15_10, resources = [encoder1, time_ms, exti], priority = 4, spawn = [enc_buffer])]
    fn encoder_b(cx: encoder_b::Context) {

        // encoder_isr(cx, Channel::B);

        let encoder_b::Resources {
            encoder1,
            time_ms,
            exti
        } = cx.resources;

        let channel = Channel::B;

        let pr = exti.pr.read();
        if pr.pr5().bit_is_set() || pr.pr13().bit_is_set() {
            // Clear the interrupt flagw.
            exti.pr.write(|w| {
                w.pr5().set_bit();
                w.pr13().set_bit()
            });
            let data_point = encoder1.update(&channel, 10);
            cx.spawn.enc_buffer(data_point, encoder1.ready()).ok();
        }
    }

     // Interrupt handlers used to dispatch software tasks
    extern "C" {
        fn USART1();
        fn SPI1();
    }


};