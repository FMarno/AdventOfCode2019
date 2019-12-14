use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("one/input.txt").expect("Couldn't open file");
    let reader = BufReader::new(file);
    let x: i32 = reader
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let mass = line.parse::<i32>().unwrap();
            needed_fuel(mass)
        })
        .sum();

    println!("{}", x);
}

fn needed_fuel(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel > 0 {
        let extra = needed_fuel(fuel);
        return fuel + extra;
    } else {
        return 0;
    }
}
