const INPUT: &'static str = "bgvyzdsv";

fn run(mask: u8) -> i32 {
    let mut i = 0;
    loop {
        let s = format!("{}{}", INPUT, i);
        let digest = md5::compute(s);

        if digest[0] == 0 && digest[1] == 0 && (digest[2] & mask) == 0 {
            break;
        }

        i += 1;
    }
    i
}

#[test]
fn part1() {
    let answer = run(0xF0);
    assert_eq!(answer, 254575);
}

#[test]
fn part2() {
    let answer = run(0xFF);
    assert_eq!(answer, 1038736);
}