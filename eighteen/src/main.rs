mod input;
mod path_finder;
mod point;
mod route_finder;

use crate::input::*;
use crate::path_finder::*;
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
    let (person, mut keys, doors, map) = read_map("eighteen/input");
    let required_keys = keys
        .iter()
        .map(|(c, p)| (c.to_owned(), doors_between(&map, &doors, &person, p).into_iter().fold(0,|a,x| a | x)))
        .collect();
    let mut route_memory = HashMap::new();
    let final_value = 2_u32.pow(keys.len() as u32)-1;
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
}
