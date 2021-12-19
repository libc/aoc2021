use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/day19.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let beacons: Vec<Vec<Beacon>> = contents
        .split("\n\n")
        .map(|report| {
            let mut lines = report.lines();
            let scanner_id = lines
                .next()
                .unwrap()
                .split(' ')
                .skip(2)
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            lines
                .map(|l| {
                    let mut coords = l.split(",");
                    let x = coords.next().unwrap().parse::<i32>().unwrap();
                    let y = coords.next().unwrap().parse::<i32>().unwrap();
                    let z = coords.next().unwrap().parse::<i32>().unwrap();

                    Beacon {
                        scanner_id,
                        point: Point { x, y, z },
                    }
                })
                .collect()
        })
        .collect();

    let mut adjusted_points: BTreeSet<Point> = BTreeSet::new();
    let mut fixed_scanners = BTreeMap::new();
    fixed_scanners.insert(
        0,
        Scanner {
            id: 0,
            point: Point { x: 0, y: 0, z: 0 },
            orientation: 0 + 1 * 6 + 2 * 36,
        },
    );

    for p in beacons[0].iter() {
        adjusted_points.insert(p.point.clone());
    }

    loop {
        let mut found = false;
        for scanners in beacons.iter() {
            for b in scanners.iter() {
                if fixed_scanners.contains_key(&b.scanner_id) {
                    break;
                }
                let mut found_scanner: Option<Scanner> = None;
                for orient in 0..217 {
                    let fixed = b.point.orient(orient);

                    for p in adjusted_points.iter() {
                        // p1.x = scanner2.x+p2.x -> scanner2.x = scanner1.x+p1.x - p2.x
                        let scanner2 = Scanner {
                            id: b.scanner_id,
                            point: *p - fixed,
                            orientation: orient,
                        };

                        let known_overlapping = adjusted_points
                            .iter()
                            .filter(|p| scanner2.is_visible(p))
                            .cloned()
                            .collect::<BTreeSet<Point>>();
                        let scanner2_points = beacons[b.scanner_id]
                            .iter()
                            .map(|p| scanner2.adjust(&p.point))
                            .collect::<Vec<_>>();

                        let new_overlapping: BTreeSet<Point> = fixed_scanners
                            .iter()
                            .flat_map(|(_, scanner)| {
                                scanner2_points
                                    .iter()
                                    .filter_map(|p| {
                                        if scanner.is_visible(p) {
                                            Some(*p)
                                        } else {
                                            None
                                        }
                                    })
                                    .collect::<BTreeSet<Point>>()
                            })
                            .collect::<BTreeSet<Point>>();

                        if known_overlapping == new_overlapping && new_overlapping.len() >= 12 {
                            println!("found {}", new_overlapping.len());
                            found_scanner = Some(scanner2);
                            break;
                        }
                    }

                    if found_scanner.is_some() {
                        break;
                    }
                }

                if found_scanner.is_some() {
                    found = true;
                    let scanner2 = found_scanner.unwrap();

                    fixed_scanners.insert(scanner2.id, scanner2.clone());

                    for beacon in beacons[scanner2.id].iter() {
                        adjusted_points.insert(scanner2.adjust(&beacon.point));
                    }
                }
            }
        }

        if found {
            println!(
                "solved {} out of {} scanners",
                fixed_scanners.len(),
                beacons.len()
            );
            println!("one more time (beacons so far {})", adjusted_points.len());
        } else {
            break;
        }
    }

    println!(
        "solved {} out of {} scanners",
        fixed_scanners.len(),
        beacons.len()
    );
    println!("answer1 {}", adjusted_points.len());

    let mut max = 0;
    for (id1, scanner1) in fixed_scanners.iter() {
        for (id2, scanner2) in fixed_scanners.iter() {
            if id1 == id2 {
                continue;
            }

            let d = scanner1.point - scanner2.point;
            let d = d.x.abs() + d.y.abs() + d.z.abs();
            if d > max {
                max = d
            }
        }
    }
    println!("answer2 {}", max);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
struct Beacon {
    scanner_id: usize,
    point: Point,
}

impl Point {
    fn orient(&self, p: usize) -> Point {
        let idx_x = p % 6;
        let idx_y = (p / 6) % 6;
        let idx_z = (p / 36) % 6;

        Point {
            x: self.coord_from_index(idx_x),
            y: self.coord_from_index(idx_y),
            z: self.coord_from_index(idx_z),
        }
    }

    fn coord_from_index(&self, idx: usize) -> i32 {
        match idx {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            3 => -self.x,
            4 => -self.y,
            5 => -self.z,
            _ => unimplemented!(),
        }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[derive(Clone)]
struct Scanner {
    id: usize,
    point: Point,
    orientation: usize,
}

impl Scanner {
    fn is_visible(&self, p: &Point) -> bool {
        (p.x - self.point.x).abs() <= 1000
            && (p.y - self.point.y).abs() <= 1000
            && (p.z - self.point.z).abs() <= 1000
    }

    fn adjust(&self, p: &Point) -> Point {
        self.point + p.orient(self.orientation)
    }
}
