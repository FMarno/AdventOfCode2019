fn main() {
    println!("{}, {:?}", "112233", contains_double("112232".as_bytes()));
    println!("{}, {:?}", "123444", contains_double("123444".as_bytes()));
    println!("{}, {:?}", "111122", contains_double("111122".as_bytes()));
    let range = 278384..824795;
    let possible = range
        .map(|x| x.to_string().as_bytes().to_vec())
        .filter(|x| contains_double(x))
        .filter(|x| accending(x))
        .count();
    println!("{}", possible);
}

fn contains_double<T>(number : &[T]) -> bool  where T : Eq + Copy {
    let mut i = 0;
    while i < number.len() {
        let val = number[i];
        let mut length = 0;
        while i < number.len() && number[i] == val {
            i += 1;
            length += 1;
        }
        if length == 2 {
            return true;
        }
    }
    return false;
}

fn accending(number : &[u8]) -> bool {
    for i in 1..number.len() {
        if number[i-1] > number [i] {
            return false;
        }
    }
    return true;
}
