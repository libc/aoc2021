use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/day3.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let bit_counts = count_bits(&contents.lines().map(|l| l.to_owned()).collect());

    let (g, eps) = bit_counts
        .iter()
        .fold((0, 0), |(gamma, epsilon), (zeroes, ones)| {
            if zeroes == ones {
                panic!("don't know what do do")
            }

            if zeroes > ones {
                (gamma * 2 + 1, epsilon * 2)
            } else {
                (gamma * 2, epsilon * 2 + 1)
            }
        });

    println!("answer1 {}", g * eps);

    let oxygen = sieve(
        &contents,
        |(zeroes, ones)| if ones >= zeroes { '1' } else { '0' },
    );
    let co2 = sieve(
        &contents,
        |(zeroes, ones)| if ones < zeroes { '1' } else { '0' },
    );

    println!("anser2 {}", oxygen * co2);

    Ok(())
}

fn sieve<F>(input: &String, f: F) -> usize
where
    F: Fn((usize, usize)) -> char,
{
    let mut numbers: Vec<String> = input.lines().map(|l| l.to_owned()).collect();
    let mut idx = 0;

    while numbers.len() > 1 {
        let bit_counts = count_bits(&numbers);

        let c = f(bit_counts[idx]);

        numbers = numbers
            .into_iter()
            .filter(|l| l.as_bytes()[idx] as char == c)
            .collect();
        idx += 1;
    }

    numbers[0]
        .chars()
        .fold(0, |o, b| o * 2 + if b == '1' { 1 } else { 0 })
}

fn count_bits(lines: &Vec<String>) -> Vec<(usize, usize)> {
    let mut bit_counts: Vec<(usize, usize)> = Vec::new();
    lines.iter().for_each(|l| {
        l.chars().enumerate().for_each(|(idx, bit)| {
            if bit_counts.len() <= idx {
                bit_counts.push((0, 0));
            }

            if bit == '0' {
                bit_counts[idx].0 += 1
            } else {
                bit_counts[idx].1 += 1
            }
        });
    });

    bit_counts
}
