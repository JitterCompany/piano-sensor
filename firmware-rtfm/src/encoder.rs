
// extern crate heapless;
// use heapless::Vec;
// use heapless::consts::*;

extern crate embedded_hal;
use embedded_hal::digital::v2::{InputPin, OutputPin};

#[repr(packed)]
pub struct EncoderPair {
    pub time: u32,
    pub pos:  i16
}

impl EncoderPair {
    pub fn get_time(&self) -> u32 { self.time }

    pub fn get_position(&self) -> i16 { self.pos }
}

pub struct Encoder<CHA: InputPin, CHB: InputPin, LED: OutputPin> {
    // data: Vec<EncoderPair, U300>,
    ready: bool,
    start: u32,
    channel_a: CHA,
    channel_b: CHB,
    led: LED,
    _prev_val: i16,
    position: i16,
    max: i16
}

pub enum Channel {
    A,
    B
}


impl<CHA: InputPin<Error = core::convert::Infallible>, CHB: InputPin<Error = core::convert::Infallible>, LED: OutputPin<Error = core::convert::Infallible>> Encoder<CHA, CHB, LED> {

    pub fn new(ch_a: CHA, ch_b: CHB, led: LED) -> Self {
        Self {
            // data: Vec::new(),
            ready: false,
            start: 0,
            _prev_val: 0,
            channel_a: ch_a,
            channel_b: ch_b,
            led: led,
            position: 0,
            max: 0
        }
    }

    pub fn reset(&mut self) {
        // self.data.clear();
        self.ready = false;
        self.start = 0;
        self._prev_val = 0;
        self.max = 0;
        {
            self.led.set_low().unwrap();
        }
    }

    pub fn update(&mut self, channel: &Channel, timestamp: u32) -> EncoderPair {
        let a: bool = self.channel_a.is_high().unwrap();
        let b: bool = self.channel_b.is_high().unwrap();
        match *channel {
            Channel::A => {
                if a == b {
                    self.position -= 1;
                    // COUNTER.decrement(cs);
                } else {
                    self.position += 1;
                    // COUNTER.increment(cs);
                }
                self.led.set_low().unwrap();
            },
            Channel::B => {
                if a == b {
                    self.position += 1;
                    // COUNTER.decrement(cs);
                } else {
                    self.position -= 1;
                    // COUNTER.increment(cs);
                }
                self.led.set_high().unwrap();
            }
        }

        // create new datapoint
        self.new_value(timestamp, self.position)

    }

    fn new_value(&mut self, timestamp: u32, position: i16) -> EncoderPair {

        // TODO: return optional
        // if self.ready {
        //     return
        // }

        // let mut dataPoint: EncoderPair;
        // if position != self._prev_val {
            if self.start == 0 {
                self.start = timestamp;
            }
            let t = timestamp - self.start; // & 0xFFFF;

            // self.data.push(EncoderPair{time: t, pos: position}).ok();
            let data_point = EncoderPair{time: t, pos: position};

            self._prev_val = position;
        // }

        let abs_pos = position.abs();
        if abs_pos > self.max {
            self.max = abs_pos;
        }

        if self.max > 30 {
            if self.start != 0 && position == 0 {
                self.ready = true;
            }
        } else if position == 0 {
            self.reset();
        }

        data_point
    }

    pub fn ready(&mut self) -> bool {
        let isReady = self.ready;
        if isReady {
            self.reset();
            // self.ready = false
        }
        isReady
    }

    pub fn set_ready(&mut self, ready: bool) { self.ready = ready }

    // pub fn get(&self) -> &Vec<EncoderPair, U300> { &self.data }

    pub fn position(&mut self) -> i16 {
        self.position
    }
}
