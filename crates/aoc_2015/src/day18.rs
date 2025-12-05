use aoc_common::{Grid, Vec2us};

fn input() -> Grid<bool> {
    Grid::file_as_grid("inputs/day18.txt", &mut |b, _| b == b'#')
}

fn run<F>(fixup: F) -> usize 
    where F: Fn(&mut Grid<bool>)
{
    let mut current = input();
    let mut next = current.clone();

    fixup(&mut current);

    for _ in 0..100 {
        for (p, b) in current.enumerate() {
            let count = current.surrounding(p).filter(|b| **b).count();
            next[p] = match count {
                2 | 3 if *b => true,
                3 if !*b => true,
                _ => false,
            };
        }

        std::mem::swap(&mut current, &mut next);
        fixup(&mut current);
    }

    let count = current.iter().filter(|b| **b).count();
    count
}

#[test]
fn part1 () {
    #[inline]
    fn nop(_: &mut Grid<bool>) { }

    let answer = run(nop);
    assert_eq!(answer, 821);
}

#[test]
fn part2 () {
    #[inline]
    fn fixup(map: &mut Grid<bool>) {
        let last = map.bounds() - Vec2us::one();

        map[(0, 0)] = true;
        map[(last.x, 0)] = true;
        map[(0, last.y)] = true;
        map[(last.x, last.y)] = true;
    }

    let answer = run(fixup);
    assert_eq!(answer, 886);
}