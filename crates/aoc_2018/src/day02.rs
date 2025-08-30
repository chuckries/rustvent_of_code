use aoc_common::{file_lines, IteratorExt};

fn input() -> Vec<String> {
    file_lines("inputs/day02.txt").collect()
}

#[test]
fn part1() {
    let mut twos = 0;
    let mut threes = 0;

    for s in input() {
        let counts = s.bytes().counts::<usize>();
        
        for count in counts.values() {
            if *count == 2 {
                twos += 1;
                break;
            }
        }

        for count in counts.values() {
            if *count == 3 {
                threes += 1;
                break;
            }
        }
    }

    let answer = twos * threes;
    assert_eq!(5952, answer);
}

#[test]
fn part2() {
    fn func() -> String {
        let input = input();
        for i in 0 .. input.len() - 1 {
            'outer: for j in i + 1 .. input.len() {
                let a = &input[i];
                let b = &input[j];

                let mut found_idx: Option<usize> = None;
                for (idx, (u, v)) in a.bytes().zip(b.bytes()).enumerate() {
                    if u != v {
                        if found_idx.is_some() {
                            continue 'outer;
                        }
                        found_idx = Some(idx);
                    }
                }

                let found_idx = found_idx.unwrap();
                let mut s = String::with_capacity(a.len());
                s.push_str(str::from_utf8(&a.as_bytes()[..found_idx]).unwrap());
                s.push_str(str::from_utf8(&a.as_bytes()[found_idx + 1 ..]).unwrap());
                return s.to_string();
            }
        }
        panic!();
    }

    let answer = func();
    assert_eq!("krdmtuqjgwfoevnaboxglzjph", answer);
}