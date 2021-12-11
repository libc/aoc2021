use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/day11.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut grid = contents
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<i8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    let mut flashes = 0;
    for day in 0..10000 {
        if day == 100 {
            println!("answer1 {}", flashes)
        }

        grid = grid
            .into_iter()
            .map(|row| row.into_iter().map(|c| c + 1).collect())
            .collect();

        let mut tens = grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x, c)| if *c == 10 { Some((x, y)) } else { None })
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect::<Vec<(usize, usize)>>();

        let mut step_flashes = 0;

        loop {
            let mut new_tens = Vec::new();
            for (x, y) in tens {
                grid[y][x] = 0;
                flashes += 1;
                step_flashes += 1;

                let x = x as i32;
                let y = y as i32;

                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if increment_if_not_zero(&mut grid, x + dx, y + dy) {
                            new_tens.push(((x + dx) as usize, (y + dy) as usize));
                        }
                    }
                }
            }
            if new_tens.len() == 0 {
                break;
            }
            tens = new_tens;
        }

        if step_flashes == grid.len() * grid[0].len() {
            println!("answer2 {}", day + 1);
            break;
        }
    }

    Ok(())
}

fn increment_if_not_zero(grid: &mut Vec<Vec<i8>>, x: i32, y: i32) -> bool {
    if y < 0 || x < 0 {
        return false;
    }

    let y = y as usize;
    let x = x as usize;

    if y < grid.len() && x < grid[y].len() && grid[y][x] != 0 {
        grid[y][x as usize] += 1;
        if grid[y][x] == 10 {
            true
        } else {
            false
        }
    } else {
        false
    }
}
