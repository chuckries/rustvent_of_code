use aoc_common::file_string;

fn input() -> Vec<(String, String)> {
    file_string("inputs/day02.txt").split(',').map(|r| {
        let mut split = r.split('-').map(|s| s.to_string());
        (split.next().unwrap(), split.next().unwrap())
    }).collect()
}

fn count<F>(f: F) -> u64
    where F: Fn(&[u8]) -> bool
{
    let mut total = 0;
    for (front, back) in input() {
        let begin: u64 = front.parse().unwrap();
        let end = back.parse().unwrap();

        let mut b = front.into_bytes();
        for i in begin ..= end {
            if f(&b) {
                total += i;
            }

            let mut idx = b.len() - 1;
            loop {
                b[idx] += 1;
                if b[idx] > b'9' {
                    b[idx] = b'0';
                    if idx == 0 {
                        b.insert(0, b'1');
                        break;
                    } else {
                        idx -= 1;
                    }
                } else {
                    break;
                }
            }
        }
    }
    total
}

#[test]
fn part1() {
    let answer = count(|b| {
        let mid = b.len() / 2;
        b[..mid] == b[mid..]
    });
    assert_eq!(23701357374, answer);
}

#[test]
fn part2() {
    let answer = count(|b| {
        'outer: for i in 1 ..= b.len() / 2 {
            if b.len() % i != 0 {
                continue;
            }       

            let mut j = 0;
            let mut k = i;
            while k < b.len() {
                if b[j] != b[k] {
                    continue 'outer;
                }
                j += 1;
                k += 1;
            }

            return true;
        }

        false
    });
    assert_eq!(34284458938, answer);
}