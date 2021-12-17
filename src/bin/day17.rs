const TARGET_MIN_X: i64 = 288;
const TARGET_MAX_X: i64 = 330;
const TARGET_MIN_Y: i64 = -96;
const TARGET_MAX_Y: i64 = -50;

fn main() {
    let mut real_max_y = 0;
    let mut reached_count = 0;

    for dx in -TARGET_MAX_X * 2..TARGET_MAX_X * 2 {
        for dy in TARGET_MIN_Y * 2..TARGET_MAX_X * 2 {
            let (reached, max_y) = calculate(dx, dy);

            if !reached {
                continue;
            }
            reached_count += 1;

            if max_y > real_max_y {
                real_max_y = max_y;
            }
        }
    }

    println!("answer1 {}", real_max_y);
    println!("answer2 {}", reached_count);
}

fn calculate(mut dx: i64, mut dy: i64) -> (bool, i64) {
    let mut x = 0;
    let mut y = 0;
    let mut max_y = 0;

    loop {
        x += dx;
        y += dy;

        if dx < 0 {
            dx += 1
        } else if dx > 0 {
            dx -= 1
        }
        dy -= 1;

        if y > max_y {
            max_y = y
        }

        if x >= TARGET_MIN_X && x <= TARGET_MAX_X && y >= TARGET_MIN_Y && y <= TARGET_MAX_Y {
            return (true, max_y);
        }

        if dx > 0 && x > TARGET_MAX_X || dx < 0 && x < TARGET_MIN_X || y < TARGET_MIN_Y {
            return (false, 0);
        }
    }
}
