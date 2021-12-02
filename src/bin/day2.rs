use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/day2.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let (x, y) = contents
        .lines()
        .map(|l| {
            if l.starts_with("forward ") {
                let dx = l["forward ".len()..].parse::<i32>().unwrap();
                (dx, 0)
            } else if l.starts_with("down ") {
                let dy = l["down ".len()..].parse::<i32>().unwrap();
                (0, dy)
            } else if l.starts_with("up ") {
                let dy = l["up ".len()..].parse::<i32>().unwrap();
                (0, -dy)
            } else {
                unimplemented!("unreachable")
            }
        })
        .reduce(|(ox, oy), (dx, dy)| (ox + dx, oy + dy))
        .unwrap();

    println!("answer 1: {}", x * y);

    let (x, y, _aim) = contents.lines().fold((0, 0, 0), |(x, y, aim), l| {
        if l.starts_with("forward ") {
            let dx = l["forward ".len()..].parse::<i32>().unwrap();
            (x + dx, y + aim * dx, aim)
        } else if l.starts_with("down ") {
            let new_aim = l["down ".len()..].parse::<i32>().unwrap();
            (x, y, aim + new_aim)
        } else if l.starts_with("up ") {
            let new_aim = l["up ".len()..].parse::<i32>().unwrap();
            (x, y, aim - new_aim)
        } else {
            unimplemented!("unreachable")
        }
    });

    println!("answer 2: {}", x * y);

    Ok(())
}
