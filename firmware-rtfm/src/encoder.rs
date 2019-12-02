
use embedded_hal::digital::v2::{InputPin, OutputPin};
use core::convert::Infallible;

#[repr(packed)]
pub struct EncoderPair {
    pub time: u32,
    pub pos:  i16
}

impl EncoderPair {
    pub fn get_time(&self) -> u32 { self.time }

    pub fn get_position(&self) -> i16 { self.pos }
}

pub trait EncoderInterface {
    fn update(&mut self, channel: &Channel, timestamp: u32) -> Option<EncoderPair>;
    fn ready(&mut self) -> bool;
    fn toggle_led(&mut self, delay: u16);
    fn zero(&mut self);
    fn current_position(&mut self) -> i16;
}

pub struct Encoder<CHA: InputPin, CHB: InputPin, LED: OutputPin> {
    ready: bool,
    done: bool,
    start: u32,
    channel_a: CHA,
    channel_b: CHB,
    led: LED,
    position: i16,
    max: i16
}

pub enum Channel {
    A,
    B
}

impl<CHA, CHB, LED> EncoderInterface for Encoder<CHA, CHB, LED>
    where
    CHA: InputPin<Error = Infallible>,
    CHB: InputPin<Error = Infallible>,
    LED: OutputPin<Error = Infallible>
    {

    fn update(&mut self, channel: &Channel, timestamp: u32) -> Option<EncoderPair> {
        let a: bool = self.channel_a.is_high().unwrap();
        let b: bool = self.channel_b.is_high().unwrap();
        match *channel {
            Channel::A => {
                if a == b {
                    self.position -= 1;
                } else {
                    self.position += 1;
                }
            },
            Channel::B => {
                if a == b {
                    self.position += 1;
                } else {
                    self.position -= 1;
                }
            }
        }

        // create new datapoint
        self.new_value(timestamp, self.position)

    }

    fn current_position(&mut self) -> i16 {
        self.position
    }

    fn ready(&mut self) -> bool {
        let is_ready = self.ready;
        if is_ready {
            self.ready = false;
            self.done = true;
            self.led.set_high().unwrap();
        }
        is_ready
    }

    fn toggle_led(&mut self, delay: u16) {
        self.led.set_high().unwrap();
        for _ in 0..delay*1000 {}
        self.led.set_low().unwrap();
    }

    fn zero(&mut self) {
        self.position = 0;
        self.reset();
    }


}


impl<CHA: InputPin<Error = Infallible>, CHB: InputPin<Error = Infallible>, LED: OutputPin<Error = Infallible>> Encoder<CHA, CHB, LED> {

    pub fn new(ch_a: CHA, ch_b: CHB, led: LED) -> Self {
        Self {
            ready: false,
            done: false,
            start: 0,
            channel_a: ch_a,
            channel_b: ch_b,
            led: led,
            position: 0,
            max: 0
        }
    }

    pub fn reset(&mut self) {
        self.ready = false;
        self.start = 0;
        self.done = false;
        self.max = 0;
        self.led.set_low().unwrap();
    }


    fn new_value(&mut self, timestamp: u32, position: i16) -> Option<EncoderPair> {

        if self.start == 0 {
            self.start = timestamp;
        }
        let t = timestamp - self.start;

        let data_point = EncoderPair{time: t, pos: position};


        let abs_pos = position.abs();
        if abs_pos > self.max {
            self.max = abs_pos;
        }

        if self.max > 100 {
            if self.start != 0 && (abs_pos < (self.max - 50)) {
                if !self.done {
                    self.ready = true;
                }
            }
        }

        if abs_pos < 10 {
            self.reset();
            return None
        }

        if !self.done {
            Some(data_point)
        } else {
            None
        }
    }

}
