use std::{thread::{self, JoinHandle}, time::Duration};

use rppal::gpio::{Gpio, IoPin, Level, Mode, OutputPin};
use crate::GLOBAL_STATE;

pub fn start_hw_thread() -> JoinHandle<()> {
    thread::spawn(|| run_hw("TODO"))
}

/// DO NOT USE
/// 
/// just a general idea of how to run hw communication
pub fn run_hw(code: &str) {
    let mut ports = GLOBAL_STATE.ports.lock().unwrap();
    // write program to eeprom
    ports.control.set_control(true);
    ports.program.write_program(code);
    loop {
        let Some(speed) = *GLOBAL_STATE.speed.read().unwrap() else {
            thread::sleep(Duration::from_millis(10));
            continue;
        };
        ports.control.step();
        ports.io.handle_io(|| 0);
        let wait_time = Duration::from_secs_f64(1.0 / speed.pow(3) as f64);
        thread::sleep(wait_time);
    }
}

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

pub fn pulse_io(pin: &mut IoPin) {
    pin.set_high();
    thread::sleep(Duration::from_nanos(200));
    pin.set_low();
}

pub fn pulse(pin: &mut OutputPin) {
    pin.set_high();
    thread::sleep(Duration::from_nanos(200));
    pin.set_low();
}

pub struct IOPort {
    pins: [rppal::gpio::IoPin; 11],
}
pub struct ControlPort {
    pins: [rppal::gpio::OutputPin; 3],
}
pub struct ProgramPort {
    pins: [rppal::gpio::OutputPin; 3],
}

impl IOPort {
    pub fn new() -> Self {
        let gpio = Gpio::new().unwrap();
        Self {
            pins: [
                // todo: set pin numbers
                gpio.get(0).unwrap().into_io(Mode::Input),
                gpio.get(1).unwrap().into_io(Mode::Input),
                gpio.get(2).unwrap().into_io(Mode::Output),
                gpio.get(3).unwrap().into_io(Mode::Input),
                gpio.get(4).unwrap().into_io(Mode::Input),
                gpio.get(5).unwrap().into_io(Mode::Input),
                gpio.get(6).unwrap().into_io(Mode::Input),
                gpio.get(7).unwrap().into_io(Mode::Input),
                gpio.get(8).unwrap().into_io(Mode::Input),
                gpio.get(9).unwrap().into_io(Mode::Input),
                gpio.get(10).unwrap().into_io(Mode::Input),
                // pin 12 ist nur ground
            ],
        }
    }

    fn set_pinmode(&mut self, mode: Mode) {
        self.pins[1].set_mode(mode);
        for pin in 3..11 {
            self.pins[pin].set_mode(mode);
        }
    }

    pub fn handle_io(&mut self, inp: impl FnOnce() -> u8) -> Option<u8> {
        // Input: writing to pins
        if self.pins[0].read() == Level::High {
            self.set_pinmode(Mode::Output);
            let data = bits(inp());
            for i in 0..8 {
                self.pins[i + 3].write(data[i]);
            }
            // pulse for push input
            pulse_io(&mut self.pins[1]);
            self.set_pinmode(Mode::Input);
        }
        // Output: reading from pins
        if self.pins[1].read() == Level::High {
            let mut val = [Level::Low; 8];
            for i in 0..8 {
                val[i] = self.pins[i + 3].read();
            }
            // pulse for output confirmed
            pulse_io(&mut self.pins[2]);
            return Some(byte(val));
        }
        None
    }
}

impl ControlPort {
    pub fn new() -> Self {
        let gpio = Gpio::new().unwrap();
        Self {
            pins: [
                // todo: set pin numbers
                gpio.get(0).unwrap().into_output(),
                gpio.get(1).unwrap().into_output(),
                gpio.get(2).unwrap().into_output(),
                // pin 12 ist nur ground
            ],
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
    pub fn new() -> Self {
        let gpio = Gpio::new().unwrap();
        Self {
            pins: [
                // todo: set pin numbers
                gpio.get(0).unwrap().into_output(),
                gpio.get(1).unwrap().into_output(),
                gpio.get(2).unwrap().into_output(),
                // pin 12 ist nur ground
            ],
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

impl Ports {
    pub fn new() -> Self {
        Self {
            control: ControlPort::new(),
            io: IOPort::new(),
            program: ProgramPort::new(),
        }
    }
}