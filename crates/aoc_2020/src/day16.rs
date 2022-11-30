use aoc_common::file_lines;

struct Param {
    name: String,
    r0: (u32, u32),
    r1: (u32, u32),
}

fn input() -> (Vec<Param>, Vec<u32>, Vec<Vec<u32>>) {
    let mut lines = file_lines("inputs/day16.txt");

    let mut params: Vec<Param> = Vec::new();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }

        let mut split = line.split(':');
        let name = split.next().unwrap().to_string();
        let rest = split.next().unwrap().split(' ').collect::<Vec<_>>();

        let mut nums = rest[1].split('-');
        let r00: u32 = nums.next().unwrap().parse().unwrap();
        let r01: u32 = nums.next().unwrap().parse().unwrap();

        let mut nums = rest[3].split('-');
        let r10: u32 = nums.next().unwrap().parse().unwrap();
        let r11: u32 = nums.next().unwrap().parse().unwrap();

        params.push(Param { name: name, r0: (r00, r01), r1: (r10, r11) });
    }

    lines.next().unwrap();
    let my_ticket: Vec<u32> = lines.next().unwrap().split(',').map(|n| n.parse().unwrap()).collect();

    lines.next().unwrap();
    lines.next().unwrap();

    let other_tickets: Vec<Vec<u32>> = lines.map(|l| l.split(',').map(|n| n.parse().unwrap()).collect()).collect();

    (params, my_ticket, other_tickets)
}

#[test]
fn part1() {
    let (params, _, tickets) = input();

    let mut answer = 0;
    for ticket in tickets {
        'outer: for val in ticket {
            for param in params.iter() {
                if (val >= param.r0.0 && val <= param.r0.1) || (val >= param.r1.0 && val <= param.r1.1) {
                    continue 'outer;
                }
            }
            answer += val;
        }
    }

    assert_eq!(answer, 30869)
}

#[test]
fn part2() {
    let (params, my_ticket, tickets) = input();

    let tickets = tickets.into_iter().filter(|t| {
        'outer: for val in t {
            for param in params.iter() {
                if (*val >= param.r0.0 && *val <= param.r0.1) || (*val >= param.r1.0 && *val <= param.r1.1) {
                    continue 'outer;
                }
            }
            return false;
        }
        true
    });

    let mut map = vec![vec![true; params.len()]; params.len()];

    for ticket in tickets {
        for (i, val) in ticket.into_iter().enumerate() {
            for (j, param) in params.iter().enumerate() {
                if (val < param.r0.0 || val > param.r0.1) && (val < param.r1.0 || val > param.r1.1) {
                    map[j][i] = false;
                }
            }
        }
    }

    let mut map = map.into_iter().enumerate().collect::<Vec<_>>();
    map.sort_by_cached_key(|i| i.1.iter().filter(|f| **f).count());

    let mut orderered_params: Vec<Option<&Param>> = vec![None; params.len()];

    'outer: for (i, bools) in map {
        for (j, b) in bools.into_iter().enumerate() {
            if b && orderered_params[j].is_none() {
                orderered_params[j] = Some(&params[i]);
                continue 'outer;
            }
        }
    }

    let orderered_params = orderered_params.into_iter().map(|p| p.unwrap()).collect::<Vec<_>>();

    let answer = orderered_params.into_iter().enumerate().filter(|(_, p)| p.name.starts_with("departure")).fold(1u64, |acc, p| acc * my_ticket[p.0] as u64);

    assert_eq!(answer, 4381476149273);
}