use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/day8.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let digits = contents
        .lines()
        .map(|l| {
            let mut iter = l.split(" | ");

            let solved = solve(iter.next().unwrap());
            iter.next()
                .unwrap()
                .split(" ")
                .map(|s| solved[&s.chars().collect()])
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    println!(
        "answer1 {}",
        digits
            .iter()
            .flat_map(|d| d.iter())
            .filter(|d| {
                let d = **d;
                d == 1 || d == 4 || d == 7 || d == 8
            })
            .count()
    );

    println!(
        "answer2 {}",
        digits
            .iter()
            .map(|d| d.iter().fold(0, |od, d| od * 10 + d))
            .sum::<usize>()
    );

    Ok(())
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum S {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

fn all_digits() -> Vec<HashSet<S>> {
    vec![
        // 0
        vec![S::A, S::B, S::C, S::E, S::F, S::G]
            .into_iter()
            .collect(),
        // 1
        vec![S::C, S::F].into_iter().collect(),
        // 2
        vec![S::A, S::C, S::D, S::E, S::G].into_iter().collect(),
        // 3
        vec![S::A, S::C, S::D, S::F, S::G].into_iter().collect(),
        // 4
        vec![S::B, S::C, S::D, S::F].into_iter().collect(),
        // 5
        vec![S::A, S::B, S::D, S::F, S::G].into_iter().collect(),
        // 6
        vec![S::A, S::B, S::D, S::E, S::F, S::G]
            .into_iter()
            .collect(),
        // 7
        vec![S::A, S::C, S::F].into_iter().collect(),
        // 8
        vec![S::A, S::B, S::C, S::D, S::E, S::F, S::G]
            .into_iter()
            .collect(),
        // 9
        vec![S::A, S::B, S::C, S::D, S::F, S::G]
            .into_iter()
            .collect(),
    ]
}

fn solve(input: &str) -> BTreeMap<BTreeSet<char>, usize> {
    let digits = all_digits();
    let chars = input
        .split(" ")
        .map(|c| c.chars().collect::<HashSet<char>>())
        .collect::<Vec<_>>();

    let mut known_wires: HashSet<S> = HashSet::new();
    let mut known_chars: HashSet<char> = HashSet::new();
    let mut result: HashMap<S, char> = HashMap::new();

    loop {
        let mut found = false;
        for d in digits.iter() {
            let ud = &d.clone() - &known_wires;
            let candidates = digits
                .iter()
                .filter(|od| {
                    let uod = od.clone() - &known_wires;

                    (&uod.clone() - &ud).len() == 1

                    // ud.len() == uod.len() + 1 && ud.is_superset(&uod)
                })
                .collect::<Vec<_>>();

            if candidates.len() == 0 {
                continue;
            }

            let diff = candidates[0] - d;
            if !candidates.iter().all(|cd| (cd.clone() - d) == diff) {
                continue;
            }

            let od = candidates[0];

            let chs = chars
                .iter()
                .filter(|c| {
                    c.len() == d.len()
                        && d.intersection(&known_wires).all(|w| c.contains(&result[w]))
                })
                .collect::<Vec<_>>();

            if chs.len() != 1 {
                continue;
            }
            let chs = chs[0];

            let o_chs = candidates
                .iter()
                .map(|od| {
                    chars
                        .iter()
                        .filter(|c| {
                            c.len() == od.len()
                                && od
                                    .intersection(&known_wires)
                                    .all(|w| c.contains(&result[w]))
                                && (&(c.clone() - chs).clone() - &known_chars).len() == 1
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            let o_chs = o_chs
                .into_iter()
                .filter(|s| s.len() == 1)
                .collect::<Vec<_>>();
            if o_chs.len() == 0 {
                continue;
            }

            if o_chs.iter().all(|s| s.len() == 1) == false {
                continue;
            }

            let new_wire = (&(od - d).clone() - &known_wires)
                .iter()
                .nth(0)
                .unwrap()
                .clone();
            let new_char = (&(o_chs[0][0] - chs).clone() - &known_chars)
                .iter()
                .nth(0)
                .unwrap()
                .clone();

            result.insert(new_wire.clone(), new_char.clone());
            known_wires.insert(new_wire.clone());
            known_chars.insert(new_char.clone());
            found = true;
        }
        if !found {
            break;
        }
    }

    digits
        .iter()
        .enumerate()
        .map(|(idx, ds)| {
            (
                ds.iter().map(|w| result[w]).collect::<BTreeSet<char>>(),
                idx,
            )
        })
        .collect()
}
