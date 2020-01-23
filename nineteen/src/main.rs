use intcomputer::*;
use std::fs;

fn main() {
    let filename = "nineteen/input";
    let input = fs::read_to_string(filename).expect("can't open file");
    let codes: Vec<i64> = input
        .trim()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let mut sum = 0;
    for y in 0..50 {
        for x in 0..50 {
            let mut robot = IntComputer::new(codes.to_owned());
            robot.input.push_back(x);
            robot.input.push_back(y);
            match robot.run_codes() {
                Some(o) => sum += o,
                None => println!("none"),
            }
        }
    }
    println!("{}", sum);
    let Point{x,y} = part2(&|p| lookup(codes.to_owned(), p));
    println!("{}", x*10_000+y)
}

struct Point {
    x : i64,
    y : i64,
}

fn part2(check: &dyn Fn(&Point) -> bool) -> Point {
    let mut tr = Point{x:99,y:0};
    let mut bl = Point{x:0,y:99};
    loop {
        while !check(&tr) {
            tr.y+=1;
            bl.y+=1;
        }
        while !check(&bl) {
            tr.x+=1;
            bl.x+=1;
        }
        if check(&tr) && check(&bl) {
            return Point{x: bl.x, y : tr.y};
        }
    }
}

fn lookup(codes: Vec<i64>, p: &Point) -> bool {
            let mut robot = IntComputer::new(codes);
            robot.input.push_back(p.x);
            robot.input.push_back(p.y);
            match robot.run_codes() {
                Some(o) => o == 1,
                None => panic!("bad input"),
            }

}
