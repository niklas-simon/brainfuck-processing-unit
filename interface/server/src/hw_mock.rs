// this is a mock, unused can be safely ignored
#![allow(unused)]
use std::{sync::mpsc::Receiver, thread::{self, JoinHandle}, time::{Duration, Instant}};

use bf_itp::Run;

use crate::{HWCmd, HWState, ItpState, GLOBAL_STATE};

pub fn start_hw_thread(tx: Receiver<HWCmd>) -> JoinHandle<()> {
    thread::spawn(move || {
        let glob = GLOBAL_STATE.get().unwrap();
        loop {
            while let Ok(cmd) = tx.try_recv() {
                handle_cmd(cmd);
            }
            let gstate = glob.state.read().unwrap();
            match *gstate {
                ItpState::Running { paused, .. } if !paused => {
                    drop(gstate);
                    handle_cmd(HWCmd::ExecStep);
                    let tick = Duration::from_secs_f64(1.0 / glob.speed.read().unwrap().pow(3) as f64);
                    thread::sleep(tick);
                },
                // if not mocked, i/o handler needs to be active when in uncontrolled state
                _ => thread::sleep(Duration::from_millis(40)),
            }
        }
    })
}

fn handle_cmd(cmd: HWCmd) {
    let glob = GLOBAL_STATE.get().unwrap();
    match cmd {
        HWCmd::EndControl => {
            *glob.hw_state.write().unwrap() = HWState::Regular;
            glob.set_state(ItpState::Uncontrolled(0));
        },
        HWCmd::StartControl => {
            *glob.hw_state.write().unwrap() = HWState::Regular;
            glob.set_state(ItpState::Idle);
        },
        HWCmd::Program => {
            // mocked
        },
        HWCmd::StartRun => {
            // control -> low
            // program()
            // control -> high
            // control reset -> high -> low
            *glob.hw_state.write().unwrap() = HWState::Startup;
            *glob.last_change.state.write().unwrap() = Instant::now();
            // wait for arbitrary startup
            thread::sleep(Duration::from_secs(3));
            glob.itp_started(false);
        },
        HWCmd::StartRunPaused => {
            *glob.hw_state.write().unwrap() = HWState::Startup;
            *glob.last_change.state.write().unwrap() = Instant::now();
            // wait for arbitrary startup
            thread::sleep(Duration::from_secs(3));
            glob.itp_started(true);
        },
        HWCmd::ExecStep => {
            //println!("step executing");
            // io betrachten
            // control clock -> high -> low
            // schauen ob neuer i/o state existiert
            let mut state = glob.state.write().unwrap();
            //println!("lock acquired");
            match *state {
                ItpState::Running { ref mut run, .. } => {
                    //println!("step 1");
                    let old_out_len = run.out.len();
                    let finished = run.step();
                    //println!("step 2");
                    *glob.last_change.state.write().unwrap() = Instant::now();
                    //println!("step 3");
                    if old_out_len != run.out.len() {
                        if let Ok(out) = String::from_utf8(run.out.clone()) {
                            glob.set_output(out);
                        }
                    }
                    //println!("step 4");
                    drop(state);
                    if finished {
                        *glob.hw_state.write().unwrap() = HWState::Regular;
                        glob.set_state(ItpState::Idle); 
                        println!("run finished");
                    }
                    //println!("step 5");
                },
                _ => eprintln!("cannot execute step if itp is not running"),
            }
        },
        HWCmd::Reset => {
            // control reset -> high -> low
            glob.set_state(ItpState::Idle);
        },
    }
}