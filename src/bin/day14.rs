use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/day14.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut parts = contents.split("\n\n");

    let template = parts.next().unwrap();
    let pairs = parts.next().unwrap();

    let patterns = pairs
        .lines()
        .map(|pair| {
            let mut kv = pair.split(" -> ");
            let key = kv.next().unwrap();
            let value = kv.next().unwrap();

            (key.to_owned(), value.to_owned())
        })
        .collect::<BTreeMap<String, String>>();

    let mut string = template.to_owned();
    for day in 0..10 {
        string = string
            .chars()
            .zip(string[1..].chars().chain(vec![' '].into_iter()))
            .flat_map(|(c1, c2)| match patterns.get(&format!("{}{}", c1, c2)) {
                None => vec![c1],
                Some(v) => vec![c1, v.chars().next().unwrap()],
            })
            .collect();

        let counts: BTreeMap<char, usize> =
            string.chars().fold(BTreeMap::new(), |mut counts, c| {
                let count = counts.entry(c).or_insert(0);
                *count += 1;
                counts
            });
        println!("day {} counts: {:?}", day, counts);
    }

    let counts: BTreeMap<char, usize> = string.chars().fold(BTreeMap::new(), |mut counts, c| {
        let count = counts.entry(c).or_insert(0);
        *count += 1;
        counts
    });

    let (min, max) = counts.iter().fold((0, 0), |(min, max), (_, c)| {
        let min = if min == 0 || *c < min { *c } else { min };
        let max = if max < *c { *c } else { max };

        (min, max)
    });

    println!("answer1 {}", max - min);

    Ok(())
}
