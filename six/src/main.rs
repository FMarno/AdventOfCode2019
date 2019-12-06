use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let f = File::open("six/input")?;
    let reader = BufReader::new(f);
    let mut masses = HashMap::new();
    for l in reader.lines(){
        let l = l.unwrap();
        let parts : Vec<_> = l.split(")").collect();
        let major = parts[0].to_owned();
        let minor = parts[1].to_owned();
        let orbitors = masses.entry(major).or_insert(Vec::new());
        (*orbitors).push(minor);
    }
    println!("{}", count_orbits(masses));
    Ok(())
}

fn count_orbits(masses : HashMap<String, Vec<String>>) -> i32 {
    return count_orbits_recursive("COM", &masses, 0);
}

fn count_orbits_recursive(mass : &str, masses : &HashMap<String, Vec<String>>, level : i32) -> i32{ 
    let children = match masses.get(mass) {
        Some(x) => x,
        None => {
            println!("leaf {}, level {}", mass, level);
            return level;
        },
    };
    let child_orbits : i32 = children.iter().map(|x| count_orbits_recursive(x, masses, level+1)).sum();
    level + child_orbits
}
