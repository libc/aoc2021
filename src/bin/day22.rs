use std::fs::File;
use std::io::prelude::*;
use std::time::{Duration, SystemTime};

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/day22.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut instructions = contents
        .lines()
        .map(|l| {
            let mut i = l.split(' ');
            let command = i.next().unwrap();
            let cuboids = i.next().unwrap();
            let mut cuboid = cuboids.split(',');

            let x = cuboid.next().unwrap();
            let y = cuboid.next().unwrap();
            let z = cuboid.next().unwrap();

            (
                command == "on",
                parse_coord(x),
                parse_coord(y),
                parse_coord(z),
            )
        })
        .collect::<Vec<(bool, (i32, i32), (i32, i32), (i32, i32))>>();

    instructions.reverse();

    let mut ons = 0;
    for x in -50..=50 {
        for y in -50..=50 {
            for z in -50..=50 {
                if value_from_instruction(&instructions, x, y, z) {
                    ons += 1;
                }
            }
        }
    }
    println!("answer1 {}", ons);

    let mut xs = Vec::new();
    let mut ys = Vec::new();
    let mut zs = Vec::new();

    xs.push(-50);
    xs.push(50);
    ys.push(-50);
    ys.push(50);
    zs.push(-50);
    zs.push(50);

    for (_, (start_x, end_x), (start_y, end_y), (start_z, end_z)) in instructions.iter() {
        xs.push(*start_x);
        xs.push(*end_x);

        ys.push(*start_y);
        ys.push(*end_y);

        zs.push(*start_z);
        zs.push(*end_z);
    }

    xs.sort();
    ys.sort();
    zs.sort();
    xs.dedup();
    ys.dedup();
    zs.dedup();

    let xs = expand(&xs);
    let ys = expand(&ys);
    let zs = expand(&zs);

    let all_regions = xs.iter().flat_map(|(start_x, end_x)| {
        ys.iter().flat_map(|(start_y, end_y)| {
            zs.iter().map(|(start_z, end_z)| {
                Region::new(
                    &Range::new(*start_x, *end_x),
                    &Range::new(*start_y, *end_y),
                    &Range::new(*start_z, *end_z),
                )
            })
        })
    });

    let instructions: Vec<(bool, Region)> = instructions
        .iter()
        .map(
            |(v, (start_x, end_x), (start_y, end_y), (start_z, end_z))| {
                (
                    *v,
                    Region::new(
                        &Range::new(*start_x, *end_x),
                        &Range::new(*start_y, *end_y),
                        &Range::new(*start_z, *end_z),
                    ),
                )
            },
        )
        .collect();

    let mut ones: u64 = 0;
    let mut ones_50x50x50: u64 = 0;
    let r_50x50x50 = Region::new(
        &Range::new(-50, 50),
        &Range::new(-50, 50),
        &Range::new(-50, 50),
    );

    let mut next_print = SystemTime::now() + Duration::new(1, 0);
    let total = xs.len() * ys.len() * zs.len();
    for (idx, r) in all_regions.enumerate() {
        if idx % 10000 == 0 && next_print < SystemTime::now() {
            println!(
                "i: {} / {} ({}%) ones: {}",
                idx,
                total,
                (idx * 100) / total,
                ones
            );
            next_print = SystemTime::now() + Duration::new(1, 0);
        }
        for (v, or) in instructions.iter() {
            if r.is_inside(&or) {
                if *v {
                    ones += r.x.cubes() * r.y.cubes() * r.z.cubes();

                    if r.is_inside(&r_50x50x50) {
                        ones_50x50x50 += r.x.cubes() * r.y.cubes() * r.z.cubes();
                    }
                }
                break;
            }
        }
    }

    println!("answer1 {}", ones_50x50x50);
    println!("answer2 {}", ones);

    Ok(())
}

fn parse_coord(x: &str) -> (i32, i32) {
    let mut i = x[2..].split("..").map(|p| p.parse::<i32>().unwrap());
    (i.next().unwrap(), i.next().unwrap())
}

fn value_from_instruction(
    instructions: &Vec<(bool, (i32, i32), (i32, i32), (i32, i32))>,
    x: i32,
    y: i32,
    z: i32,
) -> bool {
    for (v, (xs, xe), (ys, ye), (zs, ze)) in instructions {
        if *xs <= x && x <= *xe && *ys <= y && y <= *ye && *zs <= z && z <= *ze {
            return *v;
        }
    }

    return false;
}

#[derive(Clone, Debug)]
struct Range {
    start: i32,
    end: i32,
}

#[derive(Clone, Debug)]
struct Region {
    x: Range,
    y: Range,
    z: Range,
}

impl Range {
    fn new(start: i32, end: i32) -> Self {
        Self { start, end }
    }

    fn is_inside(&self, other: &Range) -> bool {
        other.start <= self.start && self.end <= other.end
    }

    fn cubes(&self) -> u64 {
        (self.end - self.start + 1) as u64
    }
}

impl Region {
    fn new(x: &Range, y: &Range, z: &Range) -> Self {
        Self {
            x: x.clone(),
            y: y.clone(),
            z: z.clone(),
        }
    }

    fn is_inside(&self, other: &Region) -> bool {
        self.x.is_inside(&other.x) && self.y.is_inside(&other.y) && self.z.is_inside(&other.z)
    }
}

fn expand(a: &Vec<i32>) -> Vec<(i32, i32)> {
    let mut i = a.iter();
    let mut output: Vec<(i32, i32)> = Vec::new();

    let mut a = i.next().unwrap();

    output.push((*a, *a));
    loop {
        let b = i.next();
        if b.is_none() {
            break;
        }
        let b = b.unwrap();
        if *b != a + 1 {
            output.push((a + 1, b - 1));
        }
        output.push((*b, *b));
        a = b
    }

    output
}
