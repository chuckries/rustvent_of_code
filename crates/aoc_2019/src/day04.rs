fn increasing_digits() -> impl Iterator<Item = Vec<i32>> {
    (158126..=624574)
    .map(|n| {
        let mut digits = Vec::new();
        let mut current = n;
        while current > 0 {
            digits.push(current % 10);
            current /= 10;
        }
        digits
    })
    .filter(|digits| {
        digits.windows(2).all(|w| w[0] >= w[1])
    })
}

#[test]
fn part1() {
    let answer = increasing_digits()
        .filter(|digits| {
            digits.windows(2).any(|w| w[0] == w[1])
        })
        .count();

    assert_eq!(answer, 1665);
}

#[test]
fn part2(){
    let answer = increasing_digits()
        .filter(|digits| {
            let mut count = 1;
            let mut current = digits[0];
            for next in digits[1..].iter() {
                if *next == current {
                    count += 1;
                } else {
                    if count == 2 {
                        return true;
                    }
                    count = 1;
                    current = *next;
                }
            }
            return count == 2;
        })
        .count();

    assert_eq!(answer, 1131);
}