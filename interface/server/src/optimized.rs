use std::{fmt::Display, io::Write as _};

use super::{parse, BFCommand, TAPE_LEN};

pub struct Prog {
    pub code: Vec<BFCommandOpt>,
    input: bool,
    pub len: usize,
}

impl Prog {
    pub fn new(code: &str) -> Option<Prog> {
        let parsed = parse(code)?;
        let input = parsed.contains(&BFCommand::In);
        let len = parsed.len();
        Some(Prog {
            code: optimise(parsed),
            input,
            len,
        })
    }

    pub fn has_input(&self) -> bool {
        self.input
    }

    pub fn run(&self, inp: &str) -> (usize, Vec<u8>) {
        run(inp, &self.code)
    }

    pub fn run_full(&self) {
        let mut inp = String::new();
        if self.has_input() {
            print!("Input: ");
            std::io::stdout().flush().expect("failed to flush");
            std::io::stdin()
                .read_line(&mut inp)
                .expect("failed to get user input");
        }
        let (_, out) = self.run(&inp);
        let out = String::from_utf8(out).unwrap_or("output contains invalid utf8".to_string());
        println!("{out}");
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum BFCommandOpt {
    Inc(u8),
    Dec(u8),
    Left(usize),
    Right(usize),
    In,
    Out,
    LoopStart,
    LoopEnd,
    SetZero,
    ArithLoop(Vec<(isize, u8)>, usize),
}

impl Display for BFCommandOpt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BFCommandOpt::Inc(by) => write!(f, "{}", "+".repeat(*by as usize)),
            BFCommandOpt::Dec(by) => write!(f, "{}", "-".repeat(*by as usize)),
            BFCommandOpt::Left(by) => write!(f, "{}", "<".repeat(*by)),
            BFCommandOpt::Right(by) => write!(f, "{}", ">".repeat(*by)),
            BFCommandOpt::In => write!(f, ","),
            BFCommandOpt::Out => write!(f, "."),
            BFCommandOpt::LoopStart => write!(f, "["),
            BFCommandOpt::LoopEnd => write!(f, "]"),
            BFCommandOpt::SetZero => write!(f, "{{_}}"),
            BFCommandOpt::ArithLoop(vec, cycles) => write!(
                f,
                "{{[{}], {}}}",
                vec.iter()
                    .map(|x| format!("({}, {})", x.0, x.1))
                    .reduce(|x, y| format!("{x}, {y}"))
                    .unwrap_or_default(),
                cycles
            ),
        }
    }
}

impl From<BFCommand> for BFCommandOpt {
    fn from(value: BFCommand) -> Self {
        match value {
            BFCommand::Inc => BFCommandOpt::Inc(1),
            BFCommand::Dec => BFCommandOpt::Dec(1),
            BFCommand::Left => BFCommandOpt::Left(1),
            BFCommand::Right => BFCommandOpt::Right(1),
            BFCommand::In => BFCommandOpt::In,
            BFCommand::Out => BFCommandOpt::Out,
            BFCommand::LoopStart => BFCommandOpt::LoopStart,
            BFCommand::LoopEnd => BFCommandOpt::LoopEnd,
        }
    }
}

pub fn optimise(code: Vec<BFCommand>) -> Vec<BFCommandOpt> {
    let mut res = Vec::new();
    let mut code = code.into_iter().peekable();
    let Some(first) = code.next() else {
        return Vec::new();
    };
    let mut curr: BFCommandOpt = first.into();
    while let Some(cmd) = code.next() {
        match &mut curr {
            BFCommandOpt::Inc(by) => {
                if cmd == BFCommand::Inc {
                    *by += 1;
                    continue;
                }
            }
            BFCommandOpt::Dec(by) => {
                if cmd == BFCommand::Dec {
                    *by += 1;
                    continue;
                }
            }
            BFCommandOpt::Left(by) => {
                if cmd == BFCommand::Left {
                    *by += 1;
                    continue;
                }
            }
            BFCommandOpt::Right(by) => {
                if cmd == BFCommand::Right {
                    *by += 1;
                    continue;
                }
            }
            BFCommandOpt::LoopStart => {
                //res.push(curr);
                let mut inner = vec![cmd];
                let mut success = false;
                while let Some(cmd) = code.peek() {
                    match cmd {
                        BFCommand::LoopEnd => {
                            success = true;
                            break;
                        }
                        BFCommand::LoopStart => break,
                        _ => inner.push(code.next().unwrap()), // unwrap: peeked successfully
                    }
                }
                let mut inner_opt = optimise(inner);
                if success {
                    if let Some(arith) = arithmetic_loop(&inner_opt) {
                        res.push(arith);
                        // consume LoopEnd
                        code.next();
                    } else {
                        res.push(curr);
                        res.append(&mut inner_opt);
                    }
                } else {
                    res.push(curr);
                    res.append(&mut inner_opt);
                }
                let Some(next) = code.next() else {
                    return res;
                };
                curr = next.into();
                continue;
            }
            _ => {}
        }
        res.push(curr);
        curr = cmd.into();
    }
    res.push(curr);
    res
}

fn arithmetic_loop(code: &[BFCommandOpt]) -> Option<BFCommandOpt> {
    let mut idx = 0_isize;
    let mut parts: Vec<(isize, u8)> = Vec::new();
    let mut cycles = 1;
    for cmd in code {
        match cmd {
            BFCommandOpt::Inc(by) => {
                if let Some(p) = parts.iter_mut().find(|(i, _)| idx == *i) {
                    p.1 = p.1.wrapping_add(*by);
                } else {
                    parts.push((idx, *by));
                }
                cycles += *by as usize;
            }
            BFCommandOpt::Dec(by) => {
                if let Some(p) = parts.iter_mut().find(|(i, _)| idx == *i) {
                    p.1 = p.1.wrapping_sub(*by);
                } else {
                    parts.push((idx, 0_u8.wrapping_sub(*by)));
                }
                cycles += *by as usize;
            }
            BFCommandOpt::Left(by) => {
                idx -= *by as isize;
                cycles += *by;
            }
            BFCommandOpt::Right(by) => {
                idx += *by as isize;
                cycles += *by;
            }
            _ => return None,
        }
    }
    if idx != 0 {
        return None;
    }
    let old_len = parts.len();
    parts.retain(|p| *p != (0, u8::MAX));
    if old_len != parts.len() + 1 {
        None
    } else if parts.is_empty() && cycles == 2 {
        Some(BFCommandOpt::SetZero)
    } else {
        Some(BFCommandOpt::ArithLoop(parts, cycles))
    }
}

pub fn run(inp: &str, prog: &[BFCommandOpt]) -> (usize, Vec<u8>) {
    // prepare jump table
    let mut stack: Vec<usize> = Vec::new();
    let mut jmp_table: Vec<_> = prog.iter().map(|_| 0).collect();
    for i in 0..prog.len() {
        match prog[i] {
            BFCommandOpt::LoopStart => stack.push(i),
            BFCommandOpt::LoopEnd => {
                let other = stack.pop().expect("invalid program");
                jmp_table[other] = i;
                jmp_table[i] = other;
            }
            _ => {}
        }
    }
    // run code
    let mut inp = inp.as_bytes().iter();
    let mut out = Vec::new();
    let mut tape = [0_u8; TAPE_LEN];
    let mut pc = 0_usize;
    let mut head = 0_usize;
    let mut cycles = 0_usize;
    let mut real_cycles = 0_usize;
    while pc < prog.len() {
        cycles += 1;
        real_cycles += 1;
        match prog[pc] {
            BFCommandOpt::Inc(by) => {
                tape[head] = tape[head].wrapping_add(by);
                real_cycles += by as usize - 1;
            }
            BFCommandOpt::Dec(by) => {
                tape[head] = tape[head].wrapping_sub(by);
                real_cycles += by as usize - 1;
            }
            BFCommandOpt::Left(by) => {
                head = (head.wrapping_sub(by)) % TAPE_LEN;
                real_cycles += by - 1;
            }
            BFCommandOpt::Right(by) => {
                head = (head.wrapping_add(by)) % TAPE_LEN;
                real_cycles += by - 1;
            }
            BFCommandOpt::In => tape[head] = *inp.next().unwrap_or(&0),
            BFCommandOpt::Out => out.push(tape[head]),
            BFCommandOpt::LoopStart => {
                if tape[head] == 0 {
                    pc = jmp_table[pc];
                }
            }
            BFCommandOpt::LoopEnd => {
                if tape[head] != 0 {
                    pc = jmp_table[pc];
                }
            }
            BFCommandOpt::SetZero => {
                real_cycles += 2 * tape[head] as usize;
                tape[head] = 0;
            }
            BFCommandOpt::ArithLoop(ref parts, cycles) => {
                real_cycles += cycles * tape[head] as usize;
                for (offset, value) in parts {
                    let idx = head.wrapping_add_signed(*offset) % TAPE_LEN;
                    tape[idx] = tape[idx].wrapping_add(value.wrapping_mul(tape[head]));
                }
                tape[head] = 0;
            }
        }
        pc += 1;
    }
    println!("cycles: {cycles} / {real_cycles}");
    (real_cycles, out)
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_arith_loop(inp: &str, exp: &str) {
        let code = optimise(parse(inp).expect("invalid program"));
        let disp = code
            .iter()
            .map(<BFCommandOpt as ToString>::to_string)
            .reduce(|acc, e| acc + &e)
            .unwrap();
        assert_eq!(&disp, exp);
    }

    #[test]
    fn arith_loops() {
        test_arith_loop("[->>+++<<<+>]", "{[(2, 3), (-1, 1)], 12}");
        test_arith_loop("[->++++[->++++<]<]", "[->++++{[(1, 4)], 8}<]");
        test_arith_loop("[<+>[-]>]", "[<+>{_}>]");
    }
}
