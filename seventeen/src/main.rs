use image;
use intcomputer::*;
use std::fs;

fn main() {
    let filename = "seventeen/input";
    let input = fs::read_to_string(filename).expect("can't open file");
    let codes: Vec<i64> = input
        .trim()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let scaffolding = view_space(codes);
    let p : String= path(scaffolding).into_iter().collect();
    println!("{:?}", p);
}

fn path(space: Vec<(Point, Status)>) -> Vec<char> {
    let (bot, bot_dir) = space
        .iter()
        .find(|(_, s)| if let Status::Bot(_) = s { true } else { false })
        .unwrap();
    let mut bot = bot.to_owned();
    let mut bot_dir = if let Status::Bot(d) = bot_dir {
        d
    } else {
        panic!("expected bot")
    }
    .to_owned();
    let mut directions = Vec::new();
    loop {
        let next = surrounding(&bot)
            .iter()
            .filter(|pnt| space.iter().any(|(p, s)| if let Status::Scaffold = s {*pnt == p} else {false}))
            .filter_map(|pnt| turn_needed(bot_dir, &bot, pnt)).next();
        match next {
            Some(t) => {
                directions.push(match t {
                    Turn::Left => 'L',
                    Turn::Right => 'R',
                });
                bot_dir = turn(bot_dir, t);
            }
            None => return directions,
        }
        let steps = line_from(&space, &bot, bot_dir);
        bot = steps[steps.len() -1].to_owned();
        directions.extend(steps.len().to_string().chars());
        directions.push(',');
    }
}

fn line_from(space: &Vec<(Point, Status)>, start: &Point, direction: Direction) -> Vec<Point> {
    let cross = space
        .iter()
        .filter(|(_, s)| {
            if let Status::Scaffold = s {
                true
            } else {
                false
            }
        })
        .map(|(p, _)| p)
        .filter(|p| p.x == start.x || p.y == start.y);
    match direction {
        Direction::North => {
            let mut projection: Vec<_> = cross.filter(|p| p.y < start.y).collect();
            projection.sort_by_key(|p| -p.y);
            projection
                .into_iter()
                .scan(start, |state, next| {
                    if state.y == next.y + 1 {
                        *state = next;
                        Some(*state)
                    } else {
                        None
                    }
                })
                .map(|p| p.to_owned())
                .collect()
        }
        Direction::South => {
            let mut projection: Vec<_> = cross.filter(|p| p.y > start.y).collect();
            projection.sort_by_key(|p| p.y);
            projection
                .into_iter()
                .scan(start, |state, next| {
                    if state.y == next.y - 1 {
                        *state = next;
                        Some(*state)
                    } else {
                        None
                    }
                })
                .map(|p| p.to_owned())
                .collect()
        }
        Direction::East => {
            let mut projection: Vec<_> = cross.filter(|p| p.x > start.x).collect();
            projection.sort_by_key(|p| p.x);
            projection
                .into_iter()
                .scan(start, |state, next| {
                    if state.x == next.x - 1 {
                        *state = next;
                        Some(*state)
                    } else {
                        None
                    }
                })
                .map(|p| p.to_owned())
                .collect()
        }
        Direction::West => {
            let mut projection: Vec<_> = cross.filter(|p| p.x < start.x).collect();
            projection.sort_by_key(|p| -p.x);
            projection
                .into_iter()
                .scan(start, |state, next| {
                    if state.x == next.x + 1 {
                        *state = next;
                        Some(*state)
                    } else {
                        None
                    }
                })
                .map(|p| p.to_owned())
                .collect()
        }
    }
}

fn turn(d: Direction, t: Turn) -> Direction {
    match (d, t) {
        (Direction::North, Turn::Left) => Direction::West,
        (Direction::North, Turn::Right) => Direction::East,
        (Direction::South, Turn::Left) => Direction::East,
        (Direction::South, Turn::Right) => Direction::West,
        (Direction::East, Turn::Left) => Direction::North,
        (Direction::East, Turn::Right) => Direction::South,
        (Direction::West, Turn::Left) => Direction::South,
        (Direction::West, Turn::Right) => Direction::North,
    }
}

fn turn_needed(direction: Direction, start: &Point, potential: &Point) -> Option<Turn> {
    match direction {
        Direction::North if potential.x == start.x - 1 => Some(Turn::Left),
        Direction::North if potential.x == start.x + 1 => Some(Turn::Right),
        Direction::South if potential.x == start.x - 1 => Some(Turn::Right),
        Direction::South if potential.x == start.x + 1 => Some(Turn::Left),
        Direction::East if potential.y == start.y - 1 => Some(Turn::Left),
        Direction::East if potential.y == start.y + 1 => Some(Turn::Right),
        Direction::West if potential.y == start.y - 1 => Some(Turn::Right),
        Direction::West if potential.y == start.y + 1 => Some(Turn::Left),
        _ => None,
    }
}

fn view_space(codes: Vec<i64>) -> Vec<(Point, Status)> {
    let mut bot = IntComputer::new(codes);
    let mut tiles = Vec::new();
    let mut x = 0;
    let mut y = 0;
    loop {
        match bot.run_codes().and_then(|v| std::char::from_u32(v as u32)) {
            Some('.') => tiles.push((Point { x, y }, Status::Space)),
            Some('#') => tiles.push((Point { x, y }, Status::Scaffold)),
            Some('<') => tiles.push((Point { x, y }, Status::Bot(Direction::West))),
            Some('>') => tiles.push((Point { x, y }, Status::Bot(Direction::East))),
            Some('^') => tiles.push((Point { x, y }, Status::Bot(Direction::North))),
            Some('v') => tiles.push((Point { x, y }, Status::Bot(Direction::South))),
            Some('\n') => {
                y += 1;
                x = 0;
                continue;
            }
            _ => break,
        }
        x += 1;
    }
    tiles
}

fn _print_maze(tiles: Vec<(Point, Status)>) {
    let min_x = tiles.iter().map(|(pos, _)| pos.x).min().unwrap();
    let max_x = tiles.iter().map(|(pos, _)| pos.x).max().unwrap();
    let min_y = tiles.iter().map(|(pos, _)| pos.y).min().unwrap();
    let max_y = tiles.iter().map(|(pos, _)| pos.y).max().unwrap();
    let points: Vec<_> = tiles
        .into_iter()
        .map(|(pos, s)| {
            (
                Point {
                    x: pos.x - min_x,
                    y: pos.y - min_y,
                },
                s,
            )
        })
        .collect();
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    let intersections: Vec<_> = points
        .iter()
        .filter(|p| {
            surrounding(&p.0).into_iter().all(|s| {
                points
                    .iter()
                    .any(|(ps, stat)| stat == &Status::Scaffold && *ps == s)
            })
        })
        .map(|(p, _)| p.to_owned())
        .collect();

    let i_sum: i32 = intersections.iter().map(|p| p.x * p.y).sum();
    println!("{}", i_sum);

    let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);

    for (p, s) in points {
        *imgbuf.get_pixel_mut(p.x as u32, p.y as u32) = match s {
            Status::Space => image::Rgb([0, 0, 0]),
            Status::Scaffold => image::Rgb([255, 255, 255]),
            Status::Bot(_) => image::Rgb([255, 0, 255]),
        };
    }
    for i in intersections {
        *imgbuf.get_pixel_mut(i.x as u32, i.y as u32) = image::Rgb([0, 255, 0]);
    }
    imgbuf.save("seventeen/out.png").unwrap();
}

fn surrounding(point: &Point) -> Vec<Point> {
    vec![
        Point {
            x: point.x + 1,
            y: point.y,
        },
        Point {
            x: point.x - 1,
            y: point.y,
        },
        Point {
            x: point.x,
            y: point.y + 1,
        },
        Point {
            x: point.x,
            y: point.y - 1,
        },
    ]
}

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
enum Status {
    Space,
    Scaffold,
    Bot(Direction),
}
