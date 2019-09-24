
extern crate heapless;
use heapless::Vec;
use heapless::consts::*;

pub struct EncoderPair {
    pub time: u32,
    pub pos:  i32
}

pub struct Encoder {
    data: Vec<EncoderPair, U200>,
    ready: bool,
    start: u32,
    _prev_val: i32
}

impl Encoder {
    // pub fn new()
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            ready: false,
            start: 0,
            _prev_val: 0
        }
    }

    pub fn reset(&mut self) {
        self.data = Vec::new();
        self.ready = false;
        self.start = 0;
        self._prev_val = 0;
    }

    pub fn new_value(&mut self, timestamp: u32, position: i32) {
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

    pub fn get(&mut self) -> &Vec<EncoderPair, U200> { &self.data }
}
