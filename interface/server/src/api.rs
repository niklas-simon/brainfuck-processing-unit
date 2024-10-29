use std::time::{Duration, Instant};

use rocket::{
    delete, fs::NamedFile, get, http::Status, post, put, response::stream::{Event, EventStream}, tokio::time, Route
};
use serde_json::{json, Value};

use crate::{BFError, BFRes, HWCmd, ItpState, GLOBAL_STATE};

/// 
pub fn get_routes() -> Vec<Route> {
    rocket::routes![
        // data
        get_code,
        set_code,
        get_input,
        set_input,
        get_output,
        get_state,
        get_speed,
        set_speed,
        // sse
        code_event,
        input_event,
        output_event,
        state_event,
        speed_event,
        // ctrl
        enable_control,
        disable_control,
        start,
        pause,
        step,
        reset,
        // other
        get_examples,
    ]
}

/*##############*\
##   /api/run   ##
\*##############*/

#[get("/run/code")]
pub fn get_code() -> String {
    GLOBAL_STATE
        .get()
        .unwrap()
        .full_code
        .read()
        .unwrap()
        .clone()
}

#[put("/run/code", data = "<code>")]
pub fn set_code(code: String) -> BFRes {
    GLOBAL_STATE.get().unwrap().change_code(code)
}

#[get("/run/input")]
pub fn get_input() -> String {
    GLOBAL_STATE.get().unwrap().input.read().unwrap().clone()
}

#[put("/run/input", data = "<input>")]
pub fn set_input(input: String) -> BFRes {
    GLOBAL_STATE.get().unwrap().change_input(input)
}

#[get("/run/output")]
pub fn get_output() -> String {
    GLOBAL_STATE.get().unwrap().output.read().unwrap().clone()
}

#[get("/run/state")]
pub fn get_state() -> Value {
    GLOBAL_STATE.get().unwrap().get_state()
}

#[get("/run/speed")]
pub fn get_speed() -> Value {
    json!(*GLOBAL_STATE.get().unwrap().speed.read().unwrap())
}

#[put("/run/speed", data = "<speed>")]
pub fn set_speed(speed: String) -> Status {
    let Ok(speed) = speed.parse() else {
        return Status::UnprocessableEntity;
    };
    if speed < 1 || speed > 100 {
        return Status::UnprocessableEntity;
    }
    GLOBAL_STATE.get().unwrap().set_speed(speed);
    Status::Ok
}

/*##############*\
##   /api/sse   ##
\*##############*/

#[get("/sse/code")]
pub fn code_event() -> EventStream![] {
    EventStream! {
        let mut interval = time::interval(Duration::from_millis(40));
        let mut last_sent = *GLOBAL_STATE.get().unwrap().last_change.code.read().unwrap() - Duration::from_millis(1);
        loop {
            let last_change = *GLOBAL_STATE.get().unwrap().last_change.code.read().unwrap();
            if last_change > last_sent {
                let code = (*GLOBAL_STATE.get().unwrap().full_code.read().unwrap()).clone();
                yield Event::json(&code);
                last_sent = Instant::now();
            }
            interval.tick().await;
        }
    }
}

#[get("/sse/input")]
pub fn input_event() -> EventStream![] {
    EventStream! {
        let mut interval = time::interval(Duration::from_millis(40));
        let mut last_sent = *GLOBAL_STATE.get().unwrap().last_change.input.read().unwrap() - Duration::from_millis(1);
        loop {
            let last_change = *GLOBAL_STATE.get().unwrap().last_change.input.read().unwrap();
            if last_change > last_sent {
                let input = (*GLOBAL_STATE.get().unwrap().input.read().unwrap()).clone();
                yield Event::json(&input);
                last_sent = Instant::now();
            }
            interval.tick().await;
        }
    }
}

#[get("/sse/output")]
pub fn output_event() -> EventStream![] {
    EventStream! {
        let mut interval = time::interval(Duration::from_millis(40));
        let mut last_sent = *GLOBAL_STATE.get().unwrap().last_change.output.read().unwrap() - Duration::from_millis(1);
        loop {
            let last_change = *GLOBAL_STATE.get().unwrap().last_change.output.read().unwrap();
            if last_change > last_sent {
                let output = (*GLOBAL_STATE.get().unwrap().output.read().unwrap()).clone();
                yield Event::json(&output);
                last_sent = Instant::now();
            }
            interval.tick().await;
        }
    }
}

#[get("/sse/speed")]
pub fn speed_event() -> EventStream![] {
    EventStream! {
        let mut interval = time::interval(Duration::from_millis(40));
        let mut last_sent = *GLOBAL_STATE.get().unwrap().last_change.speed.read().unwrap() - Duration::from_millis(1);
        loop {
            let last_change = *GLOBAL_STATE.get().unwrap().last_change.speed.read().unwrap();
            if last_change > last_sent {
                let speed = *GLOBAL_STATE.get().unwrap().speed.read().unwrap();
                yield Event::json(&speed);
                last_sent = Instant::now();
            }
            interval.tick().await;
        }
    }
}

#[get("/sse/state")]
pub fn state_event() -> EventStream![] {
    EventStream! {
        let mut interval = time::interval(Duration::from_millis(40));
        let mut last_sent = *GLOBAL_STATE.get().unwrap().last_change.state.read().unwrap() - Duration::from_millis(1);
        loop {
            let last_change = *GLOBAL_STATE.get().unwrap().last_change.state.read().unwrap();
            if last_change > last_sent {
                yield Event::json(&GLOBAL_STATE.get().unwrap().get_state());
                last_sent = Instant::now();
            }
            interval.tick().await;
        }
    }
}

/*###############*\
##   /api/ctrl   ##
\*###############*/

#[put("/ctrl")]
pub fn enable_control() {
    GLOBAL_STATE.get().unwrap().send_hw(HWCmd::StartControl);
}

#[delete("/ctrl")]
pub fn disable_control() {
    GLOBAL_STATE.get().unwrap().send_hw(HWCmd::EndControl);
}

/// start / unpause interpreter
/// 
/// * if idle, a new run will be started
/// * if paused, the active run will be unpaused
/// 
/// otherwise, an error is returned
#[post("/ctrl/start")]
pub fn start() -> BFRes {
    let glob = GLOBAL_STATE.get().unwrap();
    let mut state = glob.state.write().unwrap();
    match *state {
        ItpState::Idle => {
            drop(state);
            glob.send_hw(HWCmd::StartRun(false));
            Ok(())
        }
        ItpState::Startup => Err(BFError::StillStarting),
        ItpState::Running { ref mut paused, .. } => {
            if *paused {
                *paused = false;
                Ok(())
            } else {
                Err(BFError::ItpRunning)
            }
        }
        ItpState::Uncontrolled(_) => Err(BFError::ItpUncontrolled),
    }
}

#[post("/ctrl/pause")]
pub fn pause() -> BFRes {
    let glob = GLOBAL_STATE.get().unwrap();
    match *glob.state.write().unwrap() {
        ItpState::Idle => Err(BFError::ItpNotRunning),
        ItpState::Startup => Err(BFError::ItpNotRunning),
        ItpState::Running { ref mut paused, .. } => {
            if !*paused {
                *paused = true;
                Ok(())
            } else {
                Err(BFError::ItpNotRunning)
            }
        }
        ItpState::Uncontrolled(_) => Err(BFError::ItpUncontrolled),
    }
}

/// execute steps
/// 
/// if no number of steps is given, 1 will be used as a default.
/// 
/// * if idle, a new run will be started and immediately paused.
/// additionally, as many steps as given will be executed.
/// * if paused, the steps will be executed
/// 
/// otherwise, an error is returned
#[post("/ctrl/step", data = "<steps>")]
pub fn step(steps: Option<String>) -> BFRes {
    let steps: usize = steps.map(|n| n.parse().ok()).flatten().unwrap_or(1);
    let glob = GLOBAL_STATE.get().unwrap();
    match *glob.state.read().unwrap() {
        ItpState::Idle => {
            glob.send_hw(HWCmd::StartRun(true));
            for _ in 0..steps {
                glob.send_hw(HWCmd::ExecStep);
            }
            Ok(())
        }
        ItpState::Startup => Err(BFError::StillStarting),
        ItpState::Running { ref paused, .. } => {
            if *paused {
                for _ in 0..steps {
                    glob.send_hw(HWCmd::ExecStep);
                }
                Ok(())
            } else {
                Err(BFError::ItpRunning)
            }
        }
        ItpState::Uncontrolled(_) => Err(BFError::ItpUncontrolled),
    }
}

#[post("/ctrl/reset")]
pub fn reset() -> BFRes {
    let glob = GLOBAL_STATE.get().unwrap();
    match *glob.state.read().unwrap() {
        ItpState::Idle => Ok(()),
        ItpState::Startup => Err(BFError::StillStarting),
        ItpState::Running { .. } => {
            glob.send_hw(HWCmd::Reset);
            Ok(())
        }
        ItpState::Uncontrolled(_) => Err(BFError::ItpUncontrolled),
    }
}

/*###########*\
##   other   ##
\*###########*/

#[get("/examples")]
async fn get_examples() -> Option<NamedFile> {
    NamedFile::open("examples.json").await.ok()
}
