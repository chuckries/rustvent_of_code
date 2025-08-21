use aoc_common::{Vec2us, Vec3us};

const INPUT: i64 = 6548;

fn power_levels() -> [[i64; 300]; 300]
{
    let mut levels = [[0; 300]; 300];

    for y in 0..300 {
        for x in 0..300 {
            let rack_id = x as i64 + 1 + 10;
            let mut power_level = rack_id * (y as i64 + 1);
            power_level += INPUT;
            power_level *= rack_id;
            power_level = (power_level % 1000) / 100;
            power_level -= 5;
            levels[y][x] = power_level
        }
    }

    levels
}

#[test]
fn part1() {
    let levels = power_levels();
    let mut max_level = i64::MIN;
    let mut max_idx = Vec2us::zero();

    for j in 0..298 {
        for i in 0..298 {
            let total = 
            levels[j + 0][i + 0] + levels[j + 0][i + 1] + levels[j + 0][i + 2] + 
            levels[j + 1][i + 0] + levels[j + 1][i + 1] + levels[j + 1][i + 2] +
            levels[j + 2][i + 0] + levels[j + 2][i + 1] + levels[j + 2][i + 2];

            if total > max_level {
                max_level = total;
                max_idx = (i + 1, j + 1).into();
            }
        }
    }

    assert_eq!(max_idx, (21, 53).into());
}

#[test]
fn part2() {
    let levels = power_levels();
    let mut max_level = i64::MIN;
    let mut max_idx = Vec3us::zero();

    let mut cached: [Vec<Vec<i64>>; 300] = [const { Vec::new() }; 300];

    cached[0] = vec![vec![0; 300]; 300];
    for j in 0..300 {
        for i in 0..300 {
            cached[0][j][i] = levels[j][i];
        }
    }

    for k in 2..=300 {
        cached[k - 1] = vec![vec![0; 300 - k + 1]; 300 - k + 1];
        
        for j in 0..= 300 - k {
            for i in 0..= 300 - k {
                let mut base = cached[k - 2][j][i];

                for n in 0..k {
                    base += levels[j + n][i + k - 1];
                }

                for n in 0..k - 1 {
                    base += levels[j + k - 1][i + n];
                }

                if base > max_level {
                    max_level = base;
                    max_idx = (i + 1, j + 1, k).into();
                }

                cached[k - 1][j][i] = base;
            }
        }
    }

    assert_eq!(max_idx, (233,250,12).into());
}