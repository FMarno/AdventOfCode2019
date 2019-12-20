use image;
use intcomputer::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let filename = "fifteen/input";
    let input = fs::read_to_string(filename).expect("can't open file");
    let codes: Vec<i64> = input
        .trim()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    explore(codes);
}

fn print_maze(tiles: Vec<(Point, Status)>) {
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

    let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);

    for (p, s) in points {
        *imgbuf.get_pixel_mut(p.x as u32, p.y as u32) = match s {
            Status::Empty => image::Rgb([255, 255, 255]),
            Status::Goal => image::Rgb([255, 0, 255]),
            Status::Wall => image::Rgb([127, 127, 127]),
        };
    }
    *imgbuf.get_pixel_mut(-min_x as u32, -min_y as u32) = image::Rgb([0, 255, 0]);
    imgbuf.save("fifteen/out.png").unwrap();
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

enum Status {
    Wall,
    Empty,
    Goal,
}

fn explore(codes: Vec<i64>) {
    let mut position = Point { x: 0, y: 0 };
    let mut finish = Point { x: 0, y: 0 };
    let mut empties = vec![position.to_owned()];
    let mut walls = Vec::new();
    let mut to_explore = surronding(&position);

    let mut robot = IntComputer::new(codes);
    loop {
        let goto = match to_explore.pop() {
            Some(point) => point,
            None => break,
        };
        let path = match find_path(&position, &goto, &empties) {
            Some(x) => x,
            None => {
                println!("no path found from {:?} to {:?}", position, goto);
                return;
            }
        };
        let path_len = path.len();
        let directions = path_to_directions(path.to_owned());
        let directions_len = directions.len();
        for step in directions {
            robot.input.push_back(step as i64);
        }
        for _ in 0..directions_len - 1 {
            match robot.run_codes() {
                Some(0) => {
                    println!("hit wall in path\n{:?}", path);
                    return;
                }
                None => {
                    println!("none");
                    return;
                }
                Some(_) => (),
            }
        }
        match robot.run_codes() {
            Some(0) => {
                walls.push(goto);
                position = path[path_len - 2].to_owned();
            }
            Some(1) => {
                empties.push(goto.to_owned());
                position = goto;
                let new_spots = surronding(&position)
                    .into_iter()
                    .filter(|p| !(empties.contains(p) || empties.contains(p)));
                to_explore.extend(new_spots);
            }
            Some(2) => {
                finish = goto.to_owned();
                empties.push(goto.to_owned());
                position = goto;
                let new_spots = surronding(&position)
                    .into_iter()
                    .filter(|p| !(empties.contains(p) || empties.contains(p)));
                to_explore.extend(new_spots);
            }
            Some(_) | None => {
                println!("unexpected return");
                break;
            }
        }
    }
    println!(
        "length = {}",
        find_path(&Point { x: 0, y: 0 }, &finish, &empties)
            .unwrap()
            .len()
            - 1
    );
    println!(
        "fill time = {}",
        fill_with_oxygen(empties.to_owned(), finish.to_owned())
    );
    print_maze(collate(empties, walls, finish));
}

fn fill_with_oxygen(mut empties: Vec<Point>, start: Point) -> i32 {
    let mut time = 0;
    let mut next_to_fill = HashSet::new();
    next_to_fill.extend(
        surronding(&start)
            .into_iter()
            .filter(|p| empties.contains(p)),
    );
    empties.retain(|p| p != &start);
    loop {
        if next_to_fill.len() == 0 {
            break;
        }
        empties.retain(|p| !next_to_fill.contains(p));
        next_to_fill = next_to_fill
            .into_iter()
            .flat_map(|p| surronding(&p))
            .filter(|p| empties.contains(p))
            .collect();
        time += 1;
    }
    time
}

fn collate(empties: Vec<Point>, walls: Vec<Point>, finish: Point) -> Vec<(Point, Status)> {
    let mut tiles: Vec<_> = empties
        .into_iter()
        .map(|p| (p, Status::Empty))
        .chain(walls.into_iter().map(|p| (p, Status::Wall)))
        .collect();
    tiles.push((finish, Status::Goal));
    tiles
}

fn find_path(from: &Point, to: &Point, road: &Vec<Point>) -> Option<Vec<Point>> {
    let mut open_set = vec![from.to_owned()];
    let mut came_from = HashMap::new();

    let mut g_score: HashMap<Point, i32> = HashMap::new();
    g_score.insert(from.to_owned(), 0);
    let mut f_score: HashMap<Point, i32> = HashMap::new();
    f_score.insert(from.to_owned(), manhatten(from, to));

    loop {
        let current = match get_min_by_key(&mut open_set, |p| f_score[p]) {
            Some(x) => x,
            None => break,
        };
        if current == *to {
            return Some(reconstruct_path(came_from, current));
        }

        let neighbours = surronding(&current)
            .into_iter()
            .filter(|p| road.contains(p) || p == to);
        for n in neighbours {
            let tentative_g_score = g_score.get(&current).unwrap() + 1;
            let current_g_score = g_score.entry(n.to_owned()).or_insert(std::i32::MAX);
            if tentative_g_score < *current_g_score {
                came_from.insert(n.to_owned(), current.to_owned());
                *current_g_score = tentative_g_score;
                f_score.insert(n.to_owned(), *current_g_score + manhatten(&n, to));
                if !open_set.contains(&n) {
                    open_set.push(n);
                }
            }
        }
    }

    None
}

fn path_to_directions(points: Vec<Point>) -> Vec<Direction> {
    let mut directions = Vec::new();
    let mut previous = &points[0];
    for i in 1..points.len() {
        let next = &points[i];
        directions.push(match (next.x - previous.x, next.y - previous.y) {
            (-1, 0) => Direction::West,
            (1, 0) => Direction::East,
            (0, -1) => Direction::North,
            (0, 1) => Direction::South,
            _ => panic!("no route!"),
        });
        previous = next;
    }
    directions
}

fn reconstruct_path(came_from: HashMap<Point, Point>, mut current: Point) -> Vec<Point> {
    let mut path = vec![current.to_owned()];
    loop {
        match came_from.get(&current) {
            Some(next) => {
                path.push(next.to_owned());
                current = next.to_owned();
            }
            None => {
                path.reverse();
                return path;
            }
        }
    }
}

fn get_min_by_key<T, F, B>(values: &mut Vec<T>, mut f: F) -> Option<T>
where
    B: Ord,
    F: FnMut(&T) -> B,
{
    match values.iter().enumerate().min_by_key(|(_, v)| f(v)) {
        Some((index, _)) => Some(values.remove(index)),
        None => None,
    }
}

fn manhatten(from: &Point, to: &Point) -> i32 {
    (to.x - from.x).abs() + (to.y - from.y).abs()
}

fn surronding(point: &Point) -> Vec<Point> {
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
