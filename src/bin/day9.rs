use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/day9.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let map = contents
        .lines()
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    println!(
        "answer1 {}",
        (0..map.len())
            .map(|y| {
                (0..map[y as usize].len())
                    .filter(|x| lowest(&map, *x as i32, y as i32))
                    .map(|x| map[y as usize][x as usize] + 1)
                    .sum::<i32>()
            })
            .sum::<i32>()
    );

    let lowest_points = (0..map.len())
        .flat_map(|y| {
            (0..map[y as usize].len())
                .filter(|x| lowest(&map, *x as i32, y as i32))
                .map(|x| (x as i32, y as i32))
                .collect::<Vec<(i32, i32)>>()
        })
        .collect::<Vec<(i32, i32)>>();

    let mut basins = lowest_points
        .into_iter()
        .map(|(x, y)| expand(&map, x, y))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    basins.sort_by(|set1, set2| set2.len().partial_cmp(&set1.len()).unwrap());

    println!(
        "answer2 {}",
        basins
            .into_iter()
            .take(3)
            .map(|s| s.len())
            .fold(1, |l, s| l * s)
    );

    Ok(())
}

fn lowest(map: &Vec<Vec<i32>>, x: i32, y: i32) -> bool {
    let point = map[y as usize][x as usize];

    vec![(1, 0), (-1, 0), (0, -1), (0, 1)]
        .into_iter()
        .all(|(dx, dy)| {
            match map
                .get((y + dy) as usize)
                .and_then(|row| row.get((x + dx) as usize))
            {
                None => true,
                Some(v) => *v > point,
            }
        })
}

fn expand(map: &Vec<Vec<i32>>, x: i32, y: i32) -> BTreeSet<(i32, i32)> {
    let mut result = BTreeSet::new();
    result.insert((x, y));

    loop {
        let new_result = result
            .iter()
            .flat_map(|(x, y)| {
                vec![(0, 0), (1, 0), (-1, 0), (0, -1), (0, 1)]
                    .into_iter()
                    .filter_map(|(dx, dy)| {
                        map.get((y + dy) as usize)
                            .and_then(|row| row.get((x + dx) as usize))
                            .and_then(|v| {
                                if *v == 9 {
                                    None
                                } else {
                                    Some(((x + dx).clone(), (y + dy).clone()))
                                }
                            })
                    })
                    .collect::<BTreeSet<(i32, i32)>>()
            })
            .collect::<BTreeSet<(i32, i32)>>();

        if new_result == result {
            return result;
        }

        result = new_result;
    }
}
