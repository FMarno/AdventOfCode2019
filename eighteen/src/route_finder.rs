use crate::point::*;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
    }
}

#[derive(Eq, PartialEq, Debug)]
struct SearchState {
    score: i32, // estimate total score
    node: Point,
}

impl Ord for SearchState {
    fn cmp(&self, other: &SearchState) -> Ordering {
        // Notice that the we flip the ordering to make it a min heap
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .score
            .cmp(&self.score)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &SearchState) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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

pub fn distance_between(
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

pub fn route_between(start: &Point, end: &Point, map: &Vec<Vec<bool>>) -> Option<Vec<Point>> {
    if !lookup(&start, &map) || !lookup(&end, &map) {
        return None;
    }
    //println!("{:?} -> {:?}", start, end);

    let mut open_set: BinaryHeap<SearchState> = BinaryHeap::new();
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
