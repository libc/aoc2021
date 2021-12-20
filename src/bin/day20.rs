use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/day20.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut instructions = contents.split("\n\n");
    let filter: Vec<char> = instructions.next().unwrap().chars().collect();

    let grid = instructions
        .next()
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| ((x as i32, y as i32), c))
                .collect::<Vec<((i32, i32), char)>>()
        })
        .collect::<BTreeMap<(i32, i32), char>>();

    let mut g = grid.clone();
    for i in 0..50 {
        g = if i % 2 == 0 {
            // This is not entirely correct, because it starts with .,
            // then it needs to be filter[511] (which is . in my case)
            apply_transform(&filter, &g, '.')
        } else {
            apply_transform(&filter, &g, filter[0])
        };
        if i == 1 {
            println!("answer1 {}", g.iter().filter(|(_, c)| **c == '#').count());
        }
        if i == 49 {
            println!("answer2 {}", g.iter().filter(|(_, c)| **c == '#').count());
        }
    }

    Ok(())
}

fn apply_transform(
    filter: &Vec<char>,
    grid: &BTreeMap<(i32, i32), char>,
    empty: char,
) -> BTreeMap<(i32, i32), char> {
    let (min_x, min_y, max_x, max_y) =
        grid.iter()
            .fold((1, 1, 0, 0), |(min_x, min_y, max_x, max_y), ((x, y), _)| {
                let min_x = if *x <= min_x { *x - 2 } else { min_x };
                let min_y = if *y <= min_y { *y - 2 } else { min_y };
                let max_x = if *x >= max_x { *x + 2 } else { max_x };
                let max_y = if *y >= max_y { *y + 2 } else { max_y };

                (min_x, min_y, max_x, max_y)
            });

    let mut output = BTreeMap::new();
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let idx = vec![
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (0, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ]
            .into_iter()
            .map(|(dx, dy)| match grid.get(&(x + dx, y + dy)) {
                None => empty,
                Some(v) => *v,
            })
            .map(|c| match c {
                '.' => 0,
                '#' => 1,
                v @ _ => panic!("unknown character in the grid {}", v),
            })
            .fold(0, |v, c| (v << 1) + c);

            output.insert((x, y), filter[idx]);
        }
    }

    output
}
