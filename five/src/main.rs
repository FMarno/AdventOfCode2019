use std::fs;

fn main() {
    let input = fs::read_to_string("five/input").expect("can't open file");
    let codes: Vec<i32> = input
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut output = Vec::new();
    run_codes(codes, 5, &mut output);
    for x in output {
        print!("{} ", x);
    }
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

fn run_codes(mut codes: Vec<i32>, input: i32, output: &mut Vec<i32>) -> i32 {
    use OpCode::*;

    let mut pointer = 0;
    loop {
        println!("{} {:?}", pointer, codes);
        let code = codes[pointer];
        let (_mode3, mode2, mode1, instruction) = decode_op(code);
        println!("{} {} {} {:?}", _mode3, mode2, mode1, instruction);
        match instruction {
            Add => {
                // sum ptr+1 and ptr+2 and save to ptr+3
                let mut l = codes[pointer + 1];
                if !mode1 {
                    l = codes[l as usize];
                }
                let mut r = codes[pointer + 2];
                if !mode2 {
                    r = codes[r as usize];
                }
                let o = codes[pointer + 3];
                codes[o as usize] = l + r;
                pointer += 4;
            }
            Mult => {
                // multiply ptr+1 and ptr+2 and save to ptr+3
                let mut l = codes[pointer + 1];
                if !mode1 {
                    l = codes[l as usize];
                }
                let mut r = codes[pointer + 2];
                if !mode2 {
                    r = codes[r as usize];
                }
                let o = codes[pointer + 3];
                codes[o as usize] = l * r;
                pointer += 4;
            }
            Load => {
                // save input to pointer+1
                let location = codes[pointer + 1];
                codes[location as usize] = input;
                pointer += 2;
            }
            Save => {
                // save pointer+1 to output
                let mut location = codes[pointer + 1];
                if !mode1 {
                    location = codes[location as usize];
                }
                output.push(location);
                pointer += 2;
            }
            JumpTrue => {
                let mut test = codes[pointer + 1];
                if !mode1 {
                    test = codes[test as usize];
                }
                if test != 0 {
                    let mut location = codes[pointer + 2];
                    if !mode2 {
                        location = codes[location as usize];
                    }
                    pointer = location as usize;
                } else {
                    pointer += 3;
                }
            }
            JumpFalse => {
                let mut test = codes[pointer + 1];
                if !mode1 {
                    test = codes[test as usize];
                }
                if test == 0 {
                    let mut location = codes[pointer + 2];
                    if !mode2 {
                        location = codes[location as usize];
                    }
                    pointer = location as usize;
                } else {
                    pointer += 3;
                }
            }
            LessThan => {
                let mut l = codes[pointer + 1];
                if !mode1 {
                    l = codes[l as usize];
                }
                let mut r = codes[pointer + 2];
                if !mode2 {
                    r = codes[r as usize];
                }
                let o = codes[pointer + 3];
                codes[o as usize] = if l < r { 1 } else { 0 };
                pointer += 4;
            }
            Equals => {
                let mut l = codes[pointer + 1];
                if !mode1 {
                    l = codes[l as usize];
                }
                let mut r = codes[pointer + 2];
                if !mode2 {
                    r = codes[r as usize];
                }
                let o = codes[pointer + 3];
                codes[o as usize] = if l == r { 1 } else { 0 };
                pointer += 4;
            }
            Halt => return codes[0],
            Error(e) => {
                println!("Unexpected code! {}", e);
                return 0;
            }
        }
    }
}

fn decode_op(op: i32) -> (bool, bool, bool, OpCode) {
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
        99 => Halt,
        x => Error(x),
    };
    (
        ten_thousands != 0,
        thousands != 0,
        hundreds != 0,
        instruction,
    )
}
