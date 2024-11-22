const INPUT: &str = "abbhdwsy";
const LENGTH: usize = 8;

#[test]
fn part1() {
    let mut password = String::new();
    for (digit, _) in Md5Iter::new(INPUT).into_iter() {
        let c = if digit < 10 {
            digit + b'0'
        } else {
            digit - 10 + b'a'
        } as char;

        password.push(c);
        if password.len() >= LENGTH {
            break;
        }
    }

    assert_eq!(password, "801b56a7");
}

#[test]
fn part2() {
    let mut password = vec!['\0'; LENGTH];
    let mut found = 0;
    for (pos, digit) in Md5Iter::new(INPUT).into_iter() {
        let pos = pos as usize;

        if pos < password.len() && password[pos] == '\0' {
            let c = if digit < 10 {
                digit + b'0'
            } else {
                digit - 10 + b'a'
            } as char;

            password[pos] = c;
            found += 1;
            if found >= password.len() {
                break;
            }
        }
    }

    let password: String = password.into_iter().collect();
    assert_eq!(password, "424a0197");
}

struct Md5Iter {
    seed: String,
    n: i32,
}

impl Md5Iter {
    fn new(seed: &str) -> Self {
        Self {
            seed: seed.to_string(),
            n: 0
        }   
    }
}

impl Iterator for Md5Iter {
    type Item = (u8, u8);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.n += 1;
            let s = format!("{}{}", self.seed, self.n - 1);
            let hash = md5::compute(s);
            if hash[0] == 0 && hash[1] == 0 && hash[2] & 0xF0 == 0 {
                return Some((hash[2] & 0x0F, hash[3] >> 4));
            }
        }
    }
}