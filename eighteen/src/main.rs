use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        // a nonsense ordering but it keeps things consistent
        self.y.cmp(&other.y).then_with(|| self.x.cmp(&other.x))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq)]
struct SearchState {
    f_score: i32, // estimate total score
    point: Point,
}

impl Ord for SearchState {
    fn cmp(&self, other: &SearchState) -> Ordering {
        // Notice that the we flip the ordering to make it a min heap
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .f_score
            .cmp(&self.f_score)
            .then_with(|| self.point.cmp(&other.point))
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &SearchState) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn read_map<P: AsRef<Path>>(
    path: P,
) -> (
    Point,
    Vec<(char, Point)>,
    HashMap<Point, char>,
    Vec<Vec<bool>>,
) {
    let f = fs::File::open(path).unwrap();
    let reader = BufReader::new(f);
    let mut keys = Vec::new();
    let mut doors = HashMap::new();
    let mut person = Point { x: 0, y: 0 };
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
                        keys.push((
                            n,
                            Point {
                                x: x as i32,
                                y: y as i32,
                            },
                        ));
                        true
                    }
                    mut n if n.is_uppercase() => {
                        n.make_ascii_lowercase();
                        doors.insert(
                            Point {
                                x: x as i32,
                                y: y as i32,
                            },
                            n,
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

fn neighbours(point: &Point, map: &Vec<Vec<bool>>) -> Vec<Point> {
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
    .into_iter()
    .filter(|p| lookup(p, map))
    .collect()
}

fn lookup(p: &Point, map: &Vec<Vec<bool>>) -> bool {
    map[p.y as usize][p.x as usize]
}

fn manhattan(start: &Point, end: &Point) -> i32 {
    (end.x - start.x).abs() + (end.y - start.y).abs()
}

fn distance_between(
    route: &(Point, Point),
    map: &Vec<Vec<bool>>,
    memory: &mut HashMap<(Point, Point), i32>,
) -> i32 {
    match memory.get(route) {
        Some(d) => *d,
        None => {
            let d = route_between(&route.0, &route.1, map).unwrap().len() as i32;
            memory.insert((route.0.to_owned(), route.1.to_owned()), d);
            memory.insert((route.1.to_owned(), route.0.to_owned()), d);
            d
        }
    }
}

fn reconstruct_path(came_from: HashMap<Point, Point>, mut current: Point) -> Vec<Point> {
    let mut path = Vec::new();
    while let Some(next) = came_from.get(&current) {
        path.push(next.to_owned());
        current = next.to_owned();
    }
    path.reverse();
    path
}

fn route_between(start: &Point, end: &Point, map: &Vec<Vec<bool>>) -> Option<Vec<Point>> {
    if !lookup(&start, &map) || !lookup(&end, &map) {
        return None;
    }
    //println!("{:?} -> {:?}", start, end);

    let mut open_set: BinaryHeap<SearchState> = BinaryHeap::new();
    let start_f_score = manhattan(&start, &end);
    open_set.push(SearchState {
        f_score: start_f_score,
        point: start.to_owned(),
    });

    let mut g_score: HashMap<Point, i32> = HashMap::new();
    g_score.insert(start.to_owned(), 0);

    let mut f_score: HashMap<Point, i32> = HashMap::new();
    f_score.insert(start.to_owned(), start_f_score);

    let mut came_from: HashMap<Point, Point> = HashMap::new();

    while let Some(current) = open_set.pop() {
        if current.point == *end {
            return Some(reconstruct_path(came_from, current.point));
        }
        if current.f_score > f_score[&current.point] {
            continue;
        }
        let current_g = g_score[&current.point];

        for neighbour in neighbours(&current.point, &map) {
            let potential_g = current_g + 1;
            let neighbour_g = g_score.entry(neighbour.to_owned()).or_insert(std::i32::MAX);
            if potential_g < *neighbour_g {
                came_from.insert(neighbour.to_owned(), current.point.to_owned());
                *neighbour_g = potential_g;
                let new_f_score = potential_g + manhattan(&neighbour, &end);
                f_score.insert(neighbour.to_owned(), new_f_score);
                open_set.push(SearchState {
                    f_score: new_f_score,
                    point: neighbour,
                });
            }
        }
    }
    None
}

fn part1(
    person: Point,
    keys: &mut Vec<(char, Point)>,
    required_keys: &HashMap<char, Vec<char>>,
    found_keys: &mut Vec<char>,
    memory: &mut Vec<(Point, Vec<char>, i32)>,
    distance: &mut dyn FnMut(&(Point, Point)) -> i32,
) -> i32 {
    if keys.is_empty() {
        return 0;
    }
    if let Some((_, _, d)) = memory.iter().find(|(p, ks, _)| {
        *p == person && ks.len() == found_keys.len() && ks.iter().all(|k| found_keys.contains(k))
    }) {
        return *d;
    }
    let routes: Vec<_> = keys
        .iter()
        .filter(|(key, _)| required_keys[key].iter().all(|k| found_keys.contains(k)))
        .map(|(key, position)| {
            (
                key.to_owned(),
                position.to_owned(),
                distance(&(person.to_owned(), position.to_owned())),
            )
        })
        .collect();
    let mut min = std::i32::MAX;
    for (key, position, d) in routes {
        // mark key found
        keys.retain(|(k, _)| *k != key);
        found_keys.push(key);

        // keep exploring
        let score = d + part1(
            position.to_owned(),
            keys,
            required_keys,
            found_keys,
            memory,
            distance,
        );
        if score < min {
            min = score;
        }

        // unmark key found
        keys.push((key, position));
        found_keys.pop();
    }
    memory.push((person, found_keys.to_owned(), min));
    min
}

fn doors_between(
    map: &Vec<Vec<bool>>,
    doors: &HashMap<Point, char>,
    start: &Point,
    end: &Point,
) -> Vec<char> {
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
        .map(|(c, p)| (c.to_owned(), doors_between(&map, &doors, &person, p)))
        .collect();
    let mut found_keys = Vec::new();
    let mut route_memory = HashMap::new();
    let mut memory = Vec::new();
    println!(
        "{}",
        part1(
            person,
            &mut keys,
            &required_keys,
            &mut found_keys,
            &mut memory,
            &mut |p| distance_between(p, &map, &mut route_memory)
        )
    );
}
