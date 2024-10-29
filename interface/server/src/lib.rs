use std::fmt::Display;

use serde::Serialize;

pub mod optimized;
pub mod skill;

/// A single brainfuck instruction
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

pub fn is_nesting_correct(code: &str) -> bool {
    let mut depth = 0;
    for c in code.chars() {
        if c == '[' {
            depth += 1;
        } else if c == ']' && depth == 0 {
            return false;
        } else if c == ']' {
            depth -= 1;
        }
    }
    depth == 0
}

/// parse a brainfuck program
///
/// converts a `&str` to a vec of [`BFCommand`]s, while also
/// removing any invalid chars and checking for correctly nested loops
pub fn parse(code: &str) -> Option<Vec<BFCommand>> {
    is_nesting_correct(code).then(|| code.chars().flat_map(lex).collect())
}

const TAPE_LEN: usize = 32768;

/// the digital twin
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

/// a view of the [digital twin](Run)
///
/// determines what the client gets to see
#[derive(Serialize)]
pub struct RunView {
    tape: Vec<u8>,
    head: usize,
    code: CodeView,
    ic: usize,
    jumping: Option<usize>,
    stack: Vec<usize>,
    cycles: usize,
    control_state: String,
    run_state: String,
}

/// a view of the currently executing code
///
/// part of the [`RunView`]
#[derive(Serialize)]
pub struct CodeView {
    pc: usize,
    offset: usize,
    fragment: String,
}

impl Run {
    /// view size for tape and code
    ///
    /// controlls how many items (code / tape) to the
    /// left and right from the head / program counter
    /// are sent to the client
    const VIEW_SIZE: usize = 3;

    pub fn new(code: &str, input: &str) -> Option<Self> {
        Some(Self {
            tape: [0; TAPE_LEN],
            pc: 0,
            ic: 0,
            head: 0,
            code: parse(code)?,
            jumping: None,
            stack: Vec::new(),
            inp: input.as_bytes().to_vec(),
            out: Vec::new(),
            cycles: 0,
        })
    }

    fn get_code_view(&self) -> CodeView {
        let start = Self::VIEW_SIZE.max(self.pc) - Self::VIEW_SIZE;
        let end = self.code.len().min(self.pc + Self::VIEW_SIZE + 1);
        let code_frag: String = self.code[start..end]
            .iter()
            .map(<BFCommand as ToString>::to_string)
            .collect();
        CodeView {
            fragment: code_frag,
            offset: start,
            pc: self.pc,
        }
    }

    pub fn view(&self, ctrl_state: &str, run_state: &str) -> RunView {
        let mut tape_slice = Vec::new();
        // conversion between isize and usize needed for correct wrapping
        for i in self.head as isize - Self::VIEW_SIZE as isize
            ..=self.head as isize + Self::VIEW_SIZE as isize
        {
            tape_slice.push(self.tape[i.rem_euclid(TAPE_LEN as isize) as usize]);
        }
        RunView {
            tape: tape_slice,
            code: self.get_code_view(),
            ic: self.ic,
            head: self.head,
            jumping: self.jumping,
            stack: self.stack.clone(),
            cycles: self.cycles,
            control_state: ctrl_state.to_string(),
            run_state: self.jumping.map(|_| "jumping").unwrap_or(run_state).to_string(),
        }
    }

    /// advance by one step
    ///
    /// returns true when finished
    pub fn step(&mut self) -> bool {
        // this should not happen
        if self.pc >= self.code.len() {
            return true;
        }
        self.cycles += 1;
        if let Some(depth) = &mut self.jumping {
            match self.code[self.pc] {
                BFCommand::LoopEnd => *depth -= 1,
                BFCommand::LoopStart => *depth += 1,
                _ => {}
            }
            if *depth == 0 {
                self.jumping = None;
            }
        } else {
            match self.code[self.pc] {
                BFCommand::Inc => self.tape[self.head] = self.tape[self.head].wrapping_add(1),
                BFCommand::Dec => self.tape[self.head] = self.tape[self.head].wrapping_sub(1),
                BFCommand::Left => self.head = (self.head.wrapping_sub(1)) % TAPE_LEN,
                BFCommand::Right => self.head = (self.head.wrapping_add(1)) % TAPE_LEN,
                BFCommand::In => {
                    if self.ic < self.inp.len() {
                        self.tape[self.head] = self.inp[self.ic];
                        self.ic += 1;
                    } else {
                        self.tape[self.head] = 0;
                    }
                }
                BFCommand::Out => self.out.push(self.tape[self.head]),
                BFCommand::LoopStart => {
                    if self.tape[self.head] == 0 {
                        self.jumping = Some(1);
                    } else {
                        self.stack.push(self.pc);
                    }
                }
                BFCommand::LoopEnd => {
                    // probably correct behaviour for incorrect code
                    // should actually never happen
                    let back_addr = self.stack.last().unwrap_or(&0);
                    if self.tape[self.head] != 0 {
                        self.pc = *back_addr;
                    } else {
                        self.stack.pop();
                    }
                }
            }
        }
        self.pc += 1;
        self.pc == self.code.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hello_world() {
        let code = "+++++++++++[>++++++>+++++++++>++++++++>++++>+++>+<<<<<<-]>++++++.>++.+++++++..+++.>>.>-.<<-.<.+++.------.--------.>>>+.>-.";
        let mut run = Run::new(code, "").expect("code should be valid");
        while !run.step() {}
        assert_eq!(
            "Hello, World!\n".to_string(),
            String::from_utf8(run.out).unwrap()
        );
        assert_eq!(run.cycles, 572);
    }
}
