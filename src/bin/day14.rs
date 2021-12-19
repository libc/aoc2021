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
            let mut key = kv.next().unwrap().chars();
            let mut value = kv.next().unwrap().chars();

            (
                (key.next().unwrap(), key.next().unwrap()),
                value.next().unwrap(),
            )
        })
        .collect::<BTreeMap<(char, char), char>>();

    let mut string = template.to_owned();
    for day in 0..10 {
        string = string
            .chars()
            .zip(string[1..].chars().chain(vec![' '].into_iter()))
            .flat_map(|(c1, c2)| match patterns.get(&(c1, c2)) {
                None => vec![c1],
                Some(v) => vec![c1, *v],
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

    println!("{:?}", counts);
    println!(
        "{:?}",
        Calculator::new(&patterns).expand_string(&template.to_owned(), 10)
    );
    println!(
        "{:?}",
        Calculator::new(&patterns).expand_string(&template.to_owned(), 40)
    );

    let counts = Calculator::new(&patterns).expand_string(&template.to_owned(), 40);
    let (min, max) = counts.iter().fold((0, 0), |(min, max), (_, c)| {
        let min = if min == 0 || *c < min { *c } else { min };
        let max = if max < *c { *c } else { max };

        (min, max)
    });

    println!("answer2 {}", max - min);

    Ok(())
}

struct Calculator {
    patterns: BTreeMap<(char, char), char>,
    cache: BTreeMap<(char, char, usize), BTreeMap<char, usize>>,
}
impl Calculator {
    fn new(patterns: &BTreeMap<(char, char), char>) -> Self {
        Self {
            patterns: patterns.clone(),
            cache: BTreeMap::new(),
        }
    }

    fn expand_string(&mut self, s: &String, l: usize) -> BTreeMap<char, usize> {
        s.chars()
            .zip(s[1..].chars().chain(vec![' '].into_iter()))
            .map(|(c1, c2)| self.expand(c1, c2, l))
            .reduce(|a, b| {
                let mut treemap = BTreeMap::new();
                for (char, count) in a {
                    *treemap.entry(char).or_insert(0) += count;
                }
                for (char, count) in b {
                    *treemap.entry(char.clone()).or_insert(0) += count;
                }

                treemap
            })
            .unwrap()
    }

    fn expand(&mut self, a: char, b: char, l: usize) -> BTreeMap<char, usize> {
        if l == 0 {
            let mut treemap = BTreeMap::new();
            treemap.insert(a, 1);
            return treemap;
        }

        let mid = self.patterns.get(&(a, b));
        if mid.is_none() {
            let mut treemap = BTreeMap::new();
            treemap.insert(a, 1);
            return treemap;
        }

        let cached = self.cache.get(&(a, b, l));
        if cached.is_some() {
            return cached.unwrap().clone();
        }

        let mid = mid.unwrap().clone();

        let a_count = self.expand(a, mid, l - 1);
        let b_count = self.expand(mid, b, l - 1);

        let mut treemap = BTreeMap::new();
        for (char, count) in a_count {
            *treemap.entry(char).or_insert(0) += count;
        }
        for (char, count) in b_count {
            *treemap.entry(char.clone()).or_insert(0) += count;
        }

        self.cache.insert((a, b, l), treemap.clone());

        return treemap;
    }
}
