use std::fs;
use std::collections::VecDeque;
use permutohedron::heap_recursive;

fn main() {
    let mut inputs : Vec<_> = (0..5).collect();
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
}

fn thruster_signal(input : Vec<i32>, codes : Vec<i32>) -> i32 {
    let amps = input.iter().map(|x|{
        let mut inputs : VecDeque<i32> = VecDeque::new();
        inputs.push_back(*x);
        Amp{codes:codes.to_vec(), pointer: 0, input:inputs}
    });
    amps.fold(0, |output, mut amp| {
        amp.input.push_back(output);
        amp.run_codes().expect("NO OUTPUT")
    })
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
        let pointer = &mut self.pointer;

        loop{
            //println!("{} {:?}",pointer, codes);
            let code = codes[*pointer];
            let (_mode3, mode2, mode1, instruction) = decode_op(code);
            //println!("{} {} {} {:?}", _mode3, mode2, mode1, instruction);
            match instruction {
                Add => {
                    // sum ptr+1 and ptr+2 and save to ptr+3
                    let mut l = codes[*pointer+1];
                    if !mode1 {
                        l = codes[l as usize];
                    }
                    let mut r = codes[*pointer+2];
                    if !mode2 {
                        r = codes[r as usize];
                    }
                    let o = codes[*pointer+3];
                    codes[o as usize] = l + r;
                    *pointer += 4;
                },
                Mult => {
                    // multiply ptr+1 and ptr+2 and save to ptr+3
                    let mut l = codes[*pointer+1];
                    if !mode1 {
                        l = codes[l as usize];
                    }
                    let mut r = codes[*pointer+2];
                    if !mode2 {
                        r = codes[r as usize];
                    }
                    let o = codes[*pointer+3];
                    codes[o as usize] = l*r;
                    *pointer += 4;
                },
                Load => {
                    // save input to *pointer+1
                    let location = codes[*pointer+1];
                    codes[location as usize] = input.pop_front().unwrap();
                    *pointer += 2;
                },
                Save => {
                    // save *pointer+1 to output
                    let mut location = codes[*pointer+1];
                    if !mode1 {
                        location = codes[location as usize];
                    }
                    *pointer += 2;
                    return Some(location);
                },
                JumpTrue => {
                    let mut test = codes[*pointer+1];
                    if !mode1 {
                        test = codes[test as usize];
                    }
                    if test != 0 {
                        let mut location = codes[*pointer+2];
                        if !mode2 {
                            location = codes[location as usize];
                        }
                        *pointer = location as usize;
                    } else {
                        *pointer += 3;
                    }
                },
                JumpFalse => {
                    let mut test = codes[*pointer+1];
                    if !mode1 {
                        test = codes[test as usize];
                    }
                    if test == 0 {
                        let mut location = codes[*pointer+2];
                        if !mode2 {
                            location = codes[location as usize];
                        }
                        *pointer = location as usize;
                    } else {
                        *pointer += 3;
                    }
                },
                LessThan => {
                    let mut l = codes[*pointer+1];
                    if !mode1 {
                        l = codes[l as usize];
                    }
                    let mut r = codes[*pointer+2];
                    if !mode2 {
                        r = codes[r as usize];
                    }
                    let o = codes[*pointer+3];
                    codes[o as usize] = if l < r { 1} else {0};
                    *pointer += 4;
                },
                Equals => {
                    let mut l = codes[*pointer+1];
                    if !mode1 {
                        l = codes[l as usize];
                    }
                    let mut r = codes[*pointer+2];
                    if !mode2 {
                        r = codes[r as usize];
                    }
                    let o = codes[*pointer+3];
                    codes[o as usize] = if l == r { 1} else {0};
                    *pointer += 4;
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