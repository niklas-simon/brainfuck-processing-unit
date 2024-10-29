use std::{
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use crate::{HWCmd, HWState, ItpState, GLOBAL_STATE};

pub fn start_hw_thread(tx: Receiver<HWCmd>) -> JoinHandle<()> {
    thread::spawn(move || {
        let glob = GLOBAL_STATE.get().unwrap();
        loop {
            // first, handle all user-inputted things to do
            while let Ok(cmd) = tx.try_recv() {
                handle_cmd(cmd);
            }
            // then, enter the regular loop
            let state = glob.state.read().unwrap();
            match *state {
                ItpState::Running { paused, .. } if !paused => {
                    // itp is currently running -> execute the next step and wait depending on the global speed
                    drop(state);
                    handle_cmd(HWCmd::ExecStep);
                    thread::sleep(speed_tick(*glob.speed.read().unwrap()));
                }
                // HW: if uncontrolled, i/o must be handled
                _ => {
                    // itp is not running -> wait for 40ms so the cpu can rest a bit
                    drop(state);
                    thread::sleep(Duration::from_millis(40))
                }
            }
        }
    })
}

fn speed_tick(speed: u8) -> Duration {
    Duration::from_secs_f64(1.0 / 1_000_000.0_f64.powf((speed as f64 - 1.0) / 99.0))
}

fn handle_cmd(cmd: HWCmd) {
    let glob = GLOBAL_STATE.get().unwrap();
    match cmd {
        HWCmd::EndControl => {
            *glob.hw_state.write().unwrap() = HWState::Regular;
            glob.set_state(ItpState::Uncontrolled(0));
        }
        HWCmd::StartControl => {
            *glob.hw_state.write().unwrap() = HWState::Regular;
            glob.set_state(ItpState::Idle);
        }
        HWCmd::Program => {
            // mocked
        }
        HWCmd::StartRun(paused) => {
            // HW: control -> low
            // HW: program()
            // HW: control -> high
            // HW: control reset -> high -> low
            glob.set_state(ItpState::Startup);
            // wait for arbitrary startup
            thread::sleep(Duration::from_secs(3));
            glob.itp_started(paused);
        }
        HWCmd::ExecStep => {
            // HW: io betrachten
            // HW: control clock -> high -> low
            // HW: schauen ob neuer i/o state existiert
            let mut state = glob.state.write().unwrap();
            match *state {
                ItpState::Running { ref mut run, .. } => {
                    let old_out_len = run.out.len();
                    let finished = run.step();
                    *glob.last_change.state.write().unwrap() = Instant::now();
                    if old_out_len != run.out.len() {
                        glob.set_output(String::from_utf8_lossy(&run.out).into_owned());
                    }
                    drop(state);
                    if finished {
                        *glob.hw_state.write().unwrap() = HWState::Regular;
                        glob.set_state(ItpState::Idle);
                        println!("run finished");
                    }
                }
                _ => eprintln!("cannot execute step if itp is not running"),
            }
        }
        HWCmd::Reset => {
            // control reset -> high -> low
            glob.set_state(ItpState::Idle);
        }
    }
}
