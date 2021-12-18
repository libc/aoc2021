use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/day18.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let output = contents.lines().map(|l| parse(l)).reduce(|a, b| a + b);

    println!("answer1 {}", output.unwrap().mag());

    let numbers = contents.lines().map(|l| parse(l)).collect::<Vec<S>>();
    let mut max_mag = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue;
            }

            let mag = (numbers[i].clone() + numbers[j].clone()).mag();
            if mag > max_mag {
                max_mag = mag
            }
        }
    }
    println!("answer2 {}", max_mag);

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum S {
    Regular(i32),
    Pair(Box<S>, Box<S>),
}

fn parse<ST: Into<String>>(s: ST) -> S {
    let str = s.into();
    let mut chars = str.chars();

    let mut stack = Vec::new();

    loop {
        let c = chars.next();
        if c.is_none() {
            break;
        }
        let c = c.unwrap();
        match c {
            '0'..='9' => {
                stack.push(S::Regular(c.to_string().parse::<i32>().unwrap()));
            }
            '[' => {}
            ',' => {}
            ']' => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(S::Pair(Box::new(b), Box::new(a)));
            }
            _ => {
                panic!("unknown")
            }
        }
    }
    stack.pop().unwrap()
}

fn pair(a: S, b: S) -> S {
    S::Pair(Box::new(a), Box::new(b))
}

impl S {
    fn reduce(self) -> S {
        let mut n = self;
        loop {
            let (new_s, d) = n.explode(0);
            n = new_s;
            if d.is_none() {
                let (new_s, done) = n.split();
                if !done {
                    return new_s;
                }
                n = new_s
            }
        }
    }

    fn is_pair(&self) -> bool {
        match self {
            S::Pair(_, _) => true,
            _ => false,
        }
    }

    fn number(&self) -> i32 {
        match self {
            S::Pair(_, _) => panic!("number called for a pair"),
            S::Regular(n) => *n,
        }
    }

    fn explode(&self, nesting: usize) -> (S, Option<(i32, i32)>) {
        match self {
            S::Pair(a, b) => {
                if nesting == 4 {
                    return (S::Regular(0), Some((a.number(), b.number())));
                }
                let (new_a, add_a) = a.explode(nesting + 1);
                if add_a.is_some() {
                    let left_add = add_a.unwrap().1;
                    let new_explode_add = Some((add_a.unwrap().0, 0));
                    if b.is_pair() {
                        return (pair(new_a, b.explode_add_left(left_add)), new_explode_add);
                    } else {
                        return (
                            pair(new_a, S::Regular(b.number() + left_add)),
                            new_explode_add,
                        );
                    }
                }
                let (new_b, add_b) = b.explode(nesting + 1);
                if add_b.is_some() {
                    let right_add = add_b.unwrap().0;
                    let new_explode_add = Some((0, add_b.unwrap().1));
                    if new_a.is_pair() {
                        return (
                            pair(new_a.explode_add_right(right_add), new_b),
                            new_explode_add,
                        );
                    } else {
                        return (
                            pair(S::Regular(new_a.number() + right_add), new_b),
                            new_explode_add,
                        );
                    }
                }

                (pair(new_a, new_b), None)
            }
            S::Regular(n) => (S::Regular(*n), None),
        }
    }

    fn explode_add_left(&self, add: i32) -> S {
        match self {
            S::Pair(l, other) => pair(l.explode_add_left(add), *(other.clone())),
            S::Regular(n) => S::Regular(n + add),
        }
    }

    fn explode_add_right(&self, add: i32) -> S {
        match self {
            S::Pair(other, r) => pair(*(other.clone()), r.explode_add_right(add)),
            S::Regular(n) => S::Regular(n + add),
        }
    }

    fn split(self) -> (S, bool) {
        match self {
            S::Pair(l, r) => {
                let (nl, done) = l.split();
                if done {
                    return (pair(nl, *r), true);
                }
                let (nr, done) = r.split();
                (pair(nl, nr), done)
            }
            S::Regular(n) => {
                if n > 9 {
                    if n % 2 == 0 {
                        (pair(S::Regular(n / 2), S::Regular(n / 2)), true)
                    } else {
                        (pair(S::Regular(n / 2), S::Regular(n / 2 + 1)), true)
                    }
                } else {
                    (S::Regular(n), false)
                }
            }
        }
    }

    fn mag(&self) -> i32 {
        match self {
            S::Pair(a, b) => 3 * a.mag() + 2 * b.mag(),
            S::Regular(n) => *n,
        }
    }
}

impl std::ops::Add for S {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        pair(self.clone(), other).reduce()
    }
}

mod test {
    use super::*;

    #[test]
    fn test_reduce() {
        assert_eq!(
            parse("[[[[[9,8],1],2],3],4]").reduce(),
            parse("[[[[0,9],2],3],4]")
        );
        assert_eq!(
            parse("[7,[6,[5,[4,[3,2]]]]]").reduce(),
            parse("[7,[6,[5,[7,0]]]]")
        );
        assert_eq!(
            parse("[[6,[5,[4,[3,2]]]],1]").reduce(),
            parse("[[6,[5,[7,0]]],3]")
        );
        assert_eq!(
            parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").reduce(),
            parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
        );
        assert_eq!(
            parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").reduce(),
            parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        );
    }

    #[test]
    fn test_add() {
        assert_eq!(
            vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]"]
                .into_iter()
                .map(|n| parse(n))
                .reduce(|a, b| a + b)
                .unwrap(),
            parse("[[[[1,1],[2,2]],[3,3]],[4,4]]")
        );
        assert_eq!(
            vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"]
                .into_iter()
                .map(|n| parse(n))
                .reduce(|a, b| a + b)
                .unwrap(),
            parse("[[[[3,0],[5,3]],[4,4]],[5,5]]")
        );
        assert_eq!(
            vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"]
                .into_iter()
                .map(|n| parse(n))
                .reduce(|a, b| a + b)
                .unwrap(),
            parse("[[[[5,0],[7,4]],[5,5]],[6,6]]")
        );

        assert_eq!(
            vec![
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
                "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
                "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
                "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
                "[7,[5,[[3,8],[1,4]]]]",
                "[[2,[2,2]],[8,[8,1]]]",
                "[2,9]",
                "[1,[[[9,3],9],[[9,0],[0,7]]]]",
                "[[[5,[7,4]],7],1]",
                "[[[[4,2],2],6],[8,7]]",
            ]
            .into_iter()
            .map(|n| parse(n))
            .reduce(|a, b| a + b)
            .unwrap(),
            parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
        );
    }

    #[test]
    fn test_mag() {
        assert_eq!(parse("[[1,2],[[3,4],5]]").mag(), 143);
        assert_eq!(parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").mag(), 1384);
        assert_eq!(parse("[[[[1,1],[2,2]],[3,3]],[4,4]]").mag(), 445);
        assert_eq!(parse("[[[[3,0],[5,3]],[4,4]],[5,5]]").mag(), 791);
    }
}
