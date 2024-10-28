use std::{io::Cursor, sync::{mpsc::{self, Sender}, OnceLock, RwLock}, time::Instant};

use bf_itp::Run;
use rocket::{fs::{relative, FileServer, NamedFile}, get, http::{ContentType, Status}, response::{self, Responder}, routes, Request, Response};
use serde_json::{json, Value};

#[cfg(all(target_arch = "aarch64", target_env = "gnu"))]
mod hw;
#[cfg(not(all(target_arch = "aarch64", target_env = "gnu")))]
#[path ="hw_mock.rs"]
mod hw;
mod api;

// have a cat

pub type BFRes = Result<(), BFError>;

#[rocket::main]
async fn main() {
    let (sx, tx) = mpsc::channel::<HWCmd>();
    GLOBAL_STATE.set(Global::new(sx)).expect("global already initialised");
    let _hw_runner = hw::start_hw_thread(tx);
    start_rocket().await;
}

async fn start_rocket() {
    rocket::build()
        .mount("/api", api::get_routes())
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api", routes![get_examples])
        .launch().await
        .expect("failed to launch rocket");
}

#[get("/examples")]
async fn get_examples() -> Option<NamedFile> {
    NamedFile::open("examples.json").await.ok()
}

#[derive(Debug)]
pub enum ItpState {
    Idle,
    Running {
        run: Run,
        paused: bool,
    },
    // input counter
    Uncontrolled(usize),
}

// speed: 1..=100
// frequency = 10^(3 * log10(speed)) = speed^3
// interval = 1 / frequency

static GLOBAL_STATE: OnceLock<Global> = OnceLock::new();

#[derive(Debug)]
pub struct ChangeTimes {
    speed: RwLock<Instant>,
    code: RwLock<Instant>,
    input: RwLock<Instant>,
    output: RwLock<Instant>,
    state: RwLock<Instant>,
}

impl ChangeTimes {
    pub fn new() -> Self {
        Self {
            speed: RwLock::new(Instant::now()),
            code: RwLock::new(Instant::now()),
            input: RwLock::new(Instant::now()),
            output: RwLock::new(Instant::now()),
            state: RwLock::new(Instant::now()),
        }
    }
}

#[derive(Debug)]
pub struct Global {
    hw: Sender<HWCmd>,
    speed: RwLock<u32>,
    full_code: RwLock<String>,
    input: RwLock<String>,
    output: RwLock<String>,
    state: RwLock<ItpState>,
    hw_state: RwLock<HWState>,
    last_change: ChangeTimes,
}

impl Global {
    pub fn new(hw: Sender<HWCmd>) -> Self {
        Self {
            hw,
            speed: RwLock::new(100),
            full_code: RwLock::new(String::new()),
            input: RwLock::new(String::new()),
            output: RwLock::new(String::new()),
            state: RwLock::new(ItpState::Idle),
            hw_state: RwLock::new(HWState::Regular),
            last_change: ChangeTimes::new(),
        }
    }
}

impl Global {
    fn set_input(&self, inp: String) {
        *self.input.write().unwrap() = inp;
        *self.last_change.input.write().unwrap() = Instant::now();
    }
    
    fn set_code(&self, code: String) {
        *self.full_code.write().unwrap() = code;
        *self.last_change.code.write().unwrap() = Instant::now();
    }
    
    fn set_output(&self, output: String) {
        *self.output.write().unwrap() = output;
        *self.last_change.output.write().unwrap() = Instant::now();
    }
    
    fn set_state(&self, state: ItpState) {
        *self.state.write().unwrap() = state;
        *self.last_change.state.write().unwrap() = Instant::now();
    }

    pub fn set_speed(&self, speed: u32) {
        *self.speed.write().unwrap() = speed;
        *self.last_change.speed.write().unwrap() = Instant::now();
    }

    pub fn change_input(&self, inp: String) -> BFRes {
        match *self.state.read().unwrap() {
            ItpState::Running { ref run, .. } => {
                let curr = self.input.read().unwrap();
                if inp.len() >= run.ic && curr[..run.ic] == inp[..run.ic] {
                    self.set_input(inp);
                    Ok(())
                } else {
                    Err(BFError::InputChanged)
                }
            },
            _ => {
                self.set_input(inp);
                Ok(())
            },
        }
    }

    pub fn change_code(&self, code: String) -> BFRes {
        match *self.state.read().unwrap() {
            ItpState::Idle => {
                self.set_code(code);
                Ok(())
            },
            ItpState::Running { .. } => Err(BFError::CodeChanged),
            ItpState::Uncontrolled(_) => {
                self.set_code(code);
                self.send_hw(HWCmd::Program);
                Ok(())
            },
        }
    }

    pub fn get_state(&self) -> Value {
        let ctrl = match *self.hw_state.read().unwrap() {
            HWState::Regular => match *self.state.read().unwrap() {
                ItpState::Idle => "idle",
                ItpState::Running { paused, .. } => if paused { "paused" } else { "running" },
                ItpState::Uncontrolled(_) => "uncontrolled",
            },
            HWState::Startup => "startup",
            HWState::WaitInput => "wait_input",
            HWState::OutputReady => "output_ready",
        };
        match *self.state.read().unwrap() {
            ItpState::Running { ref run, .. } => serde_json::to_value(run.state(ctrl)).unwrap_or(json!({"control": ctrl})),
            _ => json!({"control": ctrl}),
        }
    }

    pub fn send_hw(&self, cmd: HWCmd) {
        self.hw.send(cmd).expect("hardware thread died");
    }

    pub fn itp_started(&self, paused: bool) {
        self.set_output(String::new());
        *self.hw_state.write().unwrap() = HWState::Regular;
        let code = &*self.full_code.read().unwrap();
        let input = &*self.full_code.read().unwrap();
        let run = Run::new(code, input);
        self.set_state(ItpState::Running { run, paused });
        println!("run started");
    }
}

pub enum BFError {
    /// trying to change code while itp is running
    CodeChanged,
    /// change input while itp running
    InputChanged,   
    /// tried to apply control when control is disabled
    ItpUncontrolled,
    /// interpreter already running
    ItpRunning,
    /// interpreter not running
    ItpNotRunning,
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for BFError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let (status, text) = match self {
            BFError::CodeChanged => (Status::UnprocessableEntity, "cannot change code while interpreter is running"),
            BFError::InputChanged => (Status::UnprocessableEntity, "cannot change already read input during run"),
            BFError::ItpUncontrolled => (Status::BadRequest, "control is currently not enabled"),
            BFError::ItpRunning => (Status::BadRequest, "interpreter is currently running"),
            BFError::ItpNotRunning => (Status::BadRequest, "interpreter is currently not running"),
        };
        Response::build()
            .header(ContentType::Plain)
            .sized_body(text.len(), Cursor::new(text))
            .status(status)
            .ok()
    }
}

pub enum HWCmd {
    EndControl,
    StartControl,
    Program,
    StartRun,
    StartRunPaused,
    ExecStep,
    Reset,
}

#[derive(Debug)]
pub enum HWState {
    Regular,
    Startup,
    WaitInput,
    OutputReady,
}