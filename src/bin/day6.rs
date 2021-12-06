use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/day6.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut numbers = contents
        .trim()
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    for _ in 0..80 {
        let zeroes = numbers.iter().filter(|n| **n == 0).count();

        (0..zeroes).for_each(|_| numbers.push(9));

        numbers = numbers
            .into_iter()
            .map(|n| if n == 0 { 6 } else { n - 1 })
            .collect::<Vec<_>>();
    }

    println!("answer1 {}", numbers.len());

    let numbers = contents
        .trim()
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut fish: Vec<usize> = Vec::new();
    fish.resize(10, 0);

    numbers.into_iter().for_each(|n| fish[n as usize] += 1);

    for day in 0..257 {
        if day == 80 {
            println!("answer1 (again) {}", fish.iter().sum::<usize>());
        }
        if day == 256 {
            println!("answer2 {}", fish.iter().sum::<usize>());
        }
        let mut new_fish = Vec::new();
        new_fish.resize(10, 0);

        new_fish[8] = fish[0];
        new_fish[6] = fish[0];
        for x in 1..10 {
            new_fish[x - 1] += fish[x]
        }

        fish = new_fish;
    }

    Ok(())
}
