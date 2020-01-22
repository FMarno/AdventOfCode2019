use crate::point::*;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_map<P: AsRef<Path>>(
    path: P,
) -> (
    Point,
    HashMap<u32, Point>,
    HashMap<Point, u32>,
    Vec<Vec<bool>>,
) {
    let f = fs::File::open(path).unwrap();
    let reader = BufReader::new(f);
    let mut letter_to_shift: HashMap<char, u32> = HashMap::new();
    let mut keys = HashMap::new();
    let mut doors = HashMap::new();
    let mut person = Point { x: 0, y: 0 };
    let mut curr_shift = 1;
    let map = reader
        .lines()
        .filter_map(|l| l.ok())
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .filter(|c| c.is_alphabetic() || *c == '.' || *c == '#' || *c == '@')
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => true,
                    '#' => false,
                    '@' => {
                        person = Point {
                            x: x as i32,
                            y: y as i32,
                        };
                        true
                    }
                    n if n.is_lowercase() => {
                        let shift = letter_to_shift.entry(n).or_insert(curr_shift);
                        if *shift == curr_shift {
                            curr_shift = curr_shift << 1;
                        }
                        keys.insert(
                            *shift,
                            Point {
                                x: x as i32,
                                y: y as i32,
                            },
                        );
                        true
                    }
                    mut n if n.is_uppercase() => {
                        n.make_ascii_lowercase();
                        let shift = letter_to_shift.entry(n).or_insert(curr_shift);
                        if *shift == curr_shift {
                            curr_shift = curr_shift << 1;
                        }
                        doors.insert(
                            Point {
                                x: x as i32,
                                y: y as i32,
                            },
                            *shift,
                        );
                        true
                    }
                    _ => panic!("ahh"),
                })
                .collect()
        })
        .collect();
    (person, keys, doors, map)
}
