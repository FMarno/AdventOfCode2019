use colored::*;
use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs;

fn main() {
    let args: Vec<_> = env::args().collect();
    let filename = &args[1];
    let input = fs::read_to_string(filename).expect("can't open file");
    let codes: Vec<i64> = input
        .trim()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let mut input = VecDeque::new();
    input.push_back(1);
    let mut amp = Amp::new(codes, input);
    loop {
        match amp.run_codes() {
            Some(()) => (),
            None => break,
        }
    }
    for x in amp.output {
        print!("{} ", x);
    }
}

struct Amp {
    codes: Vec<i64>,
    pointer: usize,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    relative_base: i64,
    disk: HashMap<usize, i64>,
}

#[derive(Debug)]
enum OpCode {
    Add,
    Mult,
    Load,
    Save,
    JumpTrue,
    JumpFalse,
    LessThan,
    Equals,
    AdjustBase,
    Halt,
    Error(i64),
}

#[derive(Debug)]
enum Mode {
    Immediate,
    Position,
    Relative,
}

impl Amp {
    fn new(codes: Vec<i64>, input: VecDeque<i64>) -> Amp {
        Amp {
            codes,
            pointer: 0,
            input,
            output: VecDeque::new(),
            relative_base: 0,
            disk: HashMap::new(),
        }
    }

    fn run_codes(&mut self) -> Option<()> {
        loop {
            println!("ptr:{} r-ptr:{}", self.pointer, self.relative_base);
            print_codes(&self.codes, self.pointer, self.relative_base);
            let code = self.get_argument(Mode::Immediate);
            let (_mode3, mode2, mode1, instruction) = decode_op(code);
            println!("{:?} {:?} {:?} {:?}", instruction, mode1, mode2, _mode3);
            match instruction {
                OpCode::Add => {
                    // sum ptr+1 and ptr+2 and save to ptr+3
                    let l = self.get_argument(mode1);
                    let r = self.get_argument(mode2);
                    let o = self.get_argument(Mode::Immediate);
                    println!("{} {} {}", l, r, o);
                    self.set_memory(o as usize, l + r);
                }
                OpCode::Mult => {
                    // multiply ptr+1 and ptr+2 and save to ptr+3
                    let l = self.get_argument(mode1);
                    let r = self.get_argument(mode2);
                    let o = self.get_argument(Mode::Immediate);
                    println!("{} {} {}", l, r, o);
                    self.set_memory(o as usize, l * r);
                }
                OpCode::Load => {
                    // save input to self.pointer+1
                    let location = self.get_argument(Mode::Immediate);
                    let input = self.input.pop_front().unwrap();
                    println!("{} {}", location, input);
                    self.set_memory(location as usize, input);
                }
                OpCode::Save => {
                    // save self.pointer+1 to output
                    let value = self.get_argument(mode1);
                    println!("{}", value);
                    self.output.push_back(value);
                }
                OpCode::JumpTrue => {
                    let test = self.get_argument(mode1);
                    let location = self.get_argument(mode2);
                    println!("{} {}", test, location);
                    if test != 0 {
                        self.pointer = location as usize;
                    }
                }
                OpCode::JumpFalse => {
                    let test = self.get_argument(mode1);
                    let location = self.get_argument(mode2);
                    println!("{} {}", test, location);
                    if test == 0 {
                        self.pointer = location as usize;
                    }
                }
                OpCode::LessThan => {
                    let l = self.get_argument(mode1);
                    let r = self.get_argument(mode2);
                    let o = self.get_argument(Mode::Immediate);
                    println!("{} {} {}", l, r, o);
                    self.set_memory(o as usize, if l < r { 1 } else { 0 });
                }
                OpCode::Equals => {
                    let l = self.get_argument(mode1);
                    let r = self.get_argument(mode2);
                    let o = self.get_argument(Mode::Immediate);
                    println!("{} {} {}", l, r, o);
                    self.set_memory(o as usize, if l == r { 1 } else { 0 });
                }
                OpCode::AdjustBase => {
                    let v = self.get_argument(mode1);
                    println!("{}", v);
                    self.relative_base += v;
                }
                OpCode::Halt => return None,
                OpCode::Error(e) => {
                    println!("Unexpected code! {}", e);
                    return None;
                }
            }
        }
    }

    fn set_memory(&mut self, ptr: usize, v: i64) {
        if ptr < self.codes.len() {
            self.codes[ptr] = v;
        } else {
            self.disk.insert(ptr, v);
        }
    }

    fn get_memory(&self, ptr: usize) -> i64 {
        if ptr < self.codes.len() {
            self.codes[ptr]
        } else {
            match self.disk.get(&ptr) {
                Some(x) => *x,
                None => 0,
            }
        }
    }

    fn get_argument(&mut self, mode: Mode) -> i64 {
        use Mode::*;
        let out = match mode {
            Position => self.get_memory(self.get_memory(self.pointer) as usize),
            Immediate => self.get_memory(self.pointer),
            Relative => {
                self.get_memory((self.get_memory(self.pointer) + self.relative_base) as usize)
            }
        };
        self.pointer += 1;
        out
    }
}

fn decode_op(op: i64) -> (Mode, Mode, Mode, OpCode) {
    use OpCode::*;

    let ten_thousands = op / 10000;
    let remainder = op % 10000;
    let thousands = remainder / 1000;
    let remainder = remainder % 1000;
    let hundreds = remainder / 100;
    let instruction = remainder % 100;
    let instruction = match instruction {
        1 => Add,
        2 => Mult,
        3 => Load,
        4 => Save,
        5 => JumpTrue,
        6 => JumpFalse,
        7 => LessThan,
        8 => Equals,
        9 => AdjustBase,
        99 => Halt,
        x => Error(x),
    };
    (
        to_mode(ten_thousands),
        to_mode(thousands),
        to_mode(hundreds),
        instruction,
    )
}

fn to_mode(value: i64) -> Mode {
    match value {
        0 => Mode::Position,
        1 => Mode::Immediate,
        2 => Mode::Relative,
        _ => panic!("unexpected mode value"),
    }
}

fn print_codes(codes: &Vec<i64>, pointer: usize, relative_base: i64) {
    for c in 0..codes.len() {
        if c == pointer {
            print!("{} ", (codes[c].to_string()).red());
        } else if c == relative_base as usize {
            print!("{} ", (codes[c].to_string()).green());
        } else {
            print!("{} ", codes[c]);
        }
    }
    println!();
}
