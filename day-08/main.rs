use std::collections::BTreeSet;
use std::io::{self, BufRead};

#[derive(Clone, Debug)]
enum Op {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl Op {
    fn parse(raw: &str) -> Result<Op, String> {
        let mut parts = raw.split_whitespace();

        let err1 = || format!("Unexpected input: {}", raw);
        let err2 = |_| format!("Unexpected input: {}", raw);

        match parts.next().ok_or_else(err1)? {
            op if op == "acc" || op == "jmp" || op == "nop" => {
                let num = parts
                    .next()
                    .ok_or_else(err1)?
                    .parse::<i32>()
                    .map_err(err2)?;

                match op {
                    "acc" => Ok(Op::Acc(num)),
                    "jmp" => Ok(Op::Jmp(num)),
                    "nop" => Ok(Op::Nop(num)),
                    _ => unreachable!(),
                }
            }
            _ => return Err(err1()),
        }
    }
}

type Program = Vec<Op>;

#[derive(Debug)]
enum Output {
    Success(i32),
    Cycle(i32),
}

fn run(program: &Program) -> Output {
    let mut pc: usize = 0;
    let mut acc: i32 = 0;
    let mut visited = BTreeSet::new();

    loop {
        if pc == program.len() {
            return Output::Success(acc);
        }

        if visited.contains(&pc) {
            return Output::Cycle(acc);
        }

        visited.insert(pc);

        match program[pc] {
            Op::Acc(n) => {
                acc += n;
                pc += 1;
            }
            Op::Jmp(n) => pc = (pc as i32 + n) as usize,
            Op::Nop(_) => pc += 1,
        }
    }
}

fn iter_corrections(program: Program) -> impl Iterator<Item = Program> {
    let mut pc = 0;

    std::iter::from_fn(move || {
        let mut program = program.clone();

        while pc < program.len() {
            match program[pc] {
                Op::Acc(_) => pc += 1,
                Op::Jmp(n) => {
                    program[pc] = Op::Nop(n);
                    pc += 1;
                    return Some(program);
                }
                Op::Nop(n) => {
                    program[pc] = Op::Jmp(n);
                    pc += 1;
                    return Some(program);
                }
            }
        }

        None
    })
}

fn main() {
    let stdin = io::stdin();

    let program: Program = stdin
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|l| Op::parse(&l))
        .map(Result::unwrap)
        .collect();

    println!("puzzle #1 = {:?}", run(&program));

    for program in iter_corrections(program) {
        if let Output::Success(acc) = run(&program) {
            println!("puzzle #2 = {:?}", acc);
            break;
        }
    }
}
