const STEP: usize = 355;

#[test]
fn part1() {
    let mut buff = vec![0];
    let mut pos = 0;

    for i in 1..=2017 {
        pos = (pos + STEP) % i + 1;
        buff.insert(pos, i);
    }

    let answer = buff[(pos + 1) % buff.len()];
    assert_eq!(answer, 1912);
}

#[test]
fn part2() {
    let mut current = 0;
    let mut answer = 0;
    for i in 1..=50000000 {
        current = (current + STEP) % i + 1;
        if 1 == current {
            answer = i;
        }
    }

    assert_eq!(answer, 21066990);
}