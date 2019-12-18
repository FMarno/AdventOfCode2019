use intcomputer::*;
use std::fs;

fn main() {
    //let args: Vec<_> = env::args().collect();
    let filename = "thirteen/input";
    let input = fs::read_to_string(filename).expect("can't open file");
    let mut codes: Vec<i64> = input
        .trim()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    codes[0] = 2;
    let arcade = IntComputer::new(codes);
    let score = play_game(arcade);
    println!("{}", score);
}

fn play_game(mut arcade: IntComputer) -> i64 {
    let mut score = 0;
    let mut blocks = Vec::new();
    let mut paddle = Point { x: 0, y: 0 };
    let mut ball = Point { x: 0, y: 0 };
    loop {
        let x = arcade.run_codes();
        let y = arcade.run_codes();
        let v = arcade.run_codes();
        if x.is_none() || y.is_none() || v.is_none() {
            break;
        }
        let x = x.unwrap();
        let y = y.unwrap();
        let v = v.unwrap();
        if x == -1 {
            score = v;
            continue;
        }
        match v {
            0 => (),
            1 => (),
            2 => {
                // block
                blocks.push(Point { x, y });
            }
            3 => {
                // paddle
                paddle = Point { x, y };
            }
            4 => {
                // ball
                ball = Point { x, y };
            }
            x => {
                println!("Unexpected code {}", x);
                return -1;
            }
        }
    }
    println!("\n\nStarting\n");
    loop {
        let play = if ball.x > paddle.x {
            1
        } else if ball.x < paddle.x {
            -1
        } else {
            0
        };
        match arcade.input.front_mut() {
            Some(x) => *x = play,
            None => arcade.input.push_back(play),
        }
        let x = arcade.run_codes();
        let y = arcade.run_codes();
        let v = arcade.run_codes();
        if x.is_none() || y.is_none() || v.is_none() {
            return score;
        }
        let x = x.unwrap();
        let y = y.unwrap();
        let v = v.unwrap();
        if x == -1 {
            score = v;
            continue;
        }
        match v {
            0 => {
                blocks.retain(|p| p != &Point { x, y });
            }
            3 => {
                // padle
                paddle = Point { x, y };
            }
            4 => {
                // ball
                ball = Point { x, y };
            }
            x => {
                println!("Unexpected code {}", x);
                return -1;
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}
