use rand::prelude::*;

use crate::optimized::Prog;

pub fn get_skill(code: &str, target: &str) -> f64 {
    let Some(prog) = Prog::new(code) else {
        return 0.0;
    };
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
    target
        .chars()
        .fold((0, 0 as char), |acc, e| {
            (acc.0 + 1 + (acc.1 as i32 - e as i32).abs() as usize, e)
        })
        .0
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