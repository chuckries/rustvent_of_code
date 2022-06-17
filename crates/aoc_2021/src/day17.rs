use aoc_common::Vec2i32;

const TARGET: (Vec2i32, Vec2i32) = (Vec2i32::new(124, -123), Vec2i32::new(174, -86));

#[test]
fn part1() {
    let vy = -TARGET.0.y  - 1;
    let max_y = (vy * (vy + 1)) / 2;
    assert_eq!(max_y, 7503);
}

#[test]
fn part2() {
    let mut count = 0;

    // all xs and ys that will hit in one step:
    count += (TARGET.1.x - TARGET.0.x + 1) * (TARGET.1.y - TARGET.0.y + 1);

    let min_x = f64::ceil((f64::sqrt((1.0 + 8.0 * TARGET.0.x as f64) - 1.0)) / 2.0) as i32;
    let max_x = (TARGET.1.x  + 1) / 2;
    let min_y = (TARGET.0.y + 1) / 2;
    let max_y = -(TARGET.0.y + 1);

    fn is_hit(mut vx: i32, mut vy: i32) -> bool {
        let mut px = 0;
        let mut py = 0;

        loop {
            px += vx;
            py += vy;

            if px >= TARGET.0.x && px <= TARGET.1.x && py >= TARGET.0.y && py <= TARGET.1.y {
                return true;
            }

            if px > TARGET.1.x || py < TARGET.0.y {
                return false;
            }

            if vx > 0 {
                vx -= 1;
            }
            vy -= 1;
        }
    }

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if is_hit(x, y) {
                count += 1;
            }
        }
    }

    assert_eq!(count, 3229);
}