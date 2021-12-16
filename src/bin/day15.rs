use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/day15.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let grid: Vec<Vec<usize>> = contents
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| format!("{}", c).parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    println!("answer1 {}", find_path(&grid));

    let mut new_grid: Vec<Vec<usize>> = Vec::new();

    for _ in 0..grid.len() {
        new_grid.push(Vec::new());
    }

    for i in 0..5 {
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                new_grid[y].push(add(grid[y][x], i));
            }
        }
    }

    for i in 1..5 {
        for y in 0..grid.len() {
            new_grid.push(new_grid[y].iter().map(|x| add(*x, i)).collect());
        }
    }

    println!("answer2 {}", find_path(&new_grid));

    Ok(())
}

fn find_path(grid: &Vec<Vec<usize>>) -> usize {
    let mut queue: BinaryHeap<Reverse<(usize, (usize, usize))>> = BinaryHeap::new();
    let mut min_costs = BTreeMap::new();

    queue.push(Reverse((0, (0, 0))));

    let end_y = grid.len() - 1;
    let end_x = grid[end_y].len() - 1;

    while queue.len() > 0 {
        let Reverse((cost, (x, y))) = queue.pop().unwrap();

        if x == end_x && y == end_y {
            return cost;
        }

        let positions: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        for (dx, dy) in positions {
            if x == 0 && dx == -1 {
                continue;
            }
            if y == 0 && dy == -1 {
                continue;
            }

            let nx = ((x as i32) + dx) as usize;
            let ny = ((y as i32) + dy) as usize;

            if nx >= grid[y].len() || ny >= grid.len() {
                continue;
            }
            let cost = cost + grid[ny][nx];
            let old_cost = *min_costs.get(&(nx, ny)).unwrap_or(&0);

            if old_cost == 0 || old_cost > cost {
                queue.push(Reverse((cost, (nx, ny))));
                min_costs.insert((nx, ny), cost);
            }
        }
    }

    panic!("not found");
}

fn add(x: usize, j: usize) -> usize {
    let x = x + j;
    if x > 9 {
        x - 9
    } else {
        x
    }
}
