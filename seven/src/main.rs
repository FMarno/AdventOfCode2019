use std::fs;
use std::collections::VecDeque;
use permutohedron::heap_recursive;

fn main() {
    let mut inputs : Vec<_> = (5..10).collect();
    let mut permutations = Vec::new();
    heap_recursive(&mut inputs,|x| permutations.push(x.to_owned()));
    let input = fs::read_to_string("seven/input").expect("can't open file");
    let codes : Vec<i32> = input.trim().split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    let m = permutations.iter()
        .map(
            |x| thruster_signal(x.to_vec(), codes.to_vec())
        )
        .max();
    println!("{:?}",m.expect("fuck"));
    //println!("{}", thruster_signal(vec!(9,8,7,6,5), codes));
}

fn thruster_signal(input : Vec<i32>, codes : Vec<i32>) -> i32 {
    let mut amps : Vec<_> = input.iter().map(|x|{
        let mut inputs : VecDeque<i32> = VecDeque::new();
        inputs.push_back(*x);
        Amp{codes:codes.to_vec(), pointer: 0, input:inputs}
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
    codes : Vec<i32>,
    pointer : usize,
    input : VecDeque<i32>,
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
    Error(i32),
}

impl Amp {
    fn run_codes(&mut self) -> Option<i32> {
        use OpCode::*;
        let codes = &mut self.codes;
        let input = &mut self.input;

        loop{
            //println!("{} {:?}",self.pointer, codes);
            let code = codes[self.pointer];
            let (_mode3, mode2, mode1, instruction) = decode_op(code);
            //println!("{} {} {} {:?}", _mode3, mode2, mode1, instruction);
            match instruction {
                Add => {
                    // sum ptr+1 and ptr+2 and save to ptr+3
                    let mut l = codes[self.pointer+1];
                    if !mode1 {
                        l = codes[l as usize];
                    }
                    let mut r = codes[self.pointer+2];
                    if !mode2 {
                        r = codes[r as usize];
                    }
                    let o = codes[self.pointer+3];
                    codes[o as usize] = l + r;
                    self.pointer += 4;
                },
                Mult => {
                    // multiply ptr+1 and ptr+2 and save to ptr+3
                    let mut l = codes[self.pointer+1];
                    if !mode1 {
                        l = codes[l as usize];
                    }
                    let mut r = codes[self.pointer+2];
                    if !mode2 {
                        r = codes[r as usize];
                    }
                    let o = codes[self.pointer+3];
                    codes[o as usize] = l*r;
                    self.pointer += 4;
                },
                Load => {
                    // save input to self.pointer+1
                    let location = codes[self.pointer+1];
                    codes[location as usize] = input.pop_front().unwrap();
                    self.pointer += 2;
                },
                Save => {
                    // save self.pointer+1 to output
                    let mut location = codes[self.pointer+1];
                    if !mode1 {
                        location = codes[location as usize];
                    }
                    self.pointer += 2;
                    return Some(location);
                },
                JumpTrue => {
                    let mut test = codes[self.pointer+1];
                    if !mode1 {
                        test = codes[test as usize];
                    }
                    if test != 0 {
                        let mut location = codes[self.pointer+2];
                        if !mode2 {
                            location = codes[location as usize];
                        }
                        self.pointer = location as usize;
                    } else {
                        self.pointer += 3;
                    }
                },
                JumpFalse => {
                    let mut test = codes[self.pointer+1];
                    if !mode1 {
                        test = codes[test as usize];
                    }
                    if test == 0 {
                        let mut location = codes[self.pointer+2];
                        if !mode2 {
                            location = codes[location as usize];
                        }
                        self.pointer = location as usize;
                    } else {
                        self.pointer += 3;
                    }
                },
                LessThan => {
                    let mut l = codes[self.pointer+1];
                    if !mode1 {
                        l = codes[l as usize];
                    }
                    let mut r = codes[self.pointer+2];
                    if !mode2 {
                        r = codes[r as usize];
                    }
                    let o = codes[self.pointer+3];
                    codes[o as usize] = if l < r { 1} else {0};
                    self.pointer += 4;
                },
                Equals => {
                    let mut l = codes[self.pointer+1];
                    if !mode1 {
                        l = codes[l as usize];
                    }
                    let mut r = codes[self.pointer+2];
                    if !mode2 {
                        r = codes[r as usize];
                    }
                    let o = codes[self.pointer+3];
                    codes[o as usize] = if l == r { 1} else {0};
                    self.pointer += 4;
                },
                Halt => return None,
                Error(e) => {
                    println!("Unexpected code! {}" ,e);
                    return None;
                }
            }
        }
    }        
}
fn decode_op(op : i32) -> (bool,bool,bool,OpCode) {
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
    (ten_thousands != 0, thousands != 0, hundreds != 0, instruction)
}