use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

fn main() {
    let f = File::open("ten/input").expect("Can't open file");
    let reader = BufReader::new(f);
    let mut points = Vec::new();
    for (y, line) in reader.lines().filter_map(|l| l.ok()).enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                points.push(Point {
                    x: x as f32,
                    y: y as f32,
                });
            }
        }
    }
    let best_point = points
        .iter()
        .max_by(|l, r| visible_points(l, &points).cmp(&visible_points(r, &points)));
    println!("{:?}", best_point.unwrap());
}

fn visible_points(asteroid: &Point, others: &Vec<Point>) -> usize {
    let mut angles: Vec<_> = others
        .iter()
        .map(|other| {
            let x_delta = other.x - asteroid.x;
            let y_delta = other.y - asteroid.y;
            y_delta.atan2(x_delta)
        })
        .collect();
    angles.sort_by(|a, b| a.partial_cmp(b).unwrap());
    angles.dedup();
    angles.len()
}
