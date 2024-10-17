use std::{
    sync::{LazyLock, Mutex, RwLock}, thread, time::Instant
};

use bf_itp::Run;
use rocket::{
    delete, get, post, put,
    response::stream::{Event, EventStream},
    tokio::time::{self, Duration}
};
use rppal::gpio::{Gpio, IoPin, Level, Mode, OutputPin};

// have a cat

fn main() {
    let mut args = std::env::args().skip(1);
    let file = args.next().expect("missing file to interpret");
    let code = std::fs::read_to_string(file).expect("failed to read code");
    if let Some(skill_target) = args.next() {
        let skill = bf_itp::get_skill(&code, &skill_target);
        println!("estimated skill: {skill}");
    } else {
        run_hw(&code);
    }
}

pub enum ItpState {
    Initial {
        code: String,
        input: String,
    },
    Running(bf_itp::Run),
    // finished
}

impl ItpState {
    pub fn new() -> Self {
        Self::Initial {
            code: String::new(),
            input: String::new(),
        }
    }

    pub fn start(&mut self) {
        match self {
            ItpState::Initial { code, input } => {
                let run = Run::new(code, input);
                *self = ItpState::Running(run);
            },
            _ => return,
        }
    }
}

pub type Speed = Option<u32>;
// speed: 1..=100
// frequency = 10^(3 * log10(speed)) = speed^3
// interval = 1 / frequency

/*static CURR_SPEED: RwLock<Speed> = RwLock::new(None);
static CURR_STATE: LazyLock<RwLock<ItpState>> = LazyLock::new(|| RwLock::new(ItpState::new()));

static LAST_SPEED_CHANGE: LazyLock<RwLock<Instant>> = LazyLock::new(|| RwLock::new(Instant::now()));
static LAST_CODE_CHANGE: LazyLock<RwLock<Instant>> = LazyLock::new(|| RwLock::new(Instant::now()));
static LAST_INPUT_CHANGE: LazyLock<RwLock<Instant>> = LazyLock::new(|| RwLock::new(Instant::now()));
static LAST_STATE_CHANGE: LazyLock<RwLock<Instant>> = LazyLock::new(|| RwLock::new(Instant::now()));*/

static GLOBAL_STATE: LazyLock<Global> = LazyLock::new(|| Global::new());

fn get_urls_to_fetch(last_sent: &Instant) -> Option<String> {
    let all_paths = [
        (&GLOBAL_STATE.last_change.speed, "/api/run/speed"),
        (&GLOBAL_STATE.last_change.code, "/api/run/code"),
        (&GLOBAL_STATE.last_change.input, "/api/run/input"),
        (&GLOBAL_STATE.last_change.state, "/api/run/state"),
    ];
    let changed = all_paths
        .into_iter()
        .flat_map(|(lock, path)|
            lock
                .read()
                .unwrap()
                .gt(last_sent)
                .then_some(path)
        )
        .collect::<Vec<_>>();
    (!changed.is_empty()).then(|| format!("{:?}", changed))
}

#[delete("/run/speed")]
pub fn set_speed_manual() {
    *GLOBAL_STATE.speed.write().unwrap() = None;
    *GLOBAL_STATE.last_change.speed.write().unwrap() = Instant::now();
}

#[put("/run/speed", data = "<speed>")]
pub fn set_speed_auto(speed: String) {
    *GLOBAL_STATE.speed.write().unwrap() = speed.parse().ok();
    *GLOBAL_STATE.last_change.speed.write().unwrap() = Instant::now();
}

#[post("/run")]
pub fn event_stream() -> EventStream![] {
    EventStream! {
        let mut interval = time::interval(Duration::from_millis(40));
        let mut last_sent = Instant::now();
        loop {
            if let Some(evt_data) = get_urls_to_fetch(&last_sent) {
                yield Event::data(evt_data);
                last_sent = Instant::now();
            }
            interval.tick().await;
        }
    }
}

#[post("/run/step")]
pub fn step() {
    let mut state = GLOBAL_STATE.state.write().unwrap();
    match &mut *state {
        ItpState::Running(run) => {
            run.step();
            // release lock as early as possible
            drop(state);
            *GLOBAL_STATE.last_change.state.write().unwrap() = Instant::now();
        },
        _ => {},
    }
}

#[put("/run/code", data = "<data>")]
pub fn set_code(data: String) {
    *GLOBAL_STATE.full_code.write().unwrap() = data;
    *GLOBAL_STATE.last_change.code.write().unwrap() = Instant::now();
}

#[get("/run/code")]
pub fn get_code() -> String {
    GLOBAL_STATE.full_code.read().unwrap().clone()
}

#[put("/run/input", data = "<data>")]
pub fn set_input(data: String) {
    let mut state = GLOBAL_STATE.state.write().unwrap();
    match &mut *state {
        ItpState::Initial { input, .. } => {
            *input = data;
            drop(state);
            *GLOBAL_STATE.last_change.input.write().unwrap() = Instant::now();
        },
        _ => {},        
    }
}

#[get("/run/input")]
pub fn get_input() -> String {
    let state = GLOBAL_STATE.state.read().unwrap();
    match &*state {
        ItpState::Initial { input, .. } => input.clone(),
        ItpState::Running(run) => String::from_utf8(run.inp.clone()).unwrap_or_default(),
    }
}

#[get("/run/state")]
pub fn get_state() -> String {
    let state = GLOBAL_STATE.state.read().unwrap();
    match &*state {
        ItpState::Running(run) => {
            serde_json::to_string(&run.state()).unwrap_or_default()
        },
        _ => String::new(),
    }
}

#[post("/reset")]
pub fn reset() {
    GLOBAL_STATE.ports.lock().unwrap().control.reset();
    *GLOBAL_STATE.state.write().unwrap() = ItpState::new();
}

/// DO NOT USE
/// 
/// just a general idea of how to run hw communication
fn run_hw(code: &str) {
    let mut ports = GLOBAL_STATE.ports.lock().unwrap();
    // write program to eeprom
    ports.control.set_control(Level::High);
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

fn bits(byte: u8) -> [Level; 8] {
    let mut res = [Level::Low; 8];
    for i in 0..8 {
        res[i] = Level::from((byte >> i) & 1);
    }
    res
}

fn byte(bits: [Level; 8]) -> u8 {
    let mut val = 0;
    for i in 0..8 {
        val |= (bits[i] as u8) << i;
    }
    val
}

fn pulse_io(pin: &mut IoPin) {
    pin.set_high();
    thread::sleep(Duration::from_nanos(200));
    pin.set_low();
}

fn pulse(pin: &mut OutputPin) {
    pin.set_high();
    thread::sleep(Duration::from_nanos(200));
    pin.set_low();
}

struct IOPort {
    pins: [rppal::gpio::IoPin; 11],
}
struct ControlPort {
    pins: [rppal::gpio::OutputPin; 3],
}
struct ProgramPort {
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

    pub fn set_control(&mut self, level: Level) {
        self.pins[0].write(level);
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

struct Ports {
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

pub struct ChangeTimes {
    speed: RwLock<Instant>,
    code: RwLock<Instant>,
    input: RwLock<Instant>,
    state: RwLock<Instant>,
}

impl ChangeTimes {
    pub fn new() -> Self {
        Self {
            speed: RwLock::new(Instant::now()),
            code: RwLock::new(Instant::now()),
            input: RwLock::new(Instant::now()),
            state: RwLock::new(Instant::now()),
        }
    }
}

pub struct Global {
    ports: Mutex<Ports>,
    speed: RwLock<Speed>,
    full_code: RwLock<String>,
    state: RwLock<ItpState>,
    last_change: ChangeTimes,
}

impl Global {
    pub fn new() -> Self {
        Self {
            ports: Mutex::new(Ports::new()),
            speed: RwLock::new(None),
            full_code: RwLock::new(String::new()),
            state: RwLock::new(ItpState::new()),
            last_change: ChangeTimes::new(),
        }
    }
}