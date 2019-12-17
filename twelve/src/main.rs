use num;
fn main() {
    let real_moons = vec![
        Moon::new((-7, 0), (17, 0), (-11, 0)),
        Moon::new((9, 0), (12, 0), (5, 0)),
        Moon::new((-9, 0), (0, 0), (-4, 0)),
        Moon::new((4, 0), (6, 0), (0, 0)),
    ];
    /*let test_moons = vec![
        Moon::new((-1, 0), (0, 0), (2, 0)),
        Moon::new((2, 0), (-10, 0), (-7, 0)),
        Moon::new((4, 0), (-8, 0), (8, 0)),
        Moon::new((3, 0), (5, 0), (-1, 0)),
    ];*/
    part1(real_moons.to_owned());
    println!("{}", find_cycle_3(real_moons));
}

fn part1(moons: Vec<Moon>) {
    let moons = step(moons, 1000);
    let energy: i32 = moons
        .into_iter()
        .map(|moon| {
            let pot = moon.x.position.abs() + moon.y.position.abs() + moon.z.position.abs();
            let kin = moon.x.velocity.abs() + moon.y.velocity.abs() + moon.z.velocity.abs();
            pot * kin
        })
        .sum();
    println!("{}", energy);
}

fn step(moons: Vec<Moon>, steps: usize) -> Vec<Moon> {
    let mut xs: Vec<Moon1> = moons.iter().map(|moon| moon.x.to_owned()).collect();
    for _i in 0..steps {
        xs = step1(xs);
    }
    let mut ys: Vec<Moon1> = moons.iter().map(|moon| moon.y.to_owned()).collect();
    for _i in 0..steps {
        ys = step1(ys);
    }
    let mut zs: Vec<Moon1> = moons.iter().map(|moon| moon.z.to_owned()).collect();
    for _i in 0..steps {
        zs = step1(zs);
    }
    xs.into_iter()
        .zip(ys.into_iter().zip(zs.into_iter()))
        .map(|(x, (y, z))| Moon { x, y, z })
        .collect()
}

fn find_cycle_3(moons: Vec<Moon>) -> i64 {
    let xs: Vec<Moon1> = moons.iter().map(|moon| moon.x.to_owned()).collect();
    let ys: Vec<Moon1> = moons.iter().map(|moon| moon.y.to_owned()).collect();
    let zs: Vec<Moon1> = moons.iter().map(|moon| moon.z.to_owned()).collect();
    num::integer::lcm(
        num::integer::lcm(find_cycle_time(xs), find_cycle_time(ys)),
        find_cycle_time(zs),
    )
}

fn find_cycle_time(moons: Vec<Moon1>) -> i64 {
    let mut count = 0;
    let mut changing = moons.to_owned();
    loop {
        changing = step1(changing);
        count += 1;
        if changing.iter().zip(moons.iter()).all(|(l, r)| l == r) {
            break;
        }
    }
    count
}

fn step1(moons: Vec<Moon1>) -> Vec<Moon1> {
    moons
        .iter()
        .map(|moon| {
            let v_change: i32 = moons
                .iter()
                .map(|m| {
                    if m.position > moon.position {
                        1
                    } else if m.position < moon.position {
                        -1
                    } else {
                        0
                    }
                })
                .sum();
            let new_v = moon.velocity + v_change;
            Moon1 {
                position: moon.position + new_v,
                velocity: new_v,
            }
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
struct Moon1 {
    position: i32,
    velocity: i32,
}

impl Moon1 {
    fn new((position, velocity): (i32, i32)) -> Moon1 {
        Moon1 { position, velocity }
    }
}

#[derive(Debug, Clone)]
struct Moon {
    x: Moon1,
    y: Moon1,
    z: Moon1,
}

impl Moon {
    fn new(x: (i32, i32), y: (i32, i32), z: (i32, i32)) -> Moon {
        Moon {
            x: Moon1::new(x),
            y: Moon1::new(y),
            z: Moon1::new(z),
        }
    }
}
