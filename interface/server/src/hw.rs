use std::{
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use bf_itp::BFCommand;

use crate::{HWCmd, HWState, ItpState, GLOBAL_STATE};

macro_rules! raspi {
    ($($body:tt)*) => {
        #[cfg(all(target_arch = "aarch64", target_env = "gnu"))]
        $($body)*
    };
}

macro_rules! not_raspi {
    ($($body:tt)*) => {
        #[cfg(not(all(target_arch = "aarch64", target_env = "gnu")))]
        $($body)*
    };
}

raspi!(mod ports;);
raspi!(use ports::Ports;);
not_raspi! {
    struct Ports;
}

pub fn start_hw_thread(tx: Receiver<HWCmd>) -> JoinHandle<()> {
    raspi! {
        let mut ports = Ports::new().unwrap();
    }
    thread::spawn(move || {
        let glob = GLOBAL_STATE.get().unwrap();
        not_raspi! {
            let mut ports = Ports;
        }
        loop {
            // first, handle all user-inputted things to do
            while let Ok(cmd) = tx.try_recv() {
                handle_cmd(cmd, &mut ports);
            }
            // then, enter the regular loop
            let state = glob.state.read().unwrap();
            match *state {
                ItpState::Running { paused, .. } if !paused => {
                    // itp is currently running -> execute the next step and wait depending on the global speed
                    drop(state);
                    handle_cmd(HWCmd::ExecStep(1, false), &mut ports);
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
                ItpState::Uncontrolled(ic) => {
                    raspi! {{
                        drop(state);
                        if let Some(outp) = ports.handle_io(|| {
                            let inp = glob.input.read().unwrap();
                            let res = *inp.as_bytes().get(ic).unwrap_or(&0);
                            let l = inp.len();
                            drop(inp);
                            let mut state = glob.state.write().unwrap();
                            let ItpState::Uncontrolled(ref mut ic) = *state else {
                                return res;
                            };
                            *ic = (*ic + 1) % (l + 1);
                            drop(state);
                            res
                        }) {
                            glob.output.write().unwrap().push(outp as char);
                        }
                    }}
                    not_raspi! {{
                        // prevent unused warning when mocked
                        let _ = ic;
                        drop(state);
                        thread::sleep(Duration::from_millis(40))
                    }}
                }
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

fn handle_cmd(cmd: HWCmd, ports: &mut Ports) {
    not_raspi! {
        // prevent unused warning when mocked
        let _ = ports;
    }
    let glob = GLOBAL_STATE.get().unwrap();
    match cmd {
        HWCmd::EndControl => {
            raspi! {
                ports.control.set_control(false);
            }
            *glob.hw_state.write().unwrap() = HWState::Regular;
            glob.set_state(ItpState::Uncontrolled(0));
        }
        HWCmd::StartControl => {
            raspi! {
                ports.control.set_control(true);
            }
            *glob.hw_state.write().unwrap() = HWState::Regular;
            glob.set_state(ItpState::Idle);
        }
        HWCmd::Program => {
            raspi! {{
                ports.control.set_control(false);
                ports.program.write_program(&**glob.full_code.read().unwrap());
                ports.control.set_control(true);
            }}
        }
        HWCmd::StartRun(paused) => {
            raspi! {{
                ports.control.set_control(false);
                ports.program.write_program(&**glob.full_code.read().unwrap());
                ports.control.set_control(true);
                ports.control.reset();
            }}
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
                        raspi! {{
                            ports.handle_io(|| {
                                *run.inp.get(run.ic).unwrap_or(&0)
                            });
                            ports.control.step();
                        }}
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
