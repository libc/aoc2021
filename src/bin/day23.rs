use std::collections::BTreeMap;
use std::collections::BinaryHeap;
use std::time::{Duration, SystemTime};

fn main() {
    let mut heap = BinaryHeap::new();
    let mut scheduled = BTreeMap::new();

    heap.push(Game::start());

    let mut next_print = SystemTime::now() + Duration::new(1, 0);
    let mut iteration = 0;
    let mut best_heuristic = Game::start().heuristic;
    while !heap.is_empty() {
        let g = heap.pop().unwrap();

        if g.heuristic < best_heuristic {
            best_heuristic = g.heuristic
        }

        iteration += 1;
        if iteration % 10000 == 0 && next_print < SystemTime::now() {
            println!("best_heuristic {}; score: {}", best_heuristic, g.score);
            next_print = SystemTime::now() + Duration::new(1, 0);
        }

        if g.is_final() {
            println!("answer1 {}", g.score);
            break;
        }

        for (p, _) in g.grid.iter() {
            // println!("{:?} can go to {:?}", p, g.posible_moves(p));
            for op in g.posible_moves(p) {
                let ng = g.make_a_move(p, &op);

                let os = scheduled.entry(ng.signature()).or_insert(0);

                if *os == 0 || *os > ng.score {
                    *os = ng.score;
                    heap.push(ng);
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Amphipod {
    Amber(i8),
    Bronze(i8),
    Copper(i8),
    Desert(i8),
}

impl Amphipod {
    fn destination(&self) -> i8 {
        match self {
            Amphipod::Amber(_) => 2,
            Amphipod::Bronze(_) => 4,
            Amphipod::Copper(_) => 6,
            Amphipod::Desert(_) => 8,
        }
    }

    fn multiplier(&self) -> usize {
        match self {
            Amphipod::Amber(_) => 1,
            Amphipod::Bronze(_) => 10,
            Amphipod::Copper(_) => 100,
            Amphipod::Desert(_) => 1000,
        }
    }

    fn type_num(&self) -> i8 {
        match self {
            Amphipod::Amber(_) => 1,
            Amphipod::Bronze(_) => 2,
            Amphipod::Copper(_) => 3,
            Amphipod::Desert(_) => 4,
        }
    }
}

#[derive(PartialEq, Eq, Ord, Clone, Debug)]
struct Game {
    grid: BTreeMap<(i8, i8), Amphipod>,
    score: usize,
    heuristic: usize,
}

impl Game {
    fn start() -> Self {
        //   0123456789(10)
        //  #############
        //0 #...........#
        //1 ###D#B#A#C###
        //2   #B#D#A#C#
        //    #########
        //
        //
        // 47104 is too low
        // 47364 is too high

        let grid = vec![
            ((6, 1), Amphipod::Amber(0)),
            ((6, 2), Amphipod::Amber(1)),
            ((4, 1), Amphipod::Bronze(0)),
            ((2, 2), Amphipod::Bronze(1)),
            ((8, 1), Amphipod::Copper(0)),
            ((8, 2), Amphipod::Copper(1)),
            ((2, 1), Amphipod::Desert(0)),
            ((4, 2), Amphipod::Desert(1)),
            //
            // ((2, 4), Amphipod::Amber(0)),
            // ((8, 4), Amphipod::Amber(1)),
            // ((2, 1), Amphipod::Bronze(0)),
            // ((6, 1), Amphipod::Bronze(1)),
            // ((4, 1), Amphipod::Copper(0)),
            // ((6, 4), Amphipod::Copper(1)),
            // ((8, 1), Amphipod::Desert(0)),
            // ((4, 4), Amphipod::Desert(1)),
        ]
        .into_iter()
        .collect();

        let mut game = Self {
            grid,
            score: 0,
            heuristic: 0,
        };
        game.calculate_heuristic();
        game
    }

    fn signature(&self) -> (u64, u64) {
        let mut a = 0;
        let mut b = 0;

        for ((x, y), obj) in self.grid.iter() {
            let type_num = obj.type_num() as u64;

            if *y == 0 {
                a = a | (type_num << (x * 3));
            } else {
                let offset = ((x - 2) / 2) * 4 + (y - 1);
                b = b | (type_num << (offset * 3));
            }
        }

        (a, b)
    }

    fn calculate_heuristic(&mut self) {
        self.heuristic = self
            .grid
            .iter()
            .map(|((x, _), obj)| ((x - obj.destination()).abs() as usize) * obj.multiplier())
            .sum()
    }

    fn is_final(&self) -> bool {
        self.grid
            .iter()
            .all(|((x, _), obj)| *x == obj.destination())
    }

    fn posible_moves(&self, m: &(i8, i8)) -> Vec<(i8, i8)> {
        let obj = self.grid.get(m);

        if obj.is_none() {
            return vec![];
        }

        let destination_x = obj.unwrap().destination();

        let mut destinations = vec![(0, 0), (1, 0), (3, 0), (5, 0), (7, 0), (9, 0), (10, 0)];

        if self.destination_lane_possible(destination_x) {
            destinations.push((destination_x, 1));
            destinations.push((destination_x, 2));
            destinations.push((destination_x, 3));
            destinations.push((destination_x, 4));
        }

        destinations
            .into_iter()
            .filter(|(x, y)| self.grid.get(&(*x, *y)).is_none())
            .filter(|(x, y)| self.can_go(m, &(*x, *y)))
            .collect()
    }

    fn destination_lane_possible(&self, destination: i8) -> bool {
        let o_1 = self.grid.get(&(destination, 1));
        let o_2 = self.grid.get(&(destination, 2));
        let o_3 = self.grid.get(&(destination, 3));
        let o_4 = self.grid.get(&(destination, 4));

        (o_1.is_none() || o_1.unwrap().destination() == destination)
            && (o_2.is_none() || o_2.unwrap().destination() == destination)
            && (o_3.is_none() || o_3.unwrap().destination() == destination)
            && (o_4.is_none() || o_4.unwrap().destination() == destination)
    }

    fn can_go(&self, from: &(i8, i8), to: &(i8, i8)) -> bool {
        let (fx, fy) = from;
        let (tx, ty) = to;

        if fx == tx && fy == ty {
            return false;
        }

        if tx == fx {
            let min = if fy < ty { fy + 1 } else { *ty };
            let max = if fy > ty { fy - 1 } else { *ty };

            (min..=max).all(|y| !self.grid.contains_key(&(*tx, y)))
        } else if *fy == 0 && *ty == 0 {
            let min = if fx < tx { fx + 1 } else { *tx };
            let max = if fx > tx { fx - 1 } else { *tx };

            (min..=max).all(|x| !self.grid.contains_key(&(x, 0)))
        } else {
            let mut can = true;

            if *fy != 0 {
                can = can && self.can_go(&(*fx, *fy), &(*fx, 0));
            }
            can = can && self.can_go(&(*fx, 0), &(*tx, 0));
            if *ty != 0 {
                can = can && self.can_go(&(*tx, 0), &(*tx, *ty));
            }

            can
        }
    }

    fn make_a_move(&self, from: &(i8, i8), to: &(i8, i8)) -> Self {
        let mut new_grid = self.grid.clone();
        let obj = new_grid.remove(from).unwrap();
        new_grid.insert(to.clone(), obj);

        let mut new_game = Self {
            grid: new_grid,
            score: self.score + distance(from, to) * obj.multiplier(),
            heuristic: 0,
        };

        new_game.calculate_heuristic();

        new_game
    }
}

fn distance(from: &(i8, i8), to: &(i8, i8)) -> usize {
    ((to.0 - from.0).abs() as usize) + (to.1 as usize) + (from.1 as usize)
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.score
                .cmp(&other.score)
                .then(self.heuristic.cmp(&other.heuristic))
                .reverse(),
        )
        // Some(self.heuristic.cmp(&other.heuristic).then(self.score.cmp(&other.score)).reverse())
    }
}

mod test {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(distance(&(6, 1), &(3, 0)), 4);
        assert_eq!(distance(&(2, 2), &(8, 2)), 10);
    }

    #[test]
    fn signature() {
        assert_eq!(Game::start().signature(), (0, 119513127953700));
        let mut g = Game::start();
        g = g.make_a_move(&(2, 1), &(1, 0));
        assert_eq!(g.signature(), (32, 119513127953696));
    }
}
