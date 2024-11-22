const INPUT: usize = 36000000;

#[test]
fn part1() {
    let target = INPUT / 10;
    let mut vec = vec![0; target + 1];

    let mut i = 1;
    while i <= target {
        let mut idx = i;
        while idx <= target {
            vec[idx] += i;
            idx += i;
        }

        if vec[i] >= target {
            break;
        }

        i += 1;
    }

    assert_eq!(i, 831600);
}

#[test]
fn part2() {
    let target = INPUT / 11;
    let mut vec = vec![0; target + 1];

    let mut i = 1;
    while i <= target {
        let mut idx = i;
        let mut count = 0;
        while idx <= target && count < 50 {
            vec[idx] += i;
            idx += i;
            count += 1;
        }

        if vec[i] >= target {
            break;
        }

        i += 1;
    }

    assert_eq!(i, 884520);
}