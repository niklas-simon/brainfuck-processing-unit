use std::{thread, time::Duration};

use rppal::gpio::{Gpio, IoPin, Level, Mode, OutputPin};
use serde::Deserialize;

pub fn bits(byte: u8) -> [Level; 8] {
    let mut res = [Level::Low; 8];
    for i in 0..8 {
        res[i] = Level::from((byte >> i) & 1);
    }
    res
}

pub fn byte(bits: [Level; 8]) -> u8 {
    let mut val = 0;
    for i in 0..8 {
        val |= (bits[i] as u8) << i;
    }
    val
}

const PULSE_DURATION: Duration = Duration::from_nanos(200);

pub fn pulse_io(pin: &mut IoPin) {
    pin.set_high();
    thread::sleep(PULSE_DURATION);
    pin.set_low();
}

pub fn pulse(pin: &mut OutputPin) {
    pin.set_high();
    thread::sleep(PULSE_DURATION);
    pin.set_low();
}

pub struct IOPort {
    pins: [IoPin; 11],
}
pub struct ControlPort {
    pins: [OutputPin; 3],
}
pub struct ProgramPort {
    pins: [OutputPin; 3],
}

impl IOPort {
    pub fn new(pins: impl IntoIterator<Item = u8>) -> Self {
        let gpio = Gpio::new().unwrap();
        let mut inp_pins: Vec<_> = pins
            .into_iter()
            .map(|pin| gpio.get(pin).unwrap().into_io(Mode::Input))
            .collect();
        inp_pins[2].set_mode(Mode::Output);
        Self {
            pins: inp_pins.try_into().unwrap(),
        }
    }

    fn set_pinmode(&mut self, mode: Mode) {
        self.pins[1].set_mode(mode);
        for pin in 3..11 {
            self.pins[pin].set_mode(mode);
        }
    }
}

impl ControlPort {
    pub fn new(pins: impl IntoIterator<Item = u8>) -> Self {
        let gpio = Gpio::new().unwrap();
        let inp_pins: Vec<_> = pins
            .into_iter()
            .map(|pin| gpio.get(pin).unwrap().into_output())
            .collect();
        Self {
            pins: inp_pins.try_into().unwrap(),
        }
    }

    pub fn set_control(&mut self, level: bool) {
        self.pins[0].write(level.into());
    }

    pub fn reset(&mut self) {
        pulse(&mut self.pins[1]);
    }

    pub fn step(&mut self) {
        pulse(&mut self.pins[2]);
    }
}

impl ProgramPort {
    pub fn new(pins: impl IntoIterator<Item = u8>) -> Self {
        let gpio = Gpio::new().unwrap();
        let inp_pins: Vec<_> = pins
            .into_iter()
            .map(|pin| gpio.get(pin).unwrap().into_output())
            .collect();
        Self {
            pins: inp_pins.try_into().unwrap(),
        }
    }

    pub fn write_program(&mut self, code: &str) {
        self.pins[0].set_high();
        let to_write = code
            .chars()
            .filter(|c| ['+', '-', '<', '>', '.', ',', '[', ']'].contains(c))
            .map(|c| c as u8);
        for byte in to_write {
            let b = bits(byte);
            for i in (0..8).rev() {
                self.pins[2].write(b[i]);
                pulse(&mut self.pins[1]);
            }
        }
        self.pins[0].set_low();
    }
}

pub struct Ports {
    pub io: IOPort,
    pub program: ProgramPort,
    pub control: ControlPort,
}

#[derive(Deserialize)]
struct PinFile {
    io: Vec<u8>,
    program: Vec<u8>,
    control: Vec<u8>,
}

impl Ports {
    pub fn new() -> Option<Self> {
        let pinfile = std::env::var("PIN_FILE").unwrap_or(String::from("pins.json"));
        let pins = std::fs::read_to_string(pinfile).unwrap();
        let pins: PinFile = serde_json::from_str(&pins).unwrap();
        Some(Self {
            control: ControlPort::new(pins.control),
            io: IOPort::new(pins.io),
            program: ProgramPort::new(pins.program),
        })
    }

    pub fn handle_io(&mut self, inp: impl FnOnce() -> u8) -> Option<u8> {
        // Input: writing to pins
        if self.io.pins[0].read() == Level::High {
            self.io.set_pinmode(Mode::Output);
            let data = bits(inp());
            for i in 0..8 {
                self.io.pins[i + 3].write(data[i]);
            }
            // pulse for push input
            pulse_io(&mut self.io.pins[1]);
            self.io.set_pinmode(Mode::Input);
            self.control.step();
        }
        // Output: reading from pins
        if self.io.pins[1].read() == Level::High {
            let mut val = [Level::Low; 8];
            for i in 0..8 {
                val[i] = self.io.pins[i + 3].read();
            }
            // pulse for output confirmed
            pulse_io(&mut self.io.pins[2]);
            self.control.step();
            return Some(byte(val));
        }
        None
    }
}
