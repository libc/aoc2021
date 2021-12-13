use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/day13.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut parts = contents.split("\n\n");

    let grid = parts.next().unwrap();
    let folds = parts.next().unwrap();

    let mut map: BTreeMap<(i32, i32), char> = grid.lines().fold(BTreeMap::new(), |mut map, l| {
        let mut grid = l.split(",");
        let x = grid.next().unwrap().parse::<i32>().unwrap();
        let y = grid.next().unwrap().parse::<i32>().unwrap();

        map.insert((x, y), '#');

        map
    });

    for (idx, f) in folds.lines().enumerate() {
        if !f.starts_with("fold along ") {
            panic!("wrong fold {}", f);
        }

        let axis = f["fold along ".len().."fold along x".len()].to_owned();
        let mut parsed_fold = f.split("=");
        parsed_fold.next();
        let value = parsed_fold.next().unwrap().parse::<i32>().unwrap();

        map = map
            .into_iter()
            .fold(BTreeMap::new(), |mut map, ((x, y), _)| {
                if axis == "x" {
                    if x > value {
                        map.insert((value * 2 - x, y), '#');
                    } else {
                        map.insert((x, y), '#');
                    }
                } else {
                    if y > value {
                        map.insert((x, value * 2 - y), '#');
                    } else {
                        map.insert((x, y), '#');
                    }
                }
                map
            });

        if idx == 0 {
            println!("answer1 {}", map.len());
        }
    }

    let (max_x, max_y) = map.iter().fold((0, 0), |(max_x, max_y), ((x, y), _)| {
        let max_x = if max_x < *x { *x } else { max_x };
        let max_y = if max_y < *y { *y } else { max_y };
        (max_x, max_y)
    });

    for y in 0..=max_y {
        for x in 0..=max_x {
            if map.contains_key(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }

    Ok(())
}
