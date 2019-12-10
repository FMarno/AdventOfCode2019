use std::fs;
use std::collections::VecDeque;
use permutohedron::heap_recursive;

fn main() {
    let mut inputs : Vec<_> = (5..10).collect();
    let mut permutations = Vec::new();
    heap_recursive(&mut inputs,|x| permutations.push(x.to_owned()));
    let input = fs::read_to_string("seven/input").expect("can't open file");
    let codes : Vec<i64> = input.trim().split(",").map(|x| x.parse::<i64>().unwrap()).collect();
    /*let m = permutations.iter()
        .map(
            |x| thruster_signal(x.to_vec(), codes.to_vec())
        )
        .max();
    println!("{:?}",m.expect("fuck"));*/
    println!("{}", thruster_signal(vec!(9,8,7,6,5), codes));
}

fn thruster_signal(input : Vec<i64>, codes : Vec<i64>) -> i64 {
    let mut amps : Vec<_> = input.iter().map(|x|{
        let mut inputs : VecDeque<i64> = VecDeque::new();
        inputs.push_back(*x);
        Amp::new(codes.to_vec(), inputs)
    })
    .collect();
    let mut output = 0;
    for i in (0..amps.len()).cycle() {
        amps[i].input.push_back(output);
        match amps[i].run_codes(){
            Some(x) => output = x,
            None => break,
        }
    }
    output
}

struct Amp {
    codes : Vec<i64>,
    pointer : usize,
    input : VecDeque<i64>,
    relative_base : usize,
}

#[derive(Debug)]
enum OpCode {
    Add,
    Mult,
    Load,
    Save,
    Halt,
    JumpTrue,
    JumpFalse,
    LessThan,
    Equals,
    Error(i64),
}

#[derive(Debug)]
enum Mode {
    Immediate,
    Position,
    Relative
}

impl Amp {
    fn new(codes: Vec<i64>, input :VecDeque<i64>) -> Amp {
        Amp{codes, pointer : 0, input, relative_base: 0}
    }

    fn run_codes(&mut self) -> Option<i64> {
        use OpCode::*;

        loop{
            //thread::sleep(time::Duration::from_millis(100));
            println!("{} {:?}",self.pointer, self.codes);
            let code = self.get_argument(Mode::Immediate);
            let (mode3, mode2, mode1, instruction) = decode_op(code);
            println!("{:?} {:?} {:?} {:?}", mode3, mode2, mode1, instruction);
            match instruction {
                Add => {
                    // sum ptr+1 and ptr+2 and save to ptr+3
                    let l = self.get_argument(mode1);
                    let r = self.get_argument(mode2);
                    let o = self.get_argument(Mode::Immediate);
                    self.codes[o as usize] = l + r;
                },
                Mult => {
                    // multiply ptr+1 and ptr+2 and save to ptr+3
                    let l = self.get_argument(mode1);
                    let r = self.get_argument(mode2);
                    let o = self.get_argument(Mode::Immediate);
                    self.codes[o as usize] = l*r;
                },
                Load => {
                    // save input to self.pointer+1
                    let location = self.get_argument(Mode::Immediate);
                    self.codes[location as usize] = self.input.pop_front().unwrap();
                },
                Save => {
                    // save self.pointer+1 to output
                    let location = self.get_argument(mode1);
                    return Some(location);
                },
                JumpTrue => {
                    let test = self.get_argument(mode1);
                    let location = self.get_argument(mode2);
                    if test != 0 {
                        self.pointer = location as usize;
                    }
                },
                JumpFalse => {
                    let test = self.get_argument(mode1);
                    let location = self.get_argument(mode2);
                    if test == 0 {
                        self.pointer = location as usize;
                    }
                },
                LessThan => {
                    let l = self.get_argument(mode1);
                    let r = self.get_argument(mode2);
                    let o = self.get_argument(Mode::Immediate);
                    self.codes[o as usize] = if l < r { 1} else {0};
                },
                Equals => {
                    let l = self.get_argument(mode1);
                    let r = self.get_argument(mode2);
                    let o = self.get_argument(Mode::Immediate);
                    self.codes[o as usize] = if l == r { 1} else {0};
                },
                Halt => return None,
                Error(e) => {
                    println!("Unexpected code! {}" ,e);
                    return None;
                }
            }
        }
    }        

    fn get_argument(&mut self, mode : Mode) -> i64 {
        use Mode::*;
        let out = match mode {
            Position =>self.codes[self.codes[self.pointer] as usize],
            Immediate => self.codes[self.pointer],
            Relative =>self.codes[self.relative_base+(self.codes[self.pointer] as usize)],
        };
        self.pointer+=1;
        out
    }
}

fn decode_op(op : i64) -> (Mode,Mode,Mode,OpCode) {
    use OpCode::*;

    let ten_thousands = op /10000;
    let remainder = op % 10000;
    let thousands = remainder /1000;
    let remainder = remainder % 1000;
    let hundreds = remainder /100;
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
        99 => Halt,
        x => Error(x),
    };
    (to_mode(ten_thousands), to_mode(thousands), to_mode(hundreds), instruction)
}

fn to_mode(value : i64) -> Mode {
    match value {
        0 => Mode::Position,
        1 => Mode::Immediate,
        2 => Mode::Relative,
        _ => panic!("unexpected mode value"),
    }
}