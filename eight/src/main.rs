use image;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let width = 25;
    let height = 6;
    let length = width * height;
    let f = File::open("eight/input").expect("Can't open file");
    let mut layers: Vec<Vec<u8>> = Vec::new();
    let mut input = f.bytes().take_while(|x| x.is_ok()).map(|x| x.unwrap());
    loop {
        let next_layer: Vec<_> = input.by_ref().take(length).collect();
        if next_layer.len() != length {
            break;
        } else {
            layers.push(next_layer);
        }
    }
    let message = stack_layers(layers, length);
    let message: Vec<u8> = message
        .iter()
        .map(|p| if *p == '0' as u8 { 0 } else { 255 })
        .collect();
    let saved = image::save_buffer(
        "eight/out.png",
        &message,
        width as u32,
        height as u32,
        image::ColorType::Gray(8),
    );
    match saved {
        Ok(()) => println!("Success"),
        Err(e) => println!("err {}", e),
    }
}

fn stack_layers(layers: Vec<Vec<u8>>, length: usize) -> Vec<u8> {
    let mut stack = Vec::new();
    stack.reserve_exact(length);
    for i in 0..length {
        let next_pixel = layers
            .iter()
            .map(|l| l.get(i).unwrap())
            .filter(|pixel| **pixel != '2' as u8)
            .next();
        stack.push(*next_pixel.unwrap());
    }
    stack
}

fn _check_sum(layers: Vec<Vec<u8>>) {
    let layer = layers
        .iter()
        .min_by(|l, r| _count_char(l, '0').cmp(&_count_char(r, '0')))
        .unwrap();

    println!("{}", _count_char(&layer, '1') * _count_char(&layer, '2'));
}

fn _count_char(v: &Vec<u8>, c: char) -> usize {
    v.iter().filter(|x| **x == (c as u8)).count()
}
