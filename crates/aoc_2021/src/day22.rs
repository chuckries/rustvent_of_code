use std::str::FromStr;

use aoc_common::{file_lines_as, Vec3i64, Selector, SelectorMut};

const DIMS: [(Selector<i64>, SelectorMut<i64>); 3] = [
    (Vec3i64::x, Vec3i64::x_mut),
    (Vec3i64::y, Vec3i64::y_mut),
    (Vec3i64::z, Vec3i64::z_mut),
];

struct Cube {
    points: (Vec3i64, Vec3i64),
    status: bool
}

impl Cube {
    fn new(status: bool, lo: Vec3i64, hi: Vec3i64) -> Cube {
        Cube {
            points: (lo, hi),
            status
        }
    }

    fn volume(&self) -> i64 {
        (self.points.1.x - self.points.0.x + 1) *
        (self.points.1.y - self.points.0.y + 1) *
        (self.points.1.z - self.points.0.z + 1)
    }

    fn add_to_space(self, space: &mut Vec<Cube>) {
        let existing: Vec<Cube> = space.drain(..).collect();
        let a = self.points;
        for mut cube in existing {
            let b = &mut cube.points;

            if a.0.x <= b.0.x && a.0.y <= b.0.y && a.0.z <= b.0.z && 
               a.0.x >= b.1.x && a.1.y >= b.0.y && a.1.z >= b.1.z {
                continue;
            }

            if a.1.x < b.0.x || a.0.x > b.1.x ||
               a.1.y < b.0.y || a.0.y > b.1.y ||
               a.1.z < b.0.z || a.0.z > b.1.z {
                space.push(cube);
                continue;
            }

            for dim in DIMS {
                check_dim(a, b, dim.0, dim.1, space, cube.status);
            }

            #[inline]
            fn check_dim(a: (Vec3i64, Vec3i64), b: &mut (Vec3i64, Vec3i64), selector: Selector<i64>, selector_mut: SelectorMut<i64>, space: &mut Vec<Cube>, status: bool) {
                let a_lo = selector(&a.0);
                let a_hi = selector(&a.1);
                let b_lo = selector(&b.0);
                let b_hi = selector(&b.1);

                if b_lo < a_lo {
                    let mut hi = b.1;
                    *selector_mut(&mut hi) = a_lo - 1;
                    space.push(Cube::new(status, b.0, hi).into());
                    *selector_mut(&mut b.0) = a_lo;
                }

                if b_hi > a_hi {
                    let mut lo = b.0;
                    *selector_mut(&mut lo) = a_hi + 1;
                    space.push(Cube::new(status, lo, b.1).into());
                    *selector_mut(&mut b.1) = a_hi;
                }
            }
        }

        space.push(self);
    }
}

impl FromStr for Cube {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tok: Vec<&str> = s.split(&[' ', '=', ',']).map(|s| s.split("..")).flatten().collect();

        let status = if tok[0] == "on" { true } else { false };

        let to_point = |a: &str, b: &str, c: &str| {
            (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap(), c.parse::<i64>().unwrap()).into()
        };

        let lo = to_point(tok[2], tok[5], tok[8]);
        let hi = to_point(tok[3], tok[6], tok[9]);

        Ok(Cube::new(status, lo, hi))
    }
}

fn input() -> Vec<Cube> {
    file_lines_as("inputs/day22.txt").collect()
}

#[test]
fn part1() {
    let input = input();

    let mut space: Vec<Cube> = Vec::new();
    for cube in input.into_iter().filter(|c| {
        c.points.0.x >= -50 && c.points.0.y >= -50 && c.points.0.z >= -50 &&
        c.points.1.x <=  50 && c.points.1.y <=  50 && c.points.1.z <=  50
    }) {
        cube.add_to_space(&mut space);
    }

    let answer = space.into_iter().filter_map(|c| {
        if c.status == true {
            Some(c.volume())
        } else {
            None
        }
    }).sum::<i64>();

    assert_eq!(answer, 588120);
}

#[test]
fn part2() {
    let input = input();

    let mut space: Vec<Cube> = Vec::new();
    for cube in input {
        cube.add_to_space(&mut space);
    }

    let answer = space.into_iter().filter_map(|c| {
        if c.status == true {
            Some(c.volume())
        } else {
            None
        }
    }).sum::<i64>();

    assert_eq!(answer, 1134088247046731);
}