use aoc_common::{Vec3i64, file_lines, IteratorExt, Vec2i64};

fn input() -> Vec<(Vec3i64, Vec3i64)> {
    file_lines("inputs/day24.txt").map(|l| {
        let mut split = l.split(" @ ").map(|p| {
            let mut split = p.split(", ").map(|s| s.parse::<i64>().unwrap());
            Vec3i64::new(split.next().unwrap(), split.next().unwrap(), split.next().unwrap())
        });

        (split.next().unwrap(), split.next().unwrap())
    }).to_vec()
}

#[test]
fn part1() {
    let input = input();

    // p(t) = p0 + v0 * t
    // (y - p.y) = m(x - p.x)
    // m = v.y / v.x
    // y = (v.y / v.x)(x - p.x) + p.y
    // (v0.y / v0.x)(x - p0.x) + p0.y = (v1.y / v1.x)(x - p1.x) + p1.y
    // m0x - m0p0.x + p0.y = m1x - m1p1.x + p1.y
    // (m0 - m1)x = m0p0.x - m1p1.x + p1.y - p0.y
    // x = (m0p0.x - m1p1.x + p1.y - p0.y) / (m0 - m1)

    const MIN: f64 = 200000000000000.0;
    const MAX: f64 = 400000000000000.0;

    let mut total = 0;
    for i in 0 .. input.len() - 1 {
        for j in i + 1 .. input.len() {
            let (p0, v0) = input[i];
            let (p1, v1) = input[j];

            let m0 = v0.y as f64 / v0.x as f64;
            let m1 = v1.y as f64 / v1.x as f64;

            if m0 == m1 {
                continue;
            }

            let x = (m0 * p0.x as f64 - m1 * p1.x as f64 + p1.y as f64 - p0.y as f64) / (m0 - m1);
            let y = m0 * (x - p0.x as f64) + p0.y as f64;

            let sign0 = Vec2i64::new(-v0.x.signum(), -v0.y.signum());
            if sign0 == Vec2i64::new(f64::signum(x - p0.x as f64) as i64, f64::signum(y - p0.y as f64) as i64) {
                continue;
            }

            let sign1 = Vec2i64::new(-v1.x.signum(), -v1.y.signum());
            if sign1 == Vec2i64::new(f64::signum(x - p1.x as f64) as i64, f64::signum(y - p1.y as f64) as i64) {
                continue;
            }

            if x >= MIN && x <= MAX && y >= MIN && y <= MAX {
                total += 1;
            }
        }
    }

    assert_eq!(16779, total);
}

#[test]
fn part2() {
    // p(t) = p0 + v0 * t

    // p + v * t0 = p0 + v0 * t0
    // p + v * t1 = p1 + v1 * t1
    // p + v * t2 = p2 + v2 * t2

    // p - p0 = t0 * (v0 - v)
    // p - p1 = t1 * (v1 - v)
    // p - p2 = t2 * (v2 - v)

    // p = t0 * (v0 - v) + p0
    // p = t1 * (v1 - v) + p1
    // p = t2 * (v2 - v) + p2 

}