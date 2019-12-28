use rayon::prelude::*;
use std::fs;

fn main() {
    let code = fs::read_to_string("sixteen/input").unwrap();
    let seq: Vec<i32> = code
        .chars()
        .filter_map(|x| x.to_digit(10))
        .map(|x| x as i32)
        .collect();
    let skip = code[0..7].parse::<usize>().unwrap();
    println!("{:?}", part1(seq.to_owned()));
    println!("{:?}", part2(seq, skip));
}

fn part2(code: Vec<i32>, skip: usize) -> Vec<i32> {
    let size = code.len() * 10000;
    if skip < size / 2 {
        panic!("this doesn't work on the first half");
    }
    let mut seq: Vec<_> = code.into_iter().cycle().take(size).skip(skip).collect();
    seq.reverse();
    for _ in 0..100 {
        seq = seq
            .into_iter()
            .scan(0, |state, x| {
                *state = (*state + x).abs() % 10;
                Some(*state)
            })
            .collect();
    }
    let mut out = seq[seq.len() - 8..seq.len()].to_vec();
    out.reverse();
    out
}

fn part1(mut numbers: Vec<i32>) -> Vec<i32> {
    for _ in 0..100 {
        numbers = phase(numbers.to_owned());
    }
    numbers[0..8].to_vec()
}

fn phase(numbers: Vec<i32>) -> Vec<i32> {
    let n_len = numbers.len();
    (1..n_len + 1)
        .into_par_iter()
        .map(|i| {
            let s: i32 = numbers[i - 1..]
                .chunks(i)
                .enumerate()
                .map(|(index, c)| match index % 4 {
                    0 => c.iter().sum(),
                    1 => 0,
                    2 => -1 * (c.iter().sum::<i32>()),
                    3 => 0,
                    _ => panic!("maths broke"),
                })
                .sum();
            s.abs() % 10
        })
        .collect()
}
