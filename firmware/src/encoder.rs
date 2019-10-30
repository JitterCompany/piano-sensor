
extern crate heapless;
use heapless::Vec;
use heapless::consts::*;

extern crate embedded_hal;
use embedded_hal::digital::v2::{InputPin, OutputPin};

#[repr(packed)]
pub struct EncoderPair {
    pub time: u32,
    pub pos:  i16
}

// impl EncoderPair {
//     pub fn get_time(&mut self) -> u32 { self.time }

//     pub fn get_position(&mut self) -> i16 { self.pos }
// }

pub struct Encoder<CHA: InputPin, CHB: InputPin, LED: OutputPin> {
    data: Vec<EncoderPair, U200>,
    ready: bool,
    start: u32,
    channel_a: CHA,
    channel_b: CHB,
    led: LED,
    _prev_val: i16,
    position: i16,
    max: i16
}

use void::Void;

pub enum Channel {
    A,
    B
}


impl<CHA: InputPin<Error = Void>, CHB: InputPin<Error = Void>, LED: OutputPin<Error = Void>> Encoder<CHA, CHB, LED> {

    pub fn new(ch_a: CHA, ch_b: CHB, led: LED) -> Self {
        Self {
            data: Vec::new(),
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
        self.data.clear();
        self.ready = false;
        self.start = 0;
        self._prev_val = 0;
        self.max = 0;
        {
            self.led.set_low().unwrap();
        }
    }

    pub fn update(&mut self, channel: &Channel, timestamp: u32) {
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

        // safe datapoint
        self.new_value(timestamp, self.position);

    }

    fn new_value(&mut self, timestamp: u32, position: i16) {
        if self.ready {
            return
        }
        if position != self._prev_val {
            if self.start == 0 {
                self.start = timestamp;
            }
            let t = timestamp - self.start; // & 0xFFFF;

            self.data.push(EncoderPair{time: t, pos: position}).ok();

            self._prev_val = position;
        }

        let abs_pos = position.abs();
        if abs_pos > self.max {
            self.max = abs_pos;
        }

        if self.max > 10 {
            if self.start != 0 && position == 0 {
                self.ready = true;
            }
        } else if position == 0 {
            self.reset();
        }
    }

    pub fn ready(&mut self) -> bool { self.ready }

    pub fn get(&mut self) -> &Vec<EncoderPair, U200> { &self.data }

    pub fn position(&mut self) -> i16 {
        self.position
    }
}
