use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/day25.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut grid = contents
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| ((x, y), c))
                .collect::<Vec<((usize, usize), char)>>()
        })
        .collect::<BTreeMap<(usize, usize), char>>();

    let (boundary_x, boundary_y): (usize, usize) =
        grid.iter().fold((0, 0), |(max_x, max_y), ((x, y), _)| {
            let max_x = if *x >= max_x { x + 1 } else { max_x };
            let max_y = if *y >= max_y { y + 1 } else { max_y };

            (max_x, max_y)
        });

    let mut turn = 0;
    let mut moved = true;
    println!("boundary: {} {}", boundary_x, boundary_y);

    while moved {
        moved = false;
        let mut new_grid: BTreeMap<(usize, usize), char> = BTreeMap::new();

        for ((x, y), c) in grid.iter() {
            match c {
                '.' => continue,
                '>' => {
                    let next_x = (x + 1) % boundary_x;
                    let next_y = *y;
                    if *grid.get(&(next_x, next_y)).unwrap_or(&'.') == '.' {
                        moved = true;
                        new_grid.insert((next_x, next_y), '>');
                    } else {
                        new_grid.insert((*x, *y), '>');
                    }
                }
                'v' => {
                    new_grid.insert((*x, *y), 'v');
                }
                _ => {
                    panic!("it's not happening")
                }
            }
        }
        grid = new_grid;
        new_grid = BTreeMap::new();
        for ((x, y), c) in grid.iter() {
            match c {
                '.' => continue,
                '>' => {
                    new_grid.insert((*x, *y), '>');
                }
                'v' => {
                    let next_x = *x;
                    let next_y = (y + 1) % boundary_y;
                    if *grid.get(&(next_x, next_y)).unwrap_or(&'.') == '.' {
                        moved = true;
                        new_grid.insert((next_x, next_y), 'v');
                    } else {
                        new_grid.insert((*x, *y), 'v');
                    }
                }
                _ => {
                    panic!("it's not happening")
                }
            }
        }

        turn += 1;
        grid = new_grid;
    }

    println!("answer1 {}", turn);

    Ok(())
}
