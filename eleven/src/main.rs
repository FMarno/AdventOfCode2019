use colored::*;
use std::collections::{HashMap, VecDeque};
use std::fs;
use image;

fn main() {
    //let args: Vec<_> = env::args().collect();
    let filename = "eleven/input";
    let input = fs::read_to_string(filename).expect("can't open file");
    let codes: Vec<i64> = input
        .trim()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let mut tiles = HashMap::new();
    tiles.insert(Point { x: 0, y: 0 }, true);
    run_robot(codes, &mut tiles);
    println!("{}", tiles.len());
    let min_x = tiles.keys().map(|pos| pos.x).min().unwrap();
    let max_x = tiles.keys().map(|pos| pos.x).max().unwrap();
    let min_y = tiles.keys().map(|pos| pos.y).min().unwrap();
    let max_y = tiles.keys().map(|pos| pos.y).max().unwrap();
    let points: Vec<_> = tiles
        .into_iter()
        .filter(|(_, colour)| *colour)
        .map(|(pos, _)| Point {
            x: pos.x - min_x,
            y: pos.y - min_y,
        })
        .collect();
    let width = max_x - min_x +1;
    let height = max_y - min_y +1;

    let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);

    for p in points {
        *imgbuf.get_pixel_mut(p.x as u32, p.y as u32) = image::Luma([255]);
    }

    imgbuf.save("eleven/out.png").unwrap();
}

fn run_robot(codes: Vec<i64>, tiles: &mut HashMap<Point, bool>) {
    let mut computer = IntComputer::new(codes, VecDeque::new());
    let mut position = Point { x: 0, y: 0 };
    let mut orientation = Orientation::North;
    loop {
        let current_colour = tiles.get(&position).unwrap_or(&false);
        computer
            .input
            .push_back(if *current_colour { 1 } else { 0 });
        match computer.run_codes() {
            Some(colour) => {
                tiles.insert(position.to_owned(), colour == 1);
                match computer.run_codes() {
                    Some(direction) => {
                        let (new_point, new_orientation) = move_robot(
                            &position,
                            &orientation,
                            if direction == 0 {
                                Turn::Left
                            } else {
                                Turn::Right
                            },
                        );
                        position = new_point;
                        orientation = new_orientation;
                    }
                    None => break,
                }
            }
            None => break,
        }
    }
}

fn move_robot(position: &Point, orientation: &Orientation, turn: Turn) -> (Point, Orientation) {
    match orientation {
        Orientation::North => match turn {
            Turn::Left => (
                Point {
                    x: position.x - 1,
                    y: position.y,
                },
                Orientation::West,
            ),
            Turn::Right => (
                Point {
                    x: position.x + 1,
                    y: position.y,
                },
                Orientation::East,
            ),
        },
        Orientation::South => match turn {
            Turn::Left => (
                Point {
                    x: position.x + 1,
                    y: position.y,
                },
                Orientation::East,
            ),
            Turn::Right => (
                Point {
                    x: position.x - 1,
                    y: position.y,
                },
                Orientation::West,
            ),
        },
        Orientation::West => match turn {
            Turn::Left => (
                Point {
                    x: position.x,
                    y: position.y + 1,
                },
                Orientation::South,
            ),
            Turn::Right => (
                Point {
                    x: position.x,
                    y: position.y - 1,
                },
                Orientation::North,
            ),
        },
        Orientation::East => match turn {
            Turn::Left => (
                Point {
                    x: position.x,
                    y: position.y - 1,
                },
                Orientation::North,
            ),
            Turn::Right => (
                Point {
                    x: position.x,
                    y: position.y + 1,
                },
                Orientation::South,
            ),
        },
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

enum Turn {
    Left,
    Right,
}

enum Orientation {
    North,
    South,
    West,
    East,
}

struct IntComputer {
    codes: Vec<i64>,
    pointer: usize,
    input: VecDeque<i64>,
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

impl IntComputer {
    fn new(codes: Vec<i64>, input: VecDeque<i64>) -> IntComputer {
        IntComputer {
            codes,
            pointer: 0,
            input,
            relative_base: 0,
            disk: HashMap::new(),
        }
    }

    fn run_codes(&mut self) -> Option<i64> {
        loop {
            // println!("\nptr:{} r-ptr:{}", self.pointer, self.relative_base);
            //  print_codes(&self.codes, self.pointer, self.relative_base);
            // println!("{:?}", self.disk);
            let code = self.get_memory(self.pointer);
            let (mode3, mode2, mode1, instruction) = decode_op(code);
            //println!("{:?} {:?} {:?}", mode1, mode2, mode3);
            match instruction {
                OpCode::Add => {
                    // sum ptr+1 and ptr+2 and save to ptr+3
                    let l = self.get_memory(self.get_argument_location(self.pointer + 1, mode1));
                    let r = self.get_memory(self.get_argument_location(self.pointer + 2, mode2));
                    let o = self.get_argument_location(self.pointer + 3, mode3);
                    //println!("Add {} {} {}", l, r, o);
                    self.set_memory(o as usize, l + r);
                    self.pointer += 4;
                }
                OpCode::Mult => {
                    // multiply ptr+1 and ptr+2 and save to ptr+3
                    let l = self.get_memory(self.get_argument_location(self.pointer + 1, mode1));
                    let r = self.get_memory(self.get_argument_location(self.pointer + 2, mode2));
                    let o = self.get_argument_location(self.pointer + 3, mode3);
                    //println!("Mult {} {} {}", l, r, o);
                    self.set_memory(o as usize, l * r);
                    self.pointer += 4;
                }
                OpCode::Load => {
                    let location = self.get_argument_location(self.pointer + 1, mode1);
                    let input = self.input.pop_front().unwrap();
                    //println!("Load {} {}", location, input);
                    self.set_memory(location as usize, input);
                    self.pointer += 2;
                }
                OpCode::Save => {
                    // save self.pointer+1 to output
                    let value =
                        self.get_memory(self.get_argument_location(self.pointer + 1, mode1));
                    //println!("Save {}", value);
                    self.pointer += 2;
                    return Some(value);
                }
                OpCode::JumpTrue => {
                    let test = self.get_memory(self.get_argument_location(self.pointer + 1, mode1));
                    let location =
                        self.get_memory(self.get_argument_location(self.pointer + 2, mode2));
                    //println!("JumpIfTrue {} {}", test, location);
                    if test != 0 {
                        self.pointer = location as usize;
                    } else {
                        self.pointer += 3;
                    }
                }
                OpCode::JumpFalse => {
                    let test = self.get_memory(self.get_argument_location(self.pointer + 1, mode1));
                    let location =
                        self.get_memory(self.get_argument_location(self.pointer + 2, mode2));
                    //println!("JumpIfFalse {} {}", test, location);
                    if test == 0 {
                        self.pointer = location as usize;
                    } else {
                        self.pointer += 3;
                    }
                }
                OpCode::LessThan => {
                    let l = self.get_memory(self.get_argument_location(self.pointer + 1, mode1));
                    let r = self.get_memory(self.get_argument_location(self.pointer + 2, mode2));
                    let o = self.get_argument_location(self.pointer + 3, mode3);
                    //println!("LessThan {} {} {}", l, r, o);
                    self.set_memory(o as usize, if l < r { 1 } else { 0 });
                    self.pointer += 4;
                }
                OpCode::Equals => {
                    let l = self.get_memory(self.get_argument_location(self.pointer + 1, mode1));
                    let r = self.get_memory(self.get_argument_location(self.pointer + 2, mode2));
                    let o = self.get_argument_location(self.pointer + 3, mode3);
                    //println!("Equals {} {} {}", l, r, o);
                    self.set_memory(o as usize, if l == r { 1 } else { 0 });
                    self.pointer += 4;
                }
                OpCode::AdjustBase => {
                    let v = self.get_memory(self.get_argument_location(self.pointer + 1, mode1));
                    //println!("AdjustBase {}", v);
                    self.relative_base += v;
                    self.pointer += 2;
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

    fn get_argument_location(&self, ptr: usize, mode: Mode) -> usize {
        use Mode::*;
        match mode {
            Position => self.get_memory(ptr) as usize,
            Immediate => ptr,
            Relative => (self.get_memory(ptr) + self.relative_base) as usize,
        }
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

fn _print_codes(codes: &Vec<i64>, pointer: usize, relative_base: i64) {
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
