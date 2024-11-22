use aoc_common::{file_lines, Vec2us};

fn input() -> Vec<Vec<bool>> {
    file_lines("inputs/day18.txt").map(|l| {
        l.bytes().map(|b| b == b'#').collect()
    }).collect()
}

fn run(fixup: fn(&mut Vec<Vec<bool>>, Vec2us)) -> i32 {
    let mut current = input();
    let mut next = current.clone();
    
    let bounds = Vec2us::new(current[0].len(), current.len());

    fixup(&mut current, bounds);

    for _ in 0..100 {
        for j in 0..bounds.y {
            for i in 0..bounds.x {
                let mut count_on = 0;
                for Vec2us { x: u, y: v } in Vec2us::new(i, j).surrouding_bounded(&bounds) {
                    if current[v][u] {
                        count_on += 1;
                    }
                }

                next[j][i] = match (current[j][i], count_on) {
                    (true, 2 | 3 ) => true,
                    (false, 3) => true,
                    _ => false
                };
            }
        }

        std::mem::swap(&mut current, &mut next);
        fixup(&mut current, bounds);
    }

    let mut count = 0;
    for j in 0..bounds.y {
        for i in 0..bounds.x {
            if current[j][i] {
                count += 1;
            }
        }
    }

    count
}

#[test]
fn part1 () {
    fn nop(_: &mut Vec<Vec<bool>>, _: Vec2us) { }

    let answer = run(nop);
    assert_eq!(answer, 821);
}

#[test]
fn part2 () {
    fn fixup(map: &mut Vec<Vec<bool>>, bounds: Vec2us) {
        map[0][0] = true;
        map[0][bounds.x - 1] = true;
        map[bounds.y - 1][0] = true;
        map[bounds.y - 1][bounds.x - 1] = true;
    }

    let answer = run(fixup);
    assert_eq!(answer, 886);
}