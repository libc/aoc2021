use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/day10.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!(
        "answer1 {}",
        contents
            .lines()
            .map(|l| {
                match invalid_character(l) {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    ' ' => 0,
                    _ => panic!("unknown character"),
                }
            })
            .sum::<usize>()
    );

    let mut v = contents
        .lines()
        .filter_map(|l| {
            let c = complete_characters(l);
            c.map(|c| {
                c.into_iter().fold(0 as u64, |o, c| {
                    o * 5
                        + match c {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => panic!("unknown character"),
                        }
                })
            })
        })
        .collect::<Vec<_>>();

    v.sort();

    println!("answer2 {}", v[v.len() / 2]);

    Ok(())
}

fn invalid_character(l: &str) -> char {
    let mut stack = Vec::new();
    for c in l.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            c @ _ => {
                let l = stack.pop();
                if l.is_none() || l.unwrap() != c {
                    return c;
                }
            }
        }
    }

    ' '
}

fn complete_characters(l: &str) -> Option<Vec<char>> {
    let mut stack = Vec::new();
    for c in l.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            c @ _ => {
                let l = stack.pop();
                if l.is_none() || l.unwrap() != c {
                    return None;
                }
            }
        }
    }

    stack.reverse();
    Some(stack)
}
