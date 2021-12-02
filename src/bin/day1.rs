use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/day1.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let numbers = contents
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!(
        "answer1 {}",
        numbers
            .iter()
            .zip(numbers[1..].iter())
            .filter(|(a, b)| a < b)
            .count()
    );

    let sums = numbers
        .iter()
        .zip(numbers[1..].iter())
        .zip(numbers[2..].iter())
        .map(|((a, b), c)| a + b + c)
        .collect::<Vec<i32>>();

    println!(
        "answer2 {}",
        sums.iter()
            .zip(sums[1..].iter())
            .filter(|(a, b)| a < b)
            .count()
    );

    Ok(())
}
