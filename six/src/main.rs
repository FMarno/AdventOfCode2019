use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let f = File::open("six/input")?;
    let reader = BufReader::new(f);
    let mut masses = HashMap::new();
    for l in reader.lines() {
        let l = l.unwrap();
        let parts: Vec<_> = l.split(')').collect();
        let major = parts[0].to_owned();
        let minor = parts[1].to_owned();
        let orbitors = masses.entry(major).or_insert_with(Vec::new);
        (*orbitors).push(minor);
    }
    let mut santa = path_to("SAN", "COM", &masses).unwrap();
    let mut you = path_to("YOU", "COM", &masses).unwrap();
    loop {
        if santa.last() == you.last() {
            santa.pop();
            you.pop();
        } else {
            break;
        }
    }
    println!("{}", santa.len() + you.len() - 2);
    Ok(())
}

fn _count_orbits(masses: HashMap<String, Vec<String>>) -> i32 {
    _count_orbits_recursive("COM", &masses, 0)
}

fn _count_orbits_recursive(mass: &str, masses: &HashMap<String, Vec<String>>, level: i32) -> i32 {
    let children = match masses.get(mass) {
        Some(x) => x,
        None => {
            println!("leaf {}, level {}", mass, level);
            return level;
        }
    };
    let child_orbits: i32 = children
        .iter()
        .map(|x| _count_orbits_recursive(x, masses, level + 1))
        .sum();
    level + child_orbits
}

fn path_to(
    goal: &str,
    current: &str,
    masses: &HashMap<String, Vec<String>>,
) -> Option<Vec<String>> {
    let children = match masses.get(current) {
        Some(x) => x,
        None => {
            return None;
        }
    };
    if children.iter().any(|c| c == goal) {
        return Some(vec![goal.to_owned(), current.to_owned()]);
    }
    let mut path: Vec<_> = children
        .iter()
        .filter_map(|x| path_to(goal, x, masses))
        .collect();
    match path.len() {
        0 => None,
        1 => {
            let mut path = path.pop().unwrap();
            path.push(current.to_owned());
            Some(path)
        }
        _ => {
            println!("Fucked it");
            panic!();
        }
    }
}
