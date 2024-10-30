use std::{
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use bf_itp::BFCommand;

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
                    handle_cmd(HWCmd::ExecStep(1, false));
                    let dur = speed_tick(*glob.speed.read().unwrap());
                    thread::sleep(dur);
                    let mut state = glob.state.write().unwrap();
                    let ItpState::Running { ref mut run, .. } = *state else {
                        thread::sleep(dur);
                        continue;
                    };
                    run.pc += 1;
                    drop(state);
                    *glob.last_change.state.write().unwrap() = Instant::now();
                    thread::sleep(dur);
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
    Duration::from_secs_f64(0.5 / 1_000_000.0_f64.powf((speed as f64 - 1.0) / 99.0))
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
        HWCmd::ExecStep(count, inc_pc) => {
            // HW: io betrachten
            // HW: control clock -> high -> low
            // HW: schauen ob neuer i/o state existiert
            let mut state = glob.state.write().unwrap();
            for i in 0..count {
                match *state {
                    ItpState::Running { ref mut run, .. } => {
                        let old_out_len = run.out.len();
                        if run.jumping.is_none() && run.pc < run.code.len() {
                            let hw_state = match run.code[run.pc] {
                                BFCommand::In => HWState::WaitInput,
                                BFCommand::Out => HWState::OutputReady,
                                _ => HWState::Regular, 
                            };
                            *glob.hw_state.write().unwrap() = hw_state;
                        }
                        let finished = run.step();
                        if i + 1 < count || inc_pc {
                            run.pc += 1;
                        }
                        *glob.last_change.state.write().unwrap() = Instant::now();
                        if old_out_len != run.out.len() {
                            glob.set_output(String::from_utf8_lossy(&run.out).into_owned());
                        }
                        if finished {
                            drop(state);
                            *glob.hw_state.write().unwrap() = HWState::Regular;
                            glob.set_state(ItpState::Idle);
                            println!("run finished");
                            break;
                        }
                    }
                    _ => eprintln!("cannot execute step if itp is not running"),
                }
            }
        }
        HWCmd::Reset => {
            // control reset -> high -> low
            glob.set_state(ItpState::Idle);
        }
    }
}
