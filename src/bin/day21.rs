use std::collections::BTreeMap;

fn main() {
    let mut players = vec![6, 2];
    let mut scores = vec![0, 0];

    let mut dice = 0;
    let mut current_player = 0;
    let mut dice_rolls = 0;

    loop {
        dice_rolls += 3;

        let current_roll = (dice + 1) + (dice + 2) + (dice + 3);
        dice = (dice + 3) % 100;
        players[current_player] = (players[current_player] + current_roll) % 10;
        scores[current_player] += players[current_player] + 1;
        println!(
            "player {}, position {}, score {}",
            current_player + 1,
            players[current_player] + 1,
            scores[current_player]
        );

        if scores[current_player] >= 1000 {
            println!("answer1 {}", scores[(current_player + 1) % 2] * dice_rolls);
            break;
        }

        current_player = (current_player + 1) % 2;
    }

    let (a, b) = simulate(3, 7);
    println!("answer2 (test) {} {}", a, b);

    let (a, b) = simulate(6, 2);
    println!("answer2 {} {}", a, b);
}

fn simulate(player1: i8, player2: i8) -> (usize, usize) {
    let mut grid: BTreeMap<(i8, i8), BTreeMap<(usize, usize), usize>> = BTreeMap::new();

    let a = grid.entry((player1, player2)).or_insert(BTreeMap::new());
    a.insert((0, 0), 1);

    let mut wins1 = 0;
    let mut wins2 = 0;
    let mut turn = 0;

    while grid.len() > 0 {
        let mut new_grid = BTreeMap::new();

        for ((player1, player2), games) in grid {
            for dice in vec![
                1 + 1 + 1,
                1 + 1 + 2,
                1 + 1 + 3,
                1 + 2 + 1,
                1 + 2 + 2,
                1 + 2 + 3,
                1 + 3 + 1,
                1 + 3 + 2,
                1 + 3 + 3,
                2 + 1 + 1,
                2 + 1 + 2,
                2 + 1 + 3,
                2 + 2 + 1,
                2 + 2 + 2,
                2 + 2 + 3,
                2 + 3 + 1,
                2 + 3 + 2,
                2 + 3 + 3,
                3 + 1 + 1,
                3 + 1 + 2,
                3 + 1 + 3,
                3 + 2 + 1,
                3 + 2 + 2,
                3 + 2 + 3,
                3 + 3 + 1,
                3 + 3 + 2,
                3 + 3 + 3,
            ]
            .iter()
            {
                let new_player1 = if turn % 2 == 0 {
                    (player1 + dice) % 10
                } else {
                    player1
                };
                let new_player2 = if turn % 2 == 1 {
                    (player2 + dice) % 10
                } else {
                    player2
                };

                for ((score1, score2), states) in games.iter() {
                    let score1: usize = if turn % 2 == 0 {
                        score1 + (new_player1 as usize) + 1
                    } else {
                        *score1
                    };
                    let score2: usize = if turn % 2 == 1 {
                        score2 + new_player2 as usize + 1
                    } else {
                        *score2
                    };

                    if score1 >= 21 {
                        wins1 += states;
                    } else if score2 >= 21 {
                        wins2 += states;
                    } else {
                        let new_state = new_grid
                            .entry((new_player1, new_player2))
                            .or_insert(BTreeMap::new());

                        *new_state.entry((score1, score2)).or_insert(0) += states;
                    }
                }
            }
        }

        grid = new_grid;

        turn += 1;
        println!("{:?}", grid);
    }

    (wins1, wins2)
}
