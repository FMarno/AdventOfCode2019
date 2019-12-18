use image;
use intcomputer::*;
use std::collections::HashMap;
use std::fs;

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
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);

    for p in points {
        *imgbuf.get_pixel_mut(p.x as u32, p.y as u32) = image::Luma([255]);
    }

    imgbuf.save("eleven/out.png").unwrap();
}

fn run_robot(codes: Vec<i64>, tiles: &mut HashMap<Point, bool>) {
    let mut computer = IntComputer::new(codes);
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
