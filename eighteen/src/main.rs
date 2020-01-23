mod input;
mod part2;
mod path_finder;
mod point;
mod route_finder;

use crate::input::*;
use crate::part2::part2;
use crate::path_finder::part1;
use crate::point::*;
use crate::route_finder::*;
use std::collections::HashMap;

fn doors_between(
    map: &Vec<Vec<bool>>,
    doors: &HashMap<Point, u32>,
    start: &Point,
    end: &Point,
) -> Vec<u32> {
    let route = route_between(start, end, map).unwrap();
    route
        .into_iter()
        .filter_map(|p| doors.get(&p).map(|c| c.to_owned()))
        .collect()
}

fn main() {
    let (person, mut keys, doors, mut map) = read_map("eighteen/input");
    let required_keys = keys
        .iter()
        .map(|(c, p)| {
            (
                c.to_owned(),
                doors_between(&map, &doors, &person, p)
                    .into_iter()
                    .fold(0, |a, x| a | x),
            )
        })
        .collect();
    let mut route_memory = HashMap::new();
    let final_value = 2_u32.pow(keys.len() as u32) - 1;
    println!(
        "{}",
        part1(
            person,
            &mut keys,
            &required_keys,
            &mut |p| distance_between(p, &map, &mut route_memory),
            final_value
        )
    );
    let (x, y) = (map[0].len() / 2, map.len() / 2);
    map[y][x] = false;
    map[y + 1][x] = false;
    map[y - 1][x] = false;
    map[y][x + 1] = false;
    map[y][x - 1] = false;
    let starts = [
        Point {
            x: (x + 1) as i32,
            y: (y + 1) as i32,
        },
        Point {
            x: (x + 1) as i32,
            y: (y - 1) as i32,
        },
        Point {
            x: (x - 1) as i32,
            y: (y + 1) as i32,
        },
        Point {
            x: (x - 1) as i32,
            y: (y - 1) as i32,
        },
    ];
    let mut part2_route_memory = HashMap::new();
    println!(
        "{}",
        part2(
            starts,
            &mut keys,
            &required_keys,
            &mut |p| distance_between(p, &map, &mut part2_route_memory),
            final_value
        )
    );
}
