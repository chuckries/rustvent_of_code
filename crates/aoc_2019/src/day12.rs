use aoc_common::{Vec3i32, file_string, lcm, IteratorExt, Selector, SelectorMut};
use lazy_static::lazy_static;
use regex::Regex;

const DIMS: [(Selector<i32>, SelectorMut<i32>); 3] = [
    (Vec3i32::x, Vec3i32::x_mut),
    (Vec3i32::y, Vec3i32::y_mut),
    (Vec3i32::z, Vec3i32::z_mut),
];

fn input() -> Vec<Vec3i32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
    }

    RE.captures_iter(&file_string("inputs/day12.txt")).map(|capture| {
        let mut num_iter = capture.iter().skip(1).map(|c| {
            c.unwrap().as_str().parse::<i32>().unwrap()
        });

        Vec3i32::new(num_iter.next().unwrap(), num_iter.next().unwrap(), num_iter.next().unwrap())
    }).collect()
}

fn run(iterations: usize, mut stopper: impl FnMut(&[Vec3i32], &[Vec3i32]) -> bool) -> (Vec<Vec3i32>, Vec<Vec3i32>) {
    let mut moons = input();
    let mut vels = vec![Vec3i32::zero(); moons.len()];

    #[inline]
    fn update_gravity(idx_a: usize, idx_b: usize, moons: &[Vec3i32], vels: &mut [Vec3i32], moon_selector: Selector<i32>, vel_selector: SelectorMut<i32>) {
        let a = moon_selector(&moons[idx_a]);
        let b = moon_selector(&moons[idx_b]);

        if a < b {
            *vel_selector(&mut vels[idx_a]) += 1;
            *vel_selector(&mut vels[idx_b]) -= 1;
        } else if a > b {
            *vel_selector(&mut vels[idx_a]) -= 1;
            *vel_selector(&mut vels[idx_b]) += 1;
        }
    }

    for _ in 0..iterations {
        for i in 0..moons.len() - 1 {
            for j in i..moons.len() {
                for dim in DIMS {
                    update_gravity(i, j, &moons, &mut vels, dim.0, dim.1);
                }
            }
        }

        moons.iter_mut().zip(vels.iter()).for_each(|(m, v)| { *m += *v });

        if stopper(&moons, &vels) {
            break;
        }
    }

    (moons, vels)
}

#[test]
fn part1() {
    fn stopper(_: &[Vec3i32], _: &[Vec3i32]) -> bool { false }
    let (moons, vels) = run(1000, stopper);

    let answer = moons.into_iter().zip(vels).map(|(m, v)| {
        (i32::abs(m.x) + i32::abs(m.y) + i32::abs(m.z)) * (i32::abs(v.x) + i32::abs(v.y) + i32::abs(v.z))
    }).sum::<i32>();

    assert_eq!(answer, 7687);
}

struct Dim {
    selector: fn(&Vec3i32) -> i32,
    initial: Vec<(i32, i32)>,
    cycle: usize,
    found: bool
}

impl Dim {
    fn new(selector: fn(&Vec3i32) -> i32, input: &[Vec3i32]) -> Dim {
        let initial = input.iter().map(|p| (selector(p), 0)).collect();

        Dim {
            selector,
            initial,
            cycle: 0,
            found: false
        }
    }
}

#[test]
fn part2() {
    let input = input();

    let mut dims = [
        Dim::new(Vec3i32::x, &input),
        Dim::new(Vec3i32::y, &input),
        Dim::new(Vec3i32::z, &input),
    ];

    let stopper = |moons: &[Vec3i32], vels: &[Vec3i32]| {
        dims.iter_mut().for_each(|dim| {
            if !dim.found {
                dim.cycle += 1;

                let state: Vec<(i32, i32)> = moons.iter().zip(vels)
                    .map(|(m, v)| {
                        ((dim.selector)(m), (dim.selector)(v))
                    }).collect();

                if state == dim.initial {
                    dim.found = true;
                }
            }
        });

        dims.iter().all(|d| d.found)
    };

    run(usize::MAX, stopper);

    let answer = lcm(&dims.into_iter().map(|d| d.cycle).to_vec());
    assert_eq!(answer, 334945516288044);
}