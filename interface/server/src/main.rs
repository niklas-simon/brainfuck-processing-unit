use std::{sync::{LazyLock, Mutex, RwLock}, thread, time::Instant};

use bf_itp::Run;

mod hw;
mod api;

use hw::Ports;

// have a cat

#[rocket::main]
async fn main() {
    let _hw_runner = thread::spawn(|| hw::run_hw("TODO"));
    start_rocket().await;
}

async fn start_rocket() {
    rocket::build()
        .mount("/api", api::get_routes())
        .launch().await
        .expect("failed to launch rocket");
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