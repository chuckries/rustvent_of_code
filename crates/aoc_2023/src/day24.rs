use aoc_common::{Vec3i64, file_lines, IteratorExt, Vec2i64, VecN};

fn input() -> Vec<(Vec3i64, Vec3i64)> {
    file_lines("inputs/day24.txt").map(|l| {
        let mut split = l.split(" @ ").map(|p| {
            let mut split = p.split(",").map(|s| s.trim().parse::<i64>().unwrap());
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

            let sign0 = -v0.xy().signum();
            if sign0 == Vec2i64::new(f64::signum(x - p0.x as f64) as i64, f64::signum(y - p0.y as f64) as i64) {
                continue;
            }

            let sign1 = -v1.xy().signum();
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
    // I really had no clue how to approach this one and this became an exercise in learning some linear algebra
    // Answer essentially copied from this reddit post: https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/kepu26z/?utm_source=share&utm_medium=web2x&context=3

    // p(t) = p0 + v0 * t
    // p + v * t0 = p0 + v0 * t0
    // p - p0 = -t0 (v - v0)
    // What is seen here is that p - p0 and v - v0 are the same vector just scaled by a constant, therefore there cross product is zero 
    // (p - p0) X (v - v0) = 0  
    // p X v - p0 x v - p X v0 + p0 X v0 = 0
    // p X v = p0 x v + p X v0 - p0 X v0
    // p X v term will be shared by all i, so we can equate two pairs of inputs to develop a system
    // p0 X v + p X v0 - p0 X v0 = p1 X v + p X v1 - p1 X pv1
    // (p0 - p1) X v + p X (v0 - v1) = p0 X v0 - p1 X v1
    // v X (p1 - p0) + p X (v0 - v1) = p0 X v0 - p1 X v1
    // p X (v0 - v1) + v X (p1 - p0) = p0 X v0 - p1 X v1

    // evaluating the cross product for i, j, and k components for 2 different pairs of inputs
    // gives us a series of 6 equations with 6 unknowns that can be expressed as the following augmented matrix

    let input = input();
    let (p0, v0) = input[0];
    let (p1, v1) = input[1];
    let (p2, v2) = input[2];

    let mut matrix: [VecN<7, f64>; 6] = Default::default();

    let bv = v0 - v1;
    let bp = p1 - p0;
    let cross = p0.cross(v0) - p1.cross(v1);

    matrix[0][1] = bv.z as f64;
    matrix[0][2] = -bv.y as f64;
    matrix[0][4] = bp.z as f64;
    matrix[0][5] = -bp.y as f64;
    matrix[0][6] = cross.x as f64;

    matrix[1][0] = -bv.z as f64;
    matrix[1][2] = bv.x as f64;
    matrix[1][3] = -bp.z as f64;
    matrix[1][5] = bp.x as f64;
    matrix[1][6] = cross.y as f64;

    matrix[2][0] = bv.y as f64;
    matrix[2][1] = -bv.x as f64;
    matrix[2][3] = bp.y as f64;
    matrix[2][4] = -bp.x as f64;
    matrix[2][6] = cross.z as f64;

    let bv = v0 - v2;
    let bp = p2 - p0;
    let cross = p0.cross(v0) - p2.cross(v2);

    matrix[3][1] = bv.z as f64;
    matrix[3][2] = -bv.y as f64;
    matrix[3][4] = bp.z as f64;
    matrix[3][5] = -bp.y as f64;
    matrix[3][6] = cross.x as f64;

    matrix[4][0] = -bv.z as f64;
    matrix[4][2] = bv.x as f64;
    matrix[4][3] = -bp.z as f64;
    matrix[4][5] = bp.x as f64;
    matrix[4][6] = cross.y as f64;

    matrix[5][0] = bv.y as f64;
    matrix[5][1] = -bv.x as f64;
    matrix[5][3] = bp.y as f64;
    matrix[5][4] = -bp.x as f64;
    matrix[5][6] = cross.z as f64;

    // this is a slapdash gaussian elimination for a 6x6 augmented matrix
    // floating point error certainly has the potential to be a problem here but 
    // doesn't get in the way for my input

    for row in 0..5 {
        if matrix[row][row] == 0.0 {
            for i in row + 1 .. matrix.len() {
                if matrix[i][row] != 0.0 {
                    matrix.swap(row, i);
                    break;
                }
            }
        }
        if matrix[row][row] != 0.0 {
            for i in row + 1 .. matrix.len() {
                if matrix[i][row] != 0.0 {
                    let factor = matrix[i][row] / matrix[row][row];
                    let to_add = matrix[row] * -factor;
                    matrix[i] = matrix[i] + to_add;
                }
            }
        }
    }

    let mut answers: [i64; 6] = [0; 6];
    for row in (0..6).rev() {
        let mut total = matrix[row][6];
        for i in row + 1..6 {
            total -= answers[i] as f64 * matrix[row][i];
        }
        total /= matrix[row][row];
        answers[row] = total.round() as i64;
    }

    assert_eq!(871983857253169, answers.iter().take(3).sum::<i64>());

}