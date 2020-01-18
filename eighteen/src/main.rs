use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet};
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

#[derive(Eq, PartialEq, Debug)]
struct SearchState<T> {
    score: i32, // estimate total score
    node: T,
}

impl<T> Ord for SearchState<T>
where
    T: Eq + Ord,
{
    fn cmp(&self, other: &SearchState<T>) -> Ordering {
        // Notice that the we flip the ordering to make it a min heap
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.score.cmp(&self.score).then_with(|| self.node.cmp(&other.node))
    }
}

impl<T> PartialOrd for SearchState<T>
where
    T: PartialEq + Ord,
{
    fn partial_cmp(&self, other: &SearchState<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn read_map<P: AsRef<Path>>(
    path: P,
) -> (
    Point,
    HashMap<char, Point>,
    HashMap<Point, char>,
    Vec<Vec<bool>>,
) {
    let f = fs::File::open(path).unwrap();
    let reader = BufReader::new(f);
    let mut keys = HashMap::new();
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
                        keys.insert(
                            n,
                            Point {
                                x: x as i32,
                                y: y as i32,
                            },
                        );
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

    let mut open_set: BinaryHeap<SearchState<Point>> = BinaryHeap::new();
    let start_f_score = manhattan(&start, &end);
    open_set.push(SearchState {
        score: start_f_score,
        node: start.to_owned(),
    });

    let mut g_score: HashMap<Point, i32> = HashMap::new();
    g_score.insert(start.to_owned(), 0);

    let mut f_score: HashMap<Point, i32> = HashMap::new();
    f_score.insert(start.to_owned(), start_f_score);

    let mut came_from: HashMap<Point, Point> = HashMap::new();

    while let Some(current) = open_set.pop() {
        if current.node == *end {
            return Some(reconstruct_path(came_from, current.node));
        }
        let current_g = g_score[&current.node];

        for neighbour in neighbours(&current.node, &map) {
            let potential_g = current_g + 1;
            let neighbour_g = g_score.entry(neighbour.to_owned()).or_insert(std::i32::MAX);
            if potential_g < *neighbour_g {
                came_from.insert(neighbour.to_owned(), current.node.to_owned());
                *neighbour_g = potential_g;
                let new_f_score = potential_g + manhattan(&neighbour, &end);
                f_score.insert(neighbour.to_owned(), new_f_score);
                open_set.push(SearchState {
                    score: new_f_score,
                    node: neighbour,
                });
            }
        }
    }
    None
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct KeySet {
    keys: HashSet<char>,
    point: Point,
}

impl Ord for KeySet {
    fn cmp(&self, other: &KeySet) -> Ordering {
        // Notice that the we flip the ordering to make it a min heap
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        self.keys.len().cmp(&other.keys.len()).then_with(|| self.point.cmp(&other.point))
    }
}

impl PartialOrd for KeySet {
    fn partial_cmp(&self, other: &KeySet) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn available(required_keys: &HashMap<char, Vec<char>>, owned: &HashSet<char>) -> Vec<char> {
    required_keys
        .iter()
        .filter(|(_, req)| req.iter().all(|r| owned.contains(r)))
        .filter(|(k, _)| !owned.contains(k))
        .map(|(k, _)| k.to_owned())
        .collect()
}

fn part1(
    start: Point,
    keys: &mut HashMap<char, Point>,
    required_keys: &HashMap<char, Vec<char>>,
    distance: &mut dyn FnMut(&(Point, Point)) -> i32,
) -> i32 {
    let mut distance_to: BTreeMap<KeySet, i32> = BTreeMap::new();
    distance_to.insert(
        KeySet {
            keys: HashSet::new(),
            point: start.to_owned(),
        },
        0,
    );

    let mut open_set: BinaryHeap<SearchState<_>> = BinaryHeap::new();
    open_set.push(SearchState {
        node: KeySet {
            keys: HashSet::new(),
            point: start,
        },
        score: 0,
    });
    while let Some(current) = open_set.pop() {
        if current.node.keys.len() == keys.len() {
            println!("{:?}", current.node.keys);
            return current.score;
        }

        println!("{:?} {:?} {}", current.node.keys, current.node.point, current.score);
        for neighbour in available(required_keys, &current.node.keys).into_iter() {
            let location = keys[&neighbour].to_owned();
            let tentative_score =
                current.score + distance(&(current.node.point.to_owned(), location.to_owned()));

            let mut ks = current.node.keys.to_owned();
            ks.insert(neighbour);

            let key_set = KeySet {
                keys: ks,
                point: location,
            };
            let current_score = distance_to
                .entry(key_set.to_owned())
                .or_insert(std::i32::MAX);
            if tentative_score < *current_score {
                *current_score = tentative_score;
                open_set.push(SearchState {
                    node: key_set,
                    score: *current_score,
                });
            }
        }
    }
    0
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
    let (person, mut keys, doors, map) = read_map("eighteen/test1");
    let required_keys = keys
        .iter()
        .map(|(c, p)| (c.to_owned(), doors_between(&map, &doors, &person, p)))
        .collect();
    let mut route_memory = HashMap::new();
    println!(
        "{}",
        part1(
            person,
            &mut keys,
            &required_keys,
            &mut |p| distance_between(p, &map, &mut route_memory)
        )
    );
}
