use aoc_common::IteratorExt;

const INPUT: &str = "01110110101001000";

fn enumerate<F>(seed: &[bool], f: &mut F)
    where F: FnMut(bool) -> bool
{
    fn recurse<F>(level: usize, rev: bool, seed: &[bool], f: &mut F) -> bool
        where F: FnMut(bool) -> bool
    {
        if level == 0 {
            if rev {
                for i in seed.iter().rev().copied() {
                    if !f(!i) {
                        return false;
                    }
                }
            } else {
                for i in seed.iter().copied() {
                    if !f(i) {
                        return false;
                    }
                }
            }
        } else {
            if !recurse(level - 1, false, seed, f) {
                return false;
            }

            if !f(rev) {
                return false;
            }

            if !recurse(level - 1, true, seed, f) {
                return false;
            }
        }

        true
    }

    for i in seed.iter().copied() {
        if !f(i) {
            return;
        }
    }

    let mut level = 0;
    loop {
        if !f(false) {
            return;
        }

        if !recurse(level, true, seed, f) {
            return;
        }

        level += 1;
    }
}

fn checksum(mut len: usize) -> String {
    let mut reductions = 0;

    while len % 2 == 0 {
        len /= 2;
        reductions += 1;
    }

    let take = usize::pow(2, reductions);

    let mut take_idx = 0;
    let mut checksum_idx = 0;

    let mut take_buff: Vec<bool> = vec![false; take];
    let mut checksum_buff: Vec<bool> = vec![false; len];

    let seed = INPUT.chars().map(|c| c == '1').to_vec();

    enumerate(&seed, &mut |b| {
        take_buff[take_idx] = b;
        take_idx += 1;
        if take_idx == take {
            take_idx = 0;

            let mut size = take;
            loop {
                size /= 2;
                if size == 0 { break; }

                for i in 0..size {
                    take_buff[i] = take_buff[i * 2] == take_buff[i * 2 + 1];
                }
            }

            checksum_buff[checksum_idx] = take_buff[0];
            checksum_idx += 1;
            if checksum_idx == len {
                return false;
            }
        }

        true
    });

    checksum_buff.into_iter().map(|b| {
        if b { '1' } else { '0' }
    }).collect()
}
  
#[test]
fn part1() {
    let answer: String = checksum(272);
    assert_eq!(answer, "11100111011101111");
}

#[test]
fn part2() {
    let answer: String = checksum(35651584);
    assert_eq!(answer, "10001110010000110");
}