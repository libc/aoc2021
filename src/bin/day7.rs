use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/day7.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let numbers = contents
        .trim()
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let min_fuel = (0..*numbers.iter().max().unwrap_or(&0))
        .map(|n| numbers.iter().map(|on| (n - on).abs()).sum())
        .min()
        .unwrap_or(0);

    println!("answer1 {}", min_fuel);

    let min_fuel = (0..*numbers.iter().max().unwrap_or(&0))
        .map(|a| numbers.iter().map(|b| fuel_cost(a, *b)).sum())
        .min()
        .unwrap_or(0);

    println!("answer2 {}", min_fuel);

    Ok(())
}

fn fuel_cost(a: i32, b: i32) -> i32 {
    let min = if a < b { a } else { b };
    let max = if a > b { a } else { b };

    let n = max - min;

    n * (2 + n - 1) / 2
}
