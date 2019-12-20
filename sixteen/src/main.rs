use std::fs;
use std::iter;

fn main() {
    let code = fs::read_to_string("sixteen/input").unwrap();
    let mut seq: Vec<i32> = code
        .chars()
        .filter_map(|x| x.to_digit(10))
        .map(|x| x as i32)
        .collect();
    for _ in 0..100 {
        seq = phase(seq);
    }
    println!("{:?}", seq);
    let out = seq[0..8].to_vec();
    println!("{:?}", out);
}

fn phase(numbers: Vec<i32>) -> Vec<i32> {
    let n_len = numbers.len();
    (1..n_len + 1)
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

fn _gen_pattern(r: usize) -> impl Iterator<Item = i32> {
    iter::repeat(0)
        .take(r)
        .chain(iter::repeat(1).take(r))
        .chain(iter::repeat(0).take(r))
        .chain(iter::repeat(-1).take(r))
        .cycle()
        .skip(1)
}
