use intcomputer::*;
use std::char;
use std::collections::HashMap;
use std::fs;

fn main() {
    let filename = "seventeen/input";
    let input = fs::read_to_string(filename).expect("can't open file");
    let codes: Vec<i64> = input
        .trim()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let mut bot = IntComputer::new(codes.to_owned());
    let scaffolding = view_space(&mut bot);
    ascii_maze(&scaffolding);
    let p: String = path(scaffolding).into_iter().collect();
    // let p = "R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2,".to_string();
    println!("{}", p);
    let mut subcommands = find_subcommands(p.to_owned()).unwrap();
    let main: Vec<_> = subcommands.iter().map(|(_, v)| v.to_owned()).collect();
    subcommands.sort_by(|l, r| l.1.cmp(&r.1));
    let mut routines: Vec<_> = subcommands.into_iter().map(|(k, _)| k).collect();
    routines.dedup();
    println!("{:?}\n{:?}", main, routines);

    //recovery
    let mut recovery_codes = codes;
    recovery_codes[0] = 2;
    let mut recovery_bot = IntComputer::new(recovery_codes);
    recovery_bot.input.push_back(main[0] as i64);
    for c in main.into_iter().skip(1) {
        recovery_bot.input.push_back(',' as i64);
        recovery_bot.input.push_back(c as i64);
    }
    recovery_bot.input.push_back('\n' as i64);

    for routine in routines {
        for c in routine.trim_end_matches(',').chars() {
            recovery_bot.input.push_back(c as i64);
        }
        recovery_bot.input.push_back('\n' as i64);
    }
    recovery_bot.input.push_back('n' as i64);
    recovery_bot.input.push_back('\n' as i64);
    println!("{:?}\n", recovery_bot.input);

    let mut last_c = recovery_bot.run_codes().unwrap();
    loop {
        match recovery_bot.run_codes() {
            Some(c) => {
                print!("{}", char::from_u32(last_c as u32).unwrap());
                last_c = c;
            }
            None => {
                println!("dust = {}", last_c);
                break;
            }
        }
    }
}

fn find_subcommands(command: String) -> Option<Vec<(String, char)>> {
    let subs: Vec<_> = subcommands(&command).collect();
    let sub_count = subs.len();
    for a in 0..sub_count {
        for b in a + 1..sub_count {
            for c in b + 1..sub_count {
                match try_match_subs(&command, &vec![&subs[a], &subs[b], &subs[c]]) {
                    Some(x) => return Some(x),
                    None => (),
                }
            }
        }
    }
    None
}

fn try_match_subs(command: &str, subs: &Vec<&String>) -> Option<Vec<(String, char)>> {
    let mut idx = 0;
    let mut commands = Vec::new();
    while idx < command.len() {
        match subs
            .iter()
            .zip("ABC".chars())
            .find(|(sub, _)| command[idx..].starts_with(**sub))
        {
            Some((sub, letter)) => {
                commands.push(((*sub).to_owned(), letter));
                idx += sub.len();
            }
            None => {
                //println!("failed {:?}", commands);
                return None;
            }
        }
    }
    Some(commands)
}

fn subcommands(commands: &String) -> impl Iterator<Item = String> {
    let mut subcommands = HashMap::new();
    for i in 0..commands.len() - 1 {
        if commands[i..=i] == *"," {
            continue;
        }
        for j in (i + 1..(i + 20).min(commands.len())).rev() {
            if commands[j..=j] != *"," {
                continue;
            }
            let candidate = commands[i..=j].to_string();
            let count = count_occurances(&candidate, &commands);
            if count > 1 {
                subcommands.insert(candidate, count);
            }
        }
    }
    let mut subcommands: Vec<_> = subcommands.into_iter().collect();
    subcommands.sort_by_key(|(k, _)| k.len());
    subcommands.reverse();
    subcommands.into_iter().map(|(k, _)| k)
}

fn count_occurances(candidate: &String, s: &String) -> u32 {
    let mut count = 0;
    let mut i = 0;
    while let Some(x) = s[i..].find(candidate) {
        count += 1;
        i += x + candidate.len();
    }
    count
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
            .filter(|pnt| {
                space.iter().any(|(p, s)| {
                    if let Status::Scaffold = s {
                        *pnt == p
                    } else {
                        false
                    }
                })
            })
            .filter_map(|pnt| turn_needed(bot_dir, &bot, pnt))
            .next();
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
        directions.push(',');
        let steps = line_from(&space, &bot, bot_dir);
        bot = steps[steps.len() - 1].to_owned();
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

fn view_space(bot: &mut IntComputer) -> Vec<(Point, Status)> {
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

fn ascii_maze(tiles: &Vec<(Point, Status)>) {
    let min_x = tiles.iter().map(|(pos, _)| pos.x).min().unwrap();
    let max_x = tiles.iter().map(|(pos, _)| pos.x).max().unwrap();
    let min_y = tiles.iter().map(|(pos, _)| pos.y).min().unwrap();
    let max_y = tiles.iter().map(|(pos, _)| pos.y).max().unwrap();
    let points: Vec<_> = tiles
        .into_iter()
        .filter(|(_, s)| {
            *s == Status::Scaffold || if let Status::Bot(_) = s { true } else { false }
        })
        .map(|(pos, _)| Point {
            x: pos.x - min_x,
            y: pos.y - min_y,
        })
        .collect();
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    for y in 0..height {
        for x in 0..width {
            if points.contains(&Point { x, y }) {
                print!("# ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
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
