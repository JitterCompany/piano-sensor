
extern crate heapless;
use heapless::Vec;
use heapless::consts::*;

extern crate embedded_hal;
use embedded_hal::digital::v2::InputPin;

#[repr(packed)]
pub struct EncoderPair {
    pub time: u32,
    pub pos:  i16
}

impl EncoderPair {
    pub fn get_time(&mut self) -> u32 { self.time }

    pub fn get_position(&mut self) -> i16 { self.pos }
}

#[repr(packed)]
pub struct Encoder<CHA: InputPin, CHB: InputPin> {
    data: Vec<EncoderPair, U300>,
    ready: bool,
    start: u32,
    channel_a: CHA,
    channel_b: CHB,
    _prev_val: i16,
    position: i16,
    max: i16
}

use void::Void;

pub enum Channel {
    A,
    B
}


impl<CHA: InputPin<Error = Void>, CHB: InputPin<Error = Void>> Encoder<CHA, CHB> {

    pub fn new(ch_a: CHA, ch_b: CHB) -> Self {
        Self {
            data: Vec::new(),
            ready: false,
            start: 0,
            _prev_val: 0,
            channel_a: ch_a,
            channel_b: ch_b,
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
    }

    pub fn update(&mut self, channel: &Channel, timestamp: u32) {
        let a: bool = unsafe { self.channel_a.is_high().unwrap() };
        let b: bool = unsafe { self.channel_b.is_high().unwrap() };
        match *channel {
            Channel::A => {
                if a == b {
                    self.position -= 1;
                    // COUNTER.decrement(cs);
                } else {
                    self.position += 1;
                    // COUNTER.increment(cs);
                }
            },
            Channel::B => {
                if a == b {
                    self.position += 1;
                    // COUNTER.decrement(cs);
                } else {
                    self.position -= 1;
                    // COUNTER.increment(cs);
                }
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
            unsafe {
                self.data.push(EncoderPair{time: t, pos: position}).ok();
            }
            self._prev_val = position;
        }

        if position > self.max {
            self.max = position;
        }

        // if self.max > 10 {
            if self.start != 0 && position == 0 {
                self.ready = true;
            }
        // }
    }

    pub fn ready(&mut self) -> bool { self.ready }

    pub fn get(&mut self) -> &Vec<EncoderPair, U300> { unsafe { &self.data } }

}
