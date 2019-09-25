
extern crate heapless;
use heapless::Vec;
use heapless::consts::*;

extern crate embedded_hal;
use embedded_hal::digital::v2::InputPin;


pub struct EncoderPair {
    pub time: u32,
    pub pos:  i32
}

pub struct Encoder<CHA: InputPin, CHB: InputPin> {
    data: Vec<EncoderPair, U300>,
    ready: bool,
    start: u32,
    _prev_val: i32,
    channel_a: CHA,
    channel_b: CHB,
    position: i32
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
            position: 0
        }
    }

    pub fn reset(&mut self) {
        self.data = Vec::new();
        self.ready = false;
        self.start = 0;
        self._prev_val = 0;
    }

    pub fn update(&mut self, channel: Channel, timestamp: u32) {
        let a: bool = self.channel_a.is_high().unwrap();
        let b: bool = self.channel_b.is_high().unwrap();
        match channel {
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

    fn new_value(&mut self, timestamp: u32, position: i32) {
        if self.ready {
            return
        }
        if position != self._prev_val {
            if self.start == 0 {
                self.start = timestamp;
            }
            self.data.push(EncoderPair{time: timestamp - self.start, pos: position}).ok();
            self._prev_val = position;
        }

        if self.start != 0 && position == 0 {
            self.ready = true;
        }
    }

    pub fn ready(&mut self) -> bool { self.ready }

    pub fn get(&mut self) -> &Vec<EncoderPair, U300> { &self.data }
}
