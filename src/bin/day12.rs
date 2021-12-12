use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/day12.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let graph: BTreeMap<Node, Vec<Node>> = contents
        .lines()
        .map(|l| {
            let mut i = l.split("-");

            let start = i.next().unwrap();
            let end = i.next().unwrap();

            (Node::parse(start), Node::parse(end))
        })
        .fold(BTreeMap::new(), |mut map, (start, end)| {
            let start_clone = start.clone();
            let v = map.entry(start_clone).or_insert(Vec::new());
            v.push(end.clone());
            let v = map.entry(end.clone()).or_insert(Vec::new());
            v.push(start.clone());

            map
        });

    let mut found = Vec::new();

    let mut queue = vec![vec![Node::Start]];
    while queue.len() > 0 {
        let path = queue.pop().unwrap();

        let last = &path[path.len() - 1];

        for next in graph[&last].iter() {
            match next {
                Node::Start => continue,
                Node::End => found.push(path.clone()),
                n @ Node::Small(_) => {
                    if path.iter().any(|nn| nn == n) {
                        continue;
                    } else {
                        let mut np = path.clone();
                        np.push(n.clone());
                        queue.push(np);
                    }
                }
                n @ Node::Big(_) => {
                    let mut np = path.clone();
                    np.push(n.clone());
                    queue.push(np);
                }
            }
        }
    }

    println!("answer1 {}", found.len());

    let mut found = Vec::new();

    let mut queue = vec![vec![Node::Start]];
    while queue.len() > 0 {
        let path = queue.pop().unwrap();

        let last = &path[path.len() - 1];

        for next in graph[&last].iter() {
            match next {
                Node::Start => continue,
                Node::End => found.push(path.clone()),
                n @ Node::Small(_) => {
                    if contains_double(&path) {
                        if path.iter().any(|nn| nn == n) {
                            continue;
                        } else {
                            let mut np = path.clone();
                            np.push(n.clone());
                            queue.push(np);
                        }
                    } else {
                        if path.iter().filter(|nn| nn == &n).count() > 1 {
                            continue;
                        } else {
                            let mut np = path.clone();
                            np.push(n.clone());
                            queue.push(np);
                        }
                    }
                }

                n @ Node::Big(_) => {
                    let mut np = path.clone();
                    np.push(n.clone());
                    queue.push(np);
                }
            }
        }
    }
    println!("answer2 {}", found.len());

    Ok(())
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Node {
    Start,
    End,
    Big(String),
    Small(String),
}

impl Node {
    fn parse<S: Into<String>>(s: S) -> Node {
        let s = s.into();
        if s == "start" {
            Node::Start
        } else if s == "end" {
            Node::End
        } else if s.chars().all(|c| c.is_uppercase()) {
            Node::Big(s.clone())
        } else {
            Node::Small(s.clone())
        }
    }
}

fn contains_double(v: &Vec<Node>) -> bool {
    let mut set: BTreeSet<Node> = BTreeSet::new();

    for e in v {
        match e {
            n @ Node::Small(_) => {
                if set.contains(&n) {
                    return true;
                } else {
                    set.insert(n.clone());
                }
            }
            _ => continue,
        }
    }
    false
}
