use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/day5.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines = contents
        .lines()
        .map(|l| {
            let mut points = l.split(" -> ").map(|p| {
                let mut points = p.split(",").map(|n| n.parse::<i32>().unwrap());

                (points.next().unwrap(), points.next().unwrap())
            });
            (points.next().unwrap(), points.next().unwrap())
        })
        .collect::<Vec<((i32, i32), (i32, i32))>>();

    let (max_x, max_y) = lines
        .iter()
        .fold((0, 0), |(max_x, max_y), ((x1, y1), (x2, y2))| {
            let max_x = if max_x < *x1 { *x1 + 1 } else { max_x };
            let max_y = if max_y < *y1 { *y1 + 1 } else { max_y };
            let max_x = if max_x < *x2 { *x2 + 1 } else { max_x };
            let max_y = if max_y < *y2 { *y2 + 1 } else { max_y };

            (max_x, max_y)
        });

    let mut field: Vec<usize> = Vec::new();

    field.resize((max_x * max_y) as usize, 0);
    lines
        .iter()
        .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2)
        .for_each(|((x1, y1), (x2, y2))| {
            let mut x = *x1;
            let mut y = *y1;

            let dx = direction(*x1, *x2);
            let dy = direction(*y1, *y2);

            field[(y * max_x + x) as usize] += 1;

            while x != *x2 || y != *y2 {
                x += dx;
                y += dy;
                field[(y * max_x + x) as usize] += 1;
            }
        });

    println!("answer1 {}", field.iter().filter(|v| **v >= 2).count());

    let mut field: Vec<usize> = Vec::new();

    field.resize((max_x * max_y) as usize, 0);
    lines.iter().for_each(|((x1, y1), (x2, y2))| {
        let mut x = *x1;
        let mut y = *y1;

        let dx = direction(*x1, *x2);
        let dy = direction(*y1, *y2);

        field[(y * max_x + x) as usize] += 1;

        while x != *x2 || y != *y2 {
            x += dx;
            y += dy;
            field[(y * max_x + x) as usize] += 1;
        }
    });

    println!("answer2 {}", field.iter().filter(|v| **v >= 2).count());

    Ok(())
}

fn direction(c1: i32, c2: i32) -> i32 {
    if c1 > c2 {
        -1
    } else if c1 == c2 {
        0
    } else {
        1
    }
}
