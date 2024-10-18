use std::time::{Duration, Instant};

use rocket::{delete, get, post, put, response::stream::{Event, EventStream}, tokio::time, Route};

use crate::{ItpState, GLOBAL_STATE};

pub fn get_routes() -> Vec<Route> {
    rocket::routes![set_speed_auto, set_speed_manual, event_stream, step, set_code, set_input, get_code, get_input, get_state, reset]
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
            if let Some(evt_data) = super::get_urls_to_fetch(&last_sent) {
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