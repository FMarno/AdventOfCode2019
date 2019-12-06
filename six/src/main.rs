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
    for (k,v) in masses {
        println!("{} {}", k, v.len());
    }
    Ok(())
}

fn count_orbits(masses : HashMap<String, Vec<String>>) -> i32 {
    return 0;
}
