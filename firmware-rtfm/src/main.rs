
#![no_std]
#![no_main]

extern crate panic_halt;

use core::fmt::{self, Write};

use embedded_hal::digital::v2::OutputPin;

use arrayvec::ArrayString;

use rtfm::cyccnt::{U32Ext as _};

use stm32f1xx_hal::{
    prelude::*,
    pac::{self, EXTI},
    serial::{self, Config, Serial},
    timer::{ Timer, Event, CountDownTimer },
    gpio::{*, gpiob::PB12, Output, PushPull },
};

use heapless::{
    Vec,
    i,
    spsc::{Consumer, Producer, Queue},
    consts::*
};


mod encoder;
use encoder::{Encoder, Channel, EncoderPair, EncoderInterface};

#[derive(PartialEq)]
pub enum EncoderIndex {
    EncoderNone = 0,
    Encoder1 = 1,
    Encoder2,
    Encoder3,
    Encoder4,
    Encoder5,
}

impl fmt::Display for EncoderIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           EncoderIndex::EncoderNone => write!(f, "-"),
           EncoderIndex::Encoder1 => write!(f, "1"),
           EncoderIndex::Encoder2 => write!(f, "2"),
           EncoderIndex::Encoder3 => write!(f, "3"),
           EncoderIndex::Encoder4 => write!(f, "4"),
           EncoderIndex::Encoder5 => write!(f, "5"),
       }
    }
}

type Enc1 = Encoder<
    gpioa::PA5<Input<PullUp>>,
    gpioc::PC13<Input<PullUp>>,
    gpiob::PB8<Output<PushPull>>>;

type Enc2 = Encoder<
    gpioa::PA6<Input<PullUp>>,
    gpioa::PA12<Input<PullUp>>,
    gpioa::PA4<Output<PushPull>>>;

type Enc3 =  Encoder<
    gpioa::PA7<Input<PullUp>>,
    gpioa::PA11<Input<PullUp>>,
    gpiob::PB0<Output<PushPull>>>;

type Enc4 = Encoder<
    gpioa::PA9<Input<PullUp>>,
    gpioa::PA10<Input<PullUp>>,
    gpiob::PB1<Output<PushPull>>>;

type Enc5 = Encoder<
    gpioa::PA8<Input<PullUp>>,
    gpiob::PB15<Input<PullUp>>,
    gpiob::PB14<Output<PushPull>>>;


const BAUDRATE: u32 = 57600;

const ENCODER_PREFIX: &str = "KEY ";
const ENCODER_SUFFIX: &str = "END\n";

#[rtfm::app(device = stm32f1xx_hal::pac, peripherals = true, monotonic = rtfm::cyccnt::CYCCNT)]
const APP: () = {

    struct Resources {
        time_ms: u32,
        led: PB12<Output<PushPull>>,
        timer: CountDownTimer<pac::TIM1>,
        rx2: serial::Rx<pac::USART2>,
        tx2: serial::Tx<pac::USART2>,
        tx3: serial::Tx<pac::USART3>,
        rx3: serial::Rx<pac::USART3>,
        exti: EXTI,
        encoder1: Enc1,
        encoder2: Enc2,
        encoder3: Enc3,
        encoder4: Enc4,
        encoder5: Enc5,
        encoder_vector: Vec<EncoderPair, U300>,
        p: Producer<'static, u8, U4096>,
        c: Consumer<'static, u8, U4096>,
        ext_p: Producer<'static, u8, U4096>,
        ext_c: Consumer<'static, u8, U4096>,
        uart_in_buffer: ArrayString::<[u8; 50]>,
        cmd_buffer: ArrayString::<[u8; 10]>,
    }

    #[init(spawn = [startup])]
    fn init(cx: init::Context) -> init::LateResources {

        static mut EXT_Q: Queue<u8, U4096> = Queue(i::Queue::new());
        static mut Q: Queue<u8, U4096> = Queue(i::Queue::new());

        let (ext_p, ext_c) = EXT_Q.split();
        let (p, c) = Q.split();

        // Cortex-M peripherals
        let mut core: rtfm::Peripherals = cx.core;
        // Initialize (enable) the monotonic timer (CYCCNT)
        core.DCB.enable_trace();
        core.DWT.enable_cycle_counter();

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
        let pin_a6 = gpioa.pa6.into_pull_up_input(&mut gpioa.crl);
        let pin_a7 = gpioa.pa7.into_pull_up_input(&mut gpioa.crl);
        let pin_a8 = gpioa.pa8.into_pull_up_input(&mut gpioa.crh);
        let pin_a9 = gpioa.pa9.into_pull_up_input(&mut gpioa.crh);
        let pin_a10 = gpioa.pa10.into_pull_up_input(&mut gpioa.crh);
        let pin_a11 = gpioa.pa11.into_pull_up_input(&mut gpioa.crh);
        let pin_a12 = gpioa.pa12.into_pull_up_input(&mut gpioa.crh);
        let pin_b15 = gpiob.pb15.into_pull_up_input(&mut gpiob.crh);
        let pin_c13 = gpioc.pc13.into_pull_up_input(&mut gpioc.crh);

        // configure correct pins for external interrups
        afio.exticr2.exticr2().modify(|_,w| unsafe {
            w.exti5().bits(0b0000);  //PA5 1A
            w.exti6().bits(0b0000);  //PA6 2A
            w.exti7().bits(0b0000)   //PA7 3A
        });
        afio.exticr3.exticr3().modify(|_,w| unsafe {
            w.exti8().bits(0b0000);  //PA8 5A
            w.exti9().bits(0b0000);  //PA9 4A
            w.exti10().bits(0b0000); //PA10 4B
            w.exti11().bits(0b0000)  //PA11 3B
        });
        afio.exticr4.exticr4().modify(|_,w| unsafe {
            w.exti12().bits(0b0000); //PA12 2B
            w.exti15().bits(0b0001); //PB15 5B
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
        let ch2_led = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
        let ch3_led = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);
        let ch4_led = gpiob.pb1.into_push_pull_output(&mut gpiob.crl);
        let ch5_led = gpiob.pb14.into_push_pull_output(&mut gpiob.crh);

        let encoder1 = Encoder::new(pin_a5, pin_c13, ch1_led);
        let encoder2 = Encoder::new(pin_a6, pin_a12, ch2_led);
        let encoder3 = Encoder::new(pin_a7, pin_a11, ch3_led);
        let encoder4 = Encoder::new(pin_a9, pin_a10, ch4_led);
        let encoder5 = Encoder::new(pin_a8, pin_b15, ch5_led);

        //USART2_TX PA2
        //USART2_RX PA3
        let uart2_tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
        let uart2_rx = gpioa.pa3;

        let mut serial2 = Serial::usart2(
            _device.USART2,
            (uart2_tx, uart2_rx),
            &mut afio.mapr,
            Config::default().baudrate(BAUDRATE.bps()),
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

        let mut serial3 = Serial::usart3(
            _device.USART3,
            (uart3_tx, uart3_rx),
            &mut afio.mapr,
            Config::default().baudrate(BAUDRATE.bps()),
            clocks,
            &mut rcc.apb1,
        );

        serial3.listen(serial::Event::Rxne);


        let (tx3, rx3) = serial3.split();

        // Configure the syst timer to trigger an update every second and enables interrupt
        let mut timer = Timer::tim1(_device.TIM1, &clocks, &mut rcc.apb2)
            .start_count_down(10.khz());
        timer.listen(Event::Update);

        cx.spawn.startup().ok();

        // Return the initialised resources.
        init::LateResources {
            time_ms: 0,
            led,
            timer,
            rx2,
            tx2,
            tx3,
            rx3,
            exti,
            encoder1,
            encoder2,
            encoder3,
            encoder4,
            encoder5,
            encoder_vector: Vec::new(),
            c,
            p,
            ext_p,
            ext_c,
            uart_in_buffer: ArrayString::new(),
            cmd_buffer: ArrayString::new(),
        }
    }

    #[task(resources=[encoder1, encoder2, encoder3, encoder4, encoder5, led])]
    fn startup(cx: startup::Context) {
        let startup::Resources {
            mut encoder1,
            mut encoder2,
            mut encoder3,
            mut encoder4,
            mut encoder5,
            led,
        } = cx.resources;

        for _ in 0..3 {
            led.toggle().unwrap();
            encoder1.lock(|encoder| encoder.toggle_led(20));
            led.toggle().unwrap();
            encoder2.lock(|encoder| encoder.toggle_led(20));
            led.toggle().unwrap();
            encoder3.lock(|encoder| encoder.toggle_led(20));
            led.toggle().unwrap();
            encoder4.lock(|encoder| encoder.toggle_led(20));
            led.toggle().unwrap();
            encoder5.lock(|encoder| encoder.toggle_led(20));
            led.toggle().unwrap();
        }

    }

    #[task(resources=[encoder1, encoder2, encoder3, encoder4, encoder5, p], spawn=[send])]
    fn reset_encoders(cx: reset_encoders::Context) {

        let reset_encoders::Resources {
            mut encoder1,
            mut encoder2,
            mut encoder3,
            mut encoder4,
            mut encoder5,
            mut p,
        } = cx.resources;

        encoder1.lock(|encoder| encoder.zero());
        encoder2.lock(|encoder| encoder.zero());
        encoder3.lock(|encoder| encoder.zero());
        encoder4.lock(|encoder| encoder.zero());
        encoder5.lock(|encoder| encoder.zero());

        encoder1.lock(|encoder| encoder.toggle_led(10));
        encoder2.lock(|encoder| encoder.toggle_led(10));
        encoder3.lock(|encoder| encoder.toggle_led(10));
        encoder4.lock(|encoder| encoder.toggle_led(10));
        encoder5.lock(|encoder| encoder.toggle_led(10));

        p.lock(|p| write_string_to_queue(p, "# Reset Encoders\n"));

        cx.spawn.send().ok();

    }

    #[task]
    fn system_reset(_cx: system_reset::Context) {
        cortex_m::peripheral::SCB::sys_reset();
    }

    #[task(resources=[encoder1, encoder2, encoder3, encoder4, encoder5, p], spawn=[send])]
    fn encoder_positions(cx: encoder_positions::Context) {
        let encoder_positions::Resources {
            mut encoder1,
            mut encoder2,
            mut encoder3,
            mut encoder4,
            mut encoder5,
            mut p,
        } = cx.resources;

        let mut pos: [i16; 5] = [1,2,3,4,5];
        encoder1.lock(|encoder| pos[0] = encoder.current_position());
        encoder2.lock(|encoder| pos[1] = encoder.current_position());
        encoder3.lock(|encoder| pos[2] = encoder.current_position());
        encoder4.lock(|encoder| pos[3] = encoder.current_position());
        encoder5.lock(|encoder| pos[4] = encoder.current_position());


        let mut string: ArrayString::<[u8; 40]> = ArrayString::new();
        write!(string, "POS {},{},{},{},{}\n", pos[0], pos[1], pos[2], pos[3], pos[4]).ok();
        p.lock(|p| write_string_to_queue(p, string.as_str()));
        cx.spawn.send().ok();
    }

    #[task(binds=USART2, priority = 1, resources=[rx2, tx3, cmd_buffer, p], spawn=[reset_encoders, encoder_positions, send], schedule=[system_reset])]
    fn serial_cmd(cx: serial_cmd::Context) {

        let serial_cmd::Resources {
            rx2,
            tx3,
            cmd_buffer,
            mut p
        } = cx.resources;

        match rx2.read() {
            Ok(b) => {
                match cmd_buffer.try_push(b as char) {
                    Ok(_n)  => {},
                    Err(_buffer_error) => {
                        cmd_buffer.clear();
                        return;
                    }
                }
                // send byte to next board
                tx3.write(b).ok();

                if b == b'\n' {
                    if cmd_buffer.starts_with("reset") {
                        cx.spawn.reset_encoders().ok();
                    } else if cmd_buffer.starts_with("pos") {
                        cx.spawn.encoder_positions().ok();
                    } else if cmd_buffer.starts_with("sysreset") {
                        let now = cx.start;
                        cx.schedule.system_reset(now + 8_000_000.cycles()).ok();
                    } else {
                        // cx.spawn.unknown_command().ok();
                        p.lock(|p| {
                            write_string_to_queue(p, "unknown cmd \"");
                            // remove newline
                            cmd_buffer.pop();
                            write_string_to_queue(p, cmd_buffer);
                            write_string_to_queue(p, "\"\n");
                        });
                        cx.spawn.send().ok();
                    }
                    cmd_buffer.clear();
                }
            },
            Err(_e) => {}
        }


    }

    #[task(priority = 3, resources=[ext_p, uart_in_buffer], spawn = [send], capacity = 100)]
    fn uart_buffer(cx: uart_buffer::Context, byte: Result<u8, nb::Error<serial::Error>>) {

        let uart_buffer::Resources {
            ext_p,
            uart_in_buffer
        } = cx.resources;

        match byte {
            Ok(b) => {

                match uart_in_buffer.try_push(b as char) {
                    Ok(_n)  => {},
                    Err(_buffer_error) => {
                        uart_in_buffer.clear();
                        return;
                    }
                }

                if b == b'\n' {
                    if uart_in_buffer.starts_with(ENCODER_PREFIX) {
                        let i = ENCODER_PREFIX.len();
                        let substr = &uart_in_buffer[i..uart_in_buffer.len()-1];
                        let enc_idx: i32 = substr.parse().unwrap();
                        let new_enc_index = enc_idx + 5;
                        uart_in_buffer.truncate(i);
                        let mut int_string: ArrayString::<[u8; 4]> = ArrayString::new();
                        write!(int_string, "{}\n", new_enc_index).ok();
                        uart_in_buffer.push_str(int_string.as_str());
                    }

                    write_string_to_queue(ext_p, uart_in_buffer);
                    uart_in_buffer.clear();
                    cx.spawn.send().ok();
                }
            }
            Err(_e) => {
                writeln!(uart_in_buffer, "{:?}", _e).unwrap();
                write_string_to_queue(ext_p, uart_in_buffer);
                uart_in_buffer.clear();
                cx.spawn.send().ok();
            }
        }


    }

    #[task(binds = USART3, resources = [rx3], priority = 5, spawn=[uart_buffer])]
    fn usart_in(cx: usart_in::Context) {

        let usart_in::Resources {
            rx3,
        } = cx.resources;

        cx.spawn.uart_buffer(rx3.read()).ok();
    }

    #[task(binds = TIM1_UP, resources = [timer, time_ms], priority = 7)]
    fn tim1_up(cx: tim1_up::Context) {
        *cx.resources.time_ms += 1;
        cx.resources.timer.clear_update_interrupt_flag();
    }

    #[task(resources = [tx2, c, ext_c], priority = 2)]
    fn send(cx: send::Context) {

        let send::Resources {
            tx2,
            c,
            ext_c
        } = cx.resources;

        while c.ready() {
            if let Some(byte) = c.dequeue() {
                nb::block!(tx2.write(byte)).unwrap()
            }
        }

        while ext_c.ready() {
           if let Some(byte) = ext_c.dequeue() {
                nb::block!(tx2.write(byte)).unwrap()
            }
        }

    }


    #[task(priority = 3, resources=[encoder_vector, p], spawn = [send], capacity = 10)]
    fn enc_buffer(cx: enc_buffer::Context, enc_index: EncoderIndex, data_point: Option<EncoderPair>, ready: bool) {
        static mut ACTIVE_ENCODER: EncoderIndex = EncoderIndex::EncoderNone;

        if enc_index == EncoderIndex::EncoderNone {
            return;
        }

        let enc_buffer::Resources {
            encoder_vector,
            p,
        } = cx.resources;

        if *ACTIVE_ENCODER == EncoderIndex::EncoderNone {
            *ACTIVE_ENCODER = enc_index;
        } else if *ACTIVE_ENCODER != enc_index {
            // busy with other encoder: drop data
            return;
        }

        if let Some(data_point) = data_point {
            encoder_vector.push(data_point).ok();
        } else {
            encoder_vector.clear();
            *ACTIVE_ENCODER = EncoderIndex::EncoderNone;
        }

        if ready {
            // header
            let mut uart_string: ArrayString::<[u8; 20]> = ArrayString::new();
            writeln!(uart_string, "{}{}", ENCODER_PREFIX, ACTIVE_ENCODER).unwrap();
            write_string_to_queue(p, &uart_string.as_str());
            for x in encoder_vector.iter() {
                uart_string.clear();
                let t = x.get_time();
                let v = x.get_position();
                writeln!(uart_string, "{}:{}", t, v).unwrap();
                write_string_to_queue(p, &uart_string.as_str());
            }
            write_string_to_queue(p, ENCODER_SUFFIX);

            encoder_vector.clear();
            *ACTIVE_ENCODER = EncoderIndex::EncoderNone;
            cx.spawn.send().ok();
        }
    }



    /**
     *Ch A interrupt
    */
    #[task(binds = EXTI9_5, resources = [encoder1, encoder2, encoder3, encoder4, encoder5, time_ms, exti], priority = 6, spawn = [enc_buffer])]
    fn encoder_a(cx: encoder_a::Context) {

        let encoder_a::Resources {
            encoder1,
            encoder2,
            encoder3,
            encoder4,
            encoder5,
            mut time_ms,
            exti
        } = cx.resources;

        let mut current_time: u32 = 0;

        time_ms.lock(|time_ms| {
            current_time = *time_ms;
        });

        let channel = Channel::A;

        if let Some(res) = encoder_isr((encoder1, encoder2,encoder3,
            encoder4,
            encoder5,), exti, current_time, channel) {
            cx.spawn.enc_buffer(res.0, res.1, res.2).ok();
        }
    }

    /**
     *Ch B interrupt
    */
    #[task(binds = EXTI15_10, resources = [encoder1, encoder2, encoder3, encoder4, encoder5, time_ms, exti], priority = 6, spawn = [enc_buffer])]
    fn encoder_b(cx: encoder_b::Context) {

        let encoder_b::Resources {
            encoder1,
            encoder2,
            encoder3,
            encoder4,
            encoder5,
            mut time_ms,
            exti
        } = cx.resources;


        let mut current_time: u32 = 0;

        time_ms.lock(|time_ms| {
            current_time = *time_ms;
        });
        let channel = Channel::B;


        if let Some(res) = encoder_isr((encoder1, encoder2, encoder3,
            encoder4,
            encoder5,), exti, current_time, channel) {
            cx.spawn.enc_buffer(res.0, res.1, res.2).ok();
        }
    }

     // Interrupt handlers used to dispatch software tasks
    extern "C" {
        fn USART1();
        fn SPI1();
        fn SPI2();
    }


};

fn write_string_to_queue(q: &mut Producer<'static, u8, U4096>, string: &str) {
    for byte in string.bytes() {
        q.enqueue(byte).unwrap();
    }
}

fn encoder_isr(encoder: (&mut impl EncoderInterface, &mut impl EncoderInterface, &mut impl EncoderInterface, &mut impl EncoderInterface, &mut impl EncoderInterface),
    exti: &EXTI, t: u32, channel: Channel) -> Option<(EncoderIndex, Option<EncoderPair>, bool)> {

    let pr = exti.pr.read();
    if pr.pr5().bit_is_set() || pr.pr13().bit_is_set() {
        // Clear the interrupt flagw.
        exti.pr.write(|w| {
            w.pr5().set_bit();
            w.pr13().set_bit()
        });

        let datapoint = encoder.0.update(&channel, t);
        let tuple = (EncoderIndex::Encoder1, datapoint, encoder.0.ready());
        return Some(tuple);
    }
    if pr.pr6().bit_is_set() || pr.pr12().bit_is_set() {
        // Clear the interrupt flagw.
        exti.pr.write(|w| {
            w.pr6().set_bit();
            w.pr12().set_bit()
        });
        let datapoint = encoder.1.update(&channel, t);
        let tuple = (EncoderIndex::Encoder2, datapoint, encoder.1.ready());
        return Some(tuple);
    }
    if pr.pr7().bit_is_set() || pr.pr11().bit_is_set() {
        // Clear the interrupt flagw.
        exti.pr.write(|w| {
            w.pr7().set_bit();
            w.pr11().set_bit()
        });
        let datapoint = encoder.2.update(&channel, t);
        let tuple = (EncoderIndex::Encoder3, datapoint, encoder.2.ready());
        return Some(tuple);
    }
    if pr.pr9().bit_is_set() || pr.pr10().bit_is_set() {
        // Clear the interrupt flagw.
        exti.pr.write(|w| {
            w.pr9().set_bit();
            w.pr10().set_bit()
        });
        let datapoint = encoder.3.update(&channel, t);
        let tuple = (EncoderIndex::Encoder4, datapoint, encoder.3.ready());
        return Some(tuple);
    }
    if pr.pr8().bit_is_set() || pr.pr15().bit_is_set() {
        // Clear the interrupt flagw.
        exti.pr.write(|w| {
            w.pr8().set_bit();
            w.pr15().set_bit()
        });
        let datapoint = encoder.4.update(&channel, t);
        let tuple = (EncoderIndex::Encoder5, datapoint, encoder.4.ready());
        return Some(tuple);
    }
    None
}