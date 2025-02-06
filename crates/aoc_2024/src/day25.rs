use aoc_common::file_lines;

#[derive(PartialEq, Eq)]
enum Type {
    Lock,
    Key,
}

struct Thing {
    r#type: Type,
    lengths: [i32; 5],
}

fn input() -> Vec<Thing> {
    let mut things: Vec<Thing> = Vec::new();
    let mut lines =  file_lines("inputs/day25.txt");
    
    loop {
        let line = lines.next().unwrap();
        let r#type =  if line.starts_with("#") {
            Type::Lock
        } else {
            Type::Key
        };

        let mut counts = [0; 5];

        for _ in 0..5 {
            for (idx, c) in lines.next().unwrap().bytes().enumerate() {
                if c == b'#' {
                    counts[idx] += 1;
                }
            }
        }
        lines.next().unwrap();

        things.push(Thing {
            r#type,
            lengths: counts,
        });

        if lines.next().is_none() {
            break;
        }
    }

    things
}

#[test]
fn part1() {
    let things = input();

    let mut total = 0;
    for i in 0..things.len() - 1 {
        'outer: for j in i + 1 .. things.len() {
            let a = &things[i];
            let b = &things[j];

            if a.r#type != b.r#type {
                for idx in 0..5 {
                    if a.lengths[idx] + b.lengths[idx] > 5 {
                        continue 'outer;
                    }
                }
                total += 1;
            }
        }
    }

    assert_eq!(total, 3162);
}