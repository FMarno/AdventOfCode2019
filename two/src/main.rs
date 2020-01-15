use std::fs;

fn main() {
    let input = fs::read_to_string("two/input.txt").unwrap();
    let mut codes: Vec<usize> = input
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    for x in 1..99 {
        for y in 1..99 {
            codes[1] = x;
            codes[2] = y;
            let out = run_codes(codes.to_vec());
            if out == 19_690_720 {
                println!("{} {}", x, y);
                return;
            }
        }
    }
}

fn run_codes(mut codes: Vec<usize>) -> usize {
    let mut pointer = 0;
    loop {
        let code = codes[pointer];
        match code {
            1 => {
                let l = codes[pointer + 1];
                let r = codes[pointer + 2];
                let o = codes[pointer + 3];
                codes[o] = codes[l] + codes[r];
                pointer += 4;
            }
            2 => {
                let l = codes[pointer + 1];
                let r = codes[pointer + 2];
                let o = codes[pointer + 3];
                codes[o] = codes[l] * codes[r];
                pointer += 4;
            }
            99 => return codes[0],
            _ => {
                println!("error, code {}", code);
                return 0;
            }
        }
    }
}
