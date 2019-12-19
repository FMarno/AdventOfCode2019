use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn main() -> std::io::Result<()> {
    let f = File::open("fourteen/input")?;
    let reader = BufReader::new(f);
    let recipes = create_recipes(reader.lines());
    let ore: i64 = 1000000000000;
    println!("{}", cost_for_fuel(82892753, &recipes));
    println!("{}", cost_for_fuel(82892754, &recipes));
    println!("{}", possible_fuel(ore, &recipes));
    Ok(())
}

fn possible_fuel(ore :i64, recipes : &HashMap<String, Recipe>) -> i64 {
    let mut min: i64 = 1;
    let mut max :i64= ore;
    loop {
        let mid = (max + min) / 2;
        println!("{} {} {}", min, mid, max);
        let cost = cost_for_fuel(mid, recipes);
        if cost < ore {
            min = mid;
        } else {
            max = mid;
        }
        if max == min || min +1 == max{
            break;
        }
    }
    min
}

fn cost_for_fuel(amount: i64, recipes: &HashMap<String, Recipe>) -> i64 {
    let mut required: Vec<(String, i64)> = Vec::new();
    let mut spare: HashMap<String, i64> = HashMap::new();
    let mut ore = 0;
    required.push(("FUEL".to_owned(), amount));
    loop {
        let ingredient = required.pop();
        if ingredient.is_none() {
            break;
        }
        // next thing to get
        let (ingredient, mut amount) = ingredient.unwrap();
        if ingredient == "ORE" {
            ore += amount;
            continue;
        }
        // how much do we have spare
        let spare_ingredient = spare.entry(ingredient.to_owned()).or_insert(0);
        amount -= *spare_ingredient;
        if amount < 0 {
            *spare_ingredient = amount * -1;
            amount = 0;
        } else {
            *spare_ingredient = 0;
        }
        // ingredients
        let requirements = &recipes[&ingredient];
        let pack_size = requirements.pack_size;
        let whole_packs_needed = amount / pack_size;
        let pack_remainder = amount % pack_size;
        let packs_needed = whole_packs_needed + if pack_remainder > 0 { 1 } else { 0 };
        for (requirement_quantity, requirement) in &requirements.ingredients {
            match required.iter_mut().find(|r| r.0 == *requirement) {
                Some(r) => (*r).1 += requirement_quantity * packs_needed,
                None => {
                    required.push((requirement.to_owned(), requirement_quantity * packs_needed))
                }
            }
        }
        if pack_remainder > 0 {
            let spare_produce = pack_size - pack_remainder;
            *spare_ingredient += spare_produce;
        }
    }
    ore + spare.get("ORE").unwrap_or(&0)
}

#[derive(Debug)]
struct Recipe {
    pack_size: i64,
    ingredients: Vec<(i64, String)>,
}

fn create_recipes<T>(lines: Lines<T>) -> HashMap<String, Recipe>
where
    T: Sized + BufRead,
{
    let mut recipes = HashMap::new();
    for line in lines.filter_map(|l| l.ok()) {
        let sides: Vec<_> = line.split("=>").collect();
        let out: Vec<_> = sides[1].trim().split(" ").collect();
        let pack_size = out[0].parse::<i64>().unwrap();
        let out_name = out[1].to_string();
        let ingredients: Vec<_> = sides[0]
            .trim()
            .split(",")
            .map(|s| s.trim().split(" ").collect())
            .map(|r: Vec<_>| (r[0].parse::<i64>().unwrap(), r[1].to_string()))
            .collect();
        recipes.insert(
            out_name,
            Recipe {
                pack_size,
                ingredients,
            },
        );
    }
    recipes
}
