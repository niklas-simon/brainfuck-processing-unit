use std::{fmt::Display, io::Write as _};

use rand::prelude::*;
use serde::Serialize;

pub fn get_skill(code: &str, target: &str) -> f64 {
    let prog = Prog::new(code);
    let (cycles, output) = prog.run("");
    let out = String::from_utf8(output).unwrap_or("invalid utf-8".to_string());
    if out == target {
        let res = skill_value(prog.len, cycles);
        let def_len = count_default_len(target);
        let base = skill_value(def_len, def_len);
        100.0 * (base - res) / base
    } else {
        eprintln!("wrong output: expcted {target}, got {out}");
        0.0
    }
}

fn count_default_len(target: &str) -> usize {
    target.chars().fold((0, 0 as char), |acc, e| (acc.0 + 1 + (acc.1 as i32 - e as i32).abs() as usize, e)).0
}

const CODE_LEN_WEIGHT: f64 = 8.0;
const CYCLES_WEIGHT: f64 = 1.0;

pub fn skill_value(code_len: usize, cycles: usize) -> f64 {
    (code_len as f64).ln() * CODE_LEN_WEIGHT + (cycles as f64).ln() * CYCLES_WEIGHT
}

pub fn generate_target() -> String {
    let mut rng = thread_rng();
    let len = rng.gen_range(10..20);
    (0..len).map(|_| rng.gen_range('!'..='~')).collect()
}

pub struct Prog {
    pub code: Vec<BFCommandOpt>,
    input: bool,
    len: usize,
}

impl Prog {
    pub fn new(code: &str) -> Prog {
        let parsed = parse(code);
        let input = parsed.contains(&BFCommand::In);
        let len = parsed.len();
        Prog {
            code: optimise(parsed),
            input,
            len,
        }
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
            std::io::stdin().read_line(&mut inp).expect("failed to get user input");
        }
        let (_, out) = self.run(&inp);
        let out = String::from_utf8(out).unwrap_or("output contains invalid utf8".to_string());
        println!("{out}");
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BFCommand {
    Inc,
    Dec,
    Left,
    Right,
    In,
    Out,
    LoopStart,
    LoopEnd,
}

impl Display for BFCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BFCommand::Inc => write!(f, "+"),
            BFCommand::Dec => write!(f, "-"),
            BFCommand::Left => write!(f, "<"),
            BFCommand::Right => write!(f, ">"),
            BFCommand::In => write!(f, ","),
            BFCommand::Out => write!(f, "."),
            BFCommand::LoopStart => write!(f, "["),
            BFCommand::LoopEnd => write!(f, "]"),
        }
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
                vec.iter().map(|x| format!("({}, {})", x.0, x.1)).reduce(|x, y| format!("{x}, {y}")).unwrap_or_default(),
                cycles
            ),
        }
    }
}

fn lex(c: char) -> Option<BFCommand> {
    match c {
        '+' => Some(BFCommand::Inc),
        '-' => Some(BFCommand::Dec),
        '<' => Some(BFCommand::Left),
        '>' => Some(BFCommand::Right),
        ',' => Some(BFCommand::In),
        '.' => Some(BFCommand::Out),
        '[' => Some(BFCommand::LoopStart),
        ']' => Some(BFCommand::LoopEnd),
        _ => None,
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

pub fn parse(code: &str) -> Vec<BFCommand> {
    code.chars().flat_map(lex).collect()
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
            BFCommandOpt::Inc(by) => if cmd == BFCommand::Inc {
                *by += 1;
                continue;
            },
            BFCommandOpt::Dec(by) => if cmd == BFCommand::Dec {
                *by += 1;
                continue;
            },
            BFCommandOpt::Left(by) => if cmd == BFCommand::Left {
                *by += 1;
                continue;
            },
            BFCommandOpt::Right(by) => if cmd == BFCommand::Right {
                *by += 1;
                continue;
            },
            BFCommandOpt::LoopStart => {
                //res.push(curr);
                let mut inner = vec![cmd];
                let mut success = false;
                while let Some(cmd) = code.peek() {
                    match cmd {
                        BFCommand::LoopEnd => {
                            success = true;
                            break;
                        },
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
            },
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
            },
            BFCommandOpt::Dec(by) => {
                if let Some(p) = parts.iter_mut().find(|(i, _)| idx == *i) {
                    p.1 = p.1.wrapping_sub(*by);
                } else {
                    parts.push((idx, 0_u8.wrapping_sub(*by)));
                }
                cycles += *by as usize;
            },
            BFCommandOpt::Left(by) => {
                idx -= *by as isize;
                cycles += *by;
            },
            BFCommandOpt::Right(by)  => {
                idx += *by as isize;
                cycles += *by;
            },
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

const TAPE_LEN: usize = 32768;

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
            },
            _ => {},
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
            },
            BFCommandOpt::Dec(by) => {
                tape[head] = tape[head].wrapping_sub(by);
                real_cycles += by as usize - 1;
            },
            BFCommandOpt::Left(by) => {
                head = (head.wrapping_sub(by)) % TAPE_LEN;
                real_cycles += by - 1;
            },
            BFCommandOpt::Right(by) => {
                head = (head.wrapping_add(by)) % TAPE_LEN;
                real_cycles += by - 1;
            },
            BFCommandOpt::In => tape[head] = *inp.next().unwrap_or(&0),
            BFCommandOpt::Out => out.push(tape[head]),
            BFCommandOpt::LoopStart => if tape[head] == 0 {
                pc = jmp_table[pc];
            },
            BFCommandOpt::LoopEnd => if tape[head] != 0 {
                pc = jmp_table[pc];
            },
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

#[derive(Debug)]
pub struct Run {
    tape: [u8; TAPE_LEN],
    pc: usize,
    pub ic: usize,
    head: usize,
    code: Vec<BFCommand>,
    jumping: Option<usize>,
    stack: Vec<usize>,
    pub inp: Vec<u8>,
    pub out: Vec<u8>,
    cycles: usize,
}

#[derive(Serialize)]
pub struct RunState {
    tape: Vec<u8>,
    head: usize,
    code: CodeView,
    ic: usize,
    jumping: Option<usize>,
    stack: Vec<usize>,
    cycles: usize,
    control: String,
}

#[derive(Serialize)]
pub struct CodeView {
    pc: usize,
    offset: usize,
    fragment: String,
}

impl Run {
    const VIEW_SIZE: usize = 3;

    pub fn new(code: &str, input: &str) -> Self {
        Self {
            tape: [0; TAPE_LEN],
            pc: 0,
            ic: 0,
            head: 0,
            code: parse(code),
            jumping: None,
            stack: Vec::new(),
            inp: input.as_bytes().to_vec(),
            out: Vec::new(),
            cycles: 0,
        }
    }

    fn get_code_view(&self) -> CodeView {
        let start = Self::VIEW_SIZE.max(self.pc) - Self::VIEW_SIZE;
        let end = self.code.len().min(self.pc + Self::VIEW_SIZE + 1);
        let code_frag: String = self.code[start..end].iter().map(<BFCommand as ToString>::to_string).collect();
        CodeView {
            fragment: code_frag,
            offset: start,
            pc: self.pc,
        }
    }

    pub fn state(&self, ctrl: &str) -> RunState {
        let mut tape_slice = Vec::new();
        for i in self.head as isize - Self::VIEW_SIZE as isize..=self.head as isize + Self::VIEW_SIZE as isize {
            tape_slice.push(self.tape[i.rem_euclid(TAPE_LEN as isize) as usize]);
        }
        RunState {
            tape: tape_slice,
            code: self.get_code_view(),
            ic: self.ic,
            head: self.head,
            jumping: self.jumping,
            stack: self.stack.clone(),
            cycles: self.cycles,
            control: ctrl.to_string(),
        }
    }

    pub fn view_code(&self) -> String {
        self.code.iter().map(<BFCommand as ToString>::to_string).collect()
    }

    /// advance by one step
    /// 
    /// returns true when finished
    pub fn step(&mut self) -> bool {
        if self.cycles % 1000 == 0 {
            println!("cycle {}", self.cycles);
        }
        self.cycles += 1;
        if let Some(depth) = &mut self.jumping {
            match self.code[self.pc] {
                BFCommand::LoopEnd => *depth -= 1,
                BFCommand::LoopStart => *depth += 1,
                _ => {},
            }
            if *depth == 0 {
                self.jumping = None;
            }
            return false;
        }

        match self.code[self.pc] {
            BFCommand::Inc => self.tape[self.head] = self.tape[self.head].wrapping_add(1),
            BFCommand::Dec => self.tape[self.head] = self.tape[self.head].wrapping_sub(1),
            BFCommand::Left => self.head = (self.head.wrapping_sub(1)) % TAPE_LEN,
            BFCommand::Right => self.head = (self.head.wrapping_sub(1)) % TAPE_LEN,
            BFCommand::In => {
                if self.ic < self.inp.len() {
                    self.tape[self.head] = self.inp[self.ic];
                    self.ic += 1;
                } else {
                    self.tape[self.head] = 0;
                }
            },
            BFCommand::Out => self.out.push(self.tape[self.head]),
            BFCommand::LoopStart => if self.tape[self.head] == 0 {
                self.jumping = Some(1);
            } else {
                self.stack.push(self.pc);
            },
            BFCommand::LoopEnd => {
                // probably correct behaviour for incorrect code
                let back_addr = self.stack.pop().unwrap_or(0);
                if self.tape[self.head] != 0 {
                    self.pc = back_addr;
                }
            },
        }
        self.pc += 1;
        self.pc == self.code.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_arith_loop(inp: &str, exp: &str) {
        let code = optimise(parse(inp));
        let disp = code.iter().map(<BFCommandOpt as ToString>::to_string).reduce(|acc, e| acc + &e).unwrap();
        assert_eq!(&disp, exp);
    }

    #[test]
    fn arith_loops() {
        test_arith_loop("[->>+++<<<+>]", "{[(2, 3), (-1, 1)], 12}");
        test_arith_loop("[->++++[->++++<]<]", "[->++++{[(1, 4)], 8}<]");
        test_arith_loop("[<+>[-]>]", "[<+>{_}>]");
    }
}