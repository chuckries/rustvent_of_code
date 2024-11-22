use aoc_common::file_lines;

fn input() -> impl Iterator<Item = String> {
    file_lines("inputs/day05.txt")
}

#[test]
fn part1() {
    let answer = input().filter(|s| {
        let mut vowels = 0;
        let mut previous = b'\0';
        let mut has_double = false;
        for b in s.bytes() {
            if !has_double && b == previous {
                has_double = true;
            }

            match b {
                b'a' | b'e' | b'i' | b'o' | b'u' => vowels += 1,
                b'b' if previous == b'a' => return false,
                b'd' if previous == b'c' => return false,
                b'q' if previous == b'p' => return false,
                b'y' if previous == b'x' => return false,
                _ => (),
            }

            previous = b;
        }

        vowels >= 3 && has_double
    }).count();

    assert_eq!(answer, 255);
}

#[test]
fn part2() {
    let answer = input().filter(|s| {
        let mut found_double = false;
        let mut found_split = false;
        let bytes = s.as_bytes();

        'outer: for i in 0 .. bytes.len() - 3 {
            for j in i + 2 .. bytes.len() - 1 {
                if bytes[i .. i + 2] == bytes[j .. j + 2] {
                    found_double = true;
                    break 'outer;
                }
            }
        }

        for i in 0..bytes.len() - 2 {
            if bytes[i] == bytes[i + 2] {
                found_split = true;
                break;
            }
        }

        found_double && found_split
    }).count();

    assert_eq!(answer, 55);
}