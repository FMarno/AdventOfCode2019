use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
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
    let best_point = best_point.unwrap();
    println!("{:?}: {}", best_point, visible_points(best_point, &points));
    let order = laser_order(&best_point, &points);
    let target = &order[199];
    println!("{}, {}\t{}", target.x, target.y, angle(best_point, &target));
}

#[derive(Debug, Clone)]
struct PointInfo {
    point: Point,
    distance: f32,
}

fn laser_order(origin: &Point, asteroids: &Vec<Point>) -> Vec<Point> {
    let mut angle_distance: Vec<(f32, Vec<PointInfo>)> = Vec::new();
    let asteroids_info = asteroids
        .iter()
        .filter(|x| !(x.x == origin.x && x.y == origin.y))
        .map(|other| {
            (
                angle(origin, other),
                PointInfo {
                    point: other.to_owned(),
                    distance: distance(origin, other),
                },
            )
        });
    for (angle, asteroid) in asteroids_info {
        match angle_distance.iter_mut().find(|x| x.0 == angle) {
            Some(l) => l.1.push(asteroid),
            None => angle_distance.push((angle, vec![asteroid])),
        }
    }
    for entry in &mut angle_distance {
        entry
            .1
            .sort_by(|l, r| l.distance.partial_cmp(&r.distance).unwrap());
    }
    angle_distance.sort_by(|l, r| l.0.partial_cmp(&r.0).unwrap());

    let mut order: Vec<Point> = Vec::new();
    for x in 0.. {
        let circle: Vec<_> = angle_distance
            .iter()
            .filter_map(|(_angle, infos)| infos.get(x))
            .map(|info| info.point.to_owned())
            .collect();
        match circle.len() == 0 {
            false => order.extend(circle),
            true => break,
        }
    }
    order
}

fn visible_points(asteroid: &Point, others: &Vec<Point>) -> usize {
    let mut angles: Vec<_> = others
        .iter()
        .filter(|x| !(x.x == asteroid.x && x.y == asteroid.y))
        .map(|other| angle(asteroid, other))
        .collect();
    angles.sort_by(|a, b| a.partial_cmp(b).unwrap());
    angles.dedup();
    angles.len()
}

fn angle(origin: &Point, other: &Point) -> f32 {
    let x_delta = other.x - origin.x;
    let y_delta = other.y - origin.y;
    let a = y_delta.atan2(x_delta);
    if a < -std::f32::consts::FRAC_PI_2 {
        a + (2_f32 * std::f32::consts::PI)
    } else {
        a
    }
}

fn distance(origin: &Point, other: &Point) -> f32 {
    let x_delta = other.x - origin.x;
    let y_delta = other.y - origin.y;
    (x_delta.powi(2) + y_delta.powi(2)).sqrt()
}
