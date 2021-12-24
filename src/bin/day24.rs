use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/day24.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let instructions = contents
        .lines()
        .map(|l| {
            let mut parts = l.split(' ');
            let cmd = parts.next().unwrap();
            let arg1 = parse_register(parts.next().unwrap()).unwrap();
            let arg2 = parts.next().and_then(|p| parse_register_or_number(p));

            match cmd {
                "inp" => I::Inp(arg1),
                "add" => I::Add(arg1, arg2.unwrap()),
                "mul" => I::Mul(arg1, arg2.unwrap()),
                "div" => I::Div(arg1, arg2.unwrap()),
                "mod" => I::Mod(arg1, arg2.unwrap()),
                "eql" => I::Eql(arg1, arg2.unwrap()),
                _ => panic!("unknown instruction {}", cmd),
            }
        })
        .collect();

    let max: Vec<i64> = eval(&instructions, true)
        .into_iter()
        .max()
        .unwrap_or(Vec::new());
    println!(
        "answer1 {}",
        max.iter()
            .map(|f| format!("{}", f))
            .collect::<Vec<String>>()
            .join("")
    );
    let min: Vec<i64> = eval(&instructions, false)
        .into_iter()
        .min()
        .unwrap_or(Vec::new());
    println!(
        "answer2 {}",
        min.iter()
            .map(|f| format!("{}", f))
            .collect::<Vec<String>>()
            .join("")
    );

    Ok(())
}

#[derive(Debug)]
enum I {
    Inp(R),
    Add(R, RN),
    Mul(R, RN),
    Div(R, RN),
    Mod(R, RN),
    Eql(R, RN),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum R {
    W,
    X,
    Y,
    Z,
}

#[derive(Debug)]
enum RN {
    R(R),
    N(i64),
}

fn eval(instructions: &Vec<I>, part1: bool) -> Vec<Vec<i64>> {
    let mut states: Vec<(Vec<i64>, BTreeMap<R, i64>)> = Vec::new();

    states.push((Vec::new(), BTreeMap::new()));

    for i in instructions {
        match i {
            I::Inp(r) => {
                states = states
                    .into_iter()
                    .fold(BTreeMap::new(), |mut entries, (path, rs)| {
                        let e = entries.entry(rs).or_insert(path.clone());
                        if part1 {
                            if *e < path {
                                *e = path
                            }
                        } else {
                            if *e > path {
                                *e = path
                            }
                        }

                        entries
                    })
                    .into_iter()
                    .map(|(rs, path)| (path, rs))
                    .collect();

                states = states
                    .into_iter()
                    .flat_map(|(path, rs)| {
                        (1..10)
                            .map(|w| {
                                let mut path = path.clone();
                                path.push(w);
                                let mut rs = rs.clone();
                                *rs.entry(*r).or_insert(0) = w;

                                (path, rs)
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect();
                println!("created {}", states.len());
            }
            I::Add(a, b) => {
                states = states
                    .into_iter()
                    .map(|(path, mut rs)| {
                        let b = match b {
                            RN::R(r) => rs.get(r).unwrap_or(&0).clone(),
                            RN::N(n) => *n,
                        };

                        *rs.entry(a.clone()).or_insert(0) += b;
                        (path, rs)
                    })
                    .collect();
            }
            I::Mul(a, b) => {
                states = states
                    .into_iter()
                    .map(|(path, mut rs)| {
                        let b = match b {
                            RN::R(r) => rs.get(r).unwrap_or(&0).clone(),
                            RN::N(n) => *n,
                        };

                        *rs.entry(*a).or_insert(0) *= b;
                        (path, rs)
                    })
                    .collect();
            }
            I::Div(a, b) => {
                states = states
                    .into_iter()
                    .filter_map(|(path, mut rs)| {
                        let b = match b {
                            RN::R(r) => rs.get(r).unwrap_or(&0).clone(),
                            RN::N(n) => *n,
                        };

                        if b == 0 {
                            return None;
                        }

                        *rs.entry(*a).or_insert(0) /= b;

                        Some((path, rs))
                    })
                    .collect();
            }
            I::Mod(a, b) => {
                states = states
                    .into_iter()
                    .filter_map(|(path, mut rs)| {
                        let b = match b {
                            RN::R(r) => rs.get(r).unwrap_or(&0).clone(),
                            RN::N(n) => *n,
                        };

                        if b <= 0 {
                            return None;
                        }

                        let a = rs.entry(*a).or_insert(0);
                        if *a < 0 {
                            return None;
                        }

                        *a = (*a) % b;

                        Some((path, rs))
                    })
                    .collect();
            }
            I::Eql(a, b) => {
                states = states
                    .into_iter()
                    .map(|(path, mut rs)| {
                        let b = match b {
                            RN::R(r) => rs.get(r).unwrap_or(&0).clone(),
                            RN::N(n) => *n,
                        };
                        let a = rs.entry(*a).or_insert(0);

                        *a = if *a == b { 1 } else { 0 };
                        (path, rs)
                    })
                    .collect();
            }
        }
    }

    states
        .into_iter()
        .filter(|(_path, rs)| *rs.get(&R::Z).unwrap_or(&1) == 0)
        .map(|(path, _)| path)
        .collect()
}

fn parse_register(r: &str) -> Option<R> {
    match r {
        "w" => Some(R::W),
        "x" => Some(R::X),
        "y" => Some(R::Y),
        "z" => Some(R::Z),
        _ => None,
    }
}

fn parse_register_or_number(r: &str) -> Option<RN> {
    parse_register(r)
        .map(|r| RN::R(r))
        .or(r.parse::<i64>().ok().map(|n| RN::N(n)))
}
