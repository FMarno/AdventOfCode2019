use crate::point::*;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap};

#[derive(Eq, PartialEq, Debug)]
struct SearchState {
    score: i32, // estimate total score
    node: KeySet,
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

#[derive(PartialEq, Eq, Clone, Debug)]
struct KeySet {
    keys: u32,
    point: Point,
}

impl Ord for KeySet {
    fn cmp(&self, other: &KeySet) -> Ordering {
        // Notice that the we flip the ordering to make it a min heap
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        self.keys
            .cmp(&other.keys)
            .then_with(|| self.point.cmp(&other.point))
    }
}

impl PartialOrd for KeySet {
    fn partial_cmp(&self, other: &KeySet) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn available(required_keys: &HashMap<u32, Vec<u32>>, owned: u32) -> Vec<u32> {
    required_keys
        .iter()
        .filter(|(_, req)| req.iter().all(|r| owned & *r != 0))
        .filter(|(k, _)| (owned & **k) == 0)
        .map(|(k, _)| k.to_owned())
        .collect()
}

pub fn part1(
    start: Point,
    keys: &mut HashMap<u32, Point>,
    required_keys: &HashMap<u32, Vec<u32>>,
    distance: &mut dyn FnMut(&(Point, Point)) -> i32,
    final_value : u32,
) -> i32 {
    let mut distance_to: BTreeMap<KeySet, i32> = BTreeMap::new();
    distance_to.insert(
        KeySet {
            keys: 0,
            point: start.to_owned(),
        },
        0,
    );

    let mut open_set: BinaryHeap<SearchState> = BinaryHeap::new();
    open_set.push(SearchState {
        node: KeySet {
            keys: 0,
            point: start,
        },
        score: 0,
    });

    while let Some(current) = open_set.pop() {
        if current.node.keys == final_value {
            return current.score;
        }

        for neighbour in available(required_keys, current.node.keys).into_iter() {
            let location = keys[&neighbour].to_owned();
            let tentative_score =
                current.score + distance(&(current.node.point.to_owned(), location.to_owned()));

            let mut ks = current.node.keys.to_owned();
            ks = ks | neighbour;

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
