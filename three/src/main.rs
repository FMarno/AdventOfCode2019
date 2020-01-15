use std::fs;

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    start: Point,
    finish: Point,
}

fn main() {
    let mut input = fs::read_to_string("three/input").unwrap();
    let line2 = input.split_off(input.find('\n').unwrap());
    let line1 = make_lines(input);
    let line2 = make_lines(line2);
    let intersections: Vec<Point> = line1
        .iter()
        .map(|l| line2.iter().filter_map(move |l2| intersection(l, l2)))
        .flatten()
        .filter(|p| !(p.x == 0 && p.y == 0))
        .collect();
    part2(intersections, line1, line2)
}

fn _part1(mut intersections: Vec<Point>) {
    intersections.sort_by(|a, b| _manhattan(a).cmp(&_manhattan(b)));
    let point = &intersections[0];
    println!("{}", _manhattan(point));
}

fn part2(intersections: Vec<Point>, line1: Vec<Line>, line2: Vec<Line>) {
    let mut delays: Vec<_> = intersections
        .iter()
        .map(|p| distance_to(p, &line1) + distance_to(p, &line2))
        .collect();
    delays.sort();
    println!("{:?}", delays[0]);
}

fn distance_to(p: &Point, line: &[Line]) -> i32 {
    let mut traveled = 0;
    for l in line {
        if between(l.start.x, l.finish.x, p.x) && between(l.start.y, l.finish.y, p.y) {
            if l.start.x == p.x {
                traveled += (l.start.y - p.y).abs();
            } else {
                traveled += (l.start.x - p.x).abs();
            }
            break;
        } else {
            traveled = traveled + (l.start.x - l.finish.x).abs() + (l.start.y - l.finish.y).abs();
        }
    }
    traveled
}

fn _manhattan(a: &Point) -> i32 {
    a.x.abs() + a.y.abs()
}

fn intersection(a: &Line, b: &Line) -> Option<Point> {
    let (hori, vert) = if a.start.x == a.finish.x {
        (b, a)
    } else {
        (a, b)
    };
    let x = vert.start.x;
    if between(hori.start.x, hori.finish.x, x) {
        let y = hori.start.y;
        if between(vert.start.y, vert.finish.y, y) {
            return Some(Point { x, y });
        }
    }
    None
}

fn between(a: i32, b: i32, x: i32) -> bool {
    (a < b && a <= x && x <= b) || (b <= x && x <= a)
}

fn make_lines(l: String) -> Vec<Line> {
    let l = l.trim();
    l.split(',')
        .scan(Point { x: 0, y: 0 }, |state, next| {
            let (direction, distance) = next.split_at(1);
            let distance = distance.parse::<i32>().unwrap();
            let last = (*state).clone();
            match direction {
                "U" => (*state).y = last.y + distance,
                "D" => (*state).y = last.y - distance,
                "R" => (*state).x = last.x + distance,
                "L" => (*state).x = last.x - distance,
                _ => panic!("That's not supposed to happen"),
            };
            Some(Line {
                start: last,
                finish: (*state).clone(),
            })
        })
        .collect()
}
