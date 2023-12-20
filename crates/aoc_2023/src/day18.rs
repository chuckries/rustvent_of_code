use std::{collections::HashMap, cmp::Ordering} ;

use aoc_common::{file_lines, IteratorExt, Vec2i64};

struct Line {
    p0: Vec2i64,
    p1: Vec2i64,
}

impl Line {
    fn new(mut p0: Vec2i64, mut p1: Vec2i64) -> Self {
        if p0.x == p1.x {
            if p1.y < p0.y {
                (p0, p1) = (p1, p0)
            }
        } else if p0.y == p1.y {
            if p1.x < p0.x {
                (p0, p1) = (p1, p0)
            }
        } else {
            panic!()
        }

        Self {
            p0, 
            p1
        }
    }
}

fn point_cmp(lhs: &Vec2i64, rhs: &Vec2i64) -> Ordering {
    let mut ord = lhs.x.cmp(&rhs.x);
    if ord == Ordering::Equal {
        ord = lhs.y.cmp(&rhs.y);
    }
    ord
}

fn parse_hex(s: &str) -> i64 {
    let mut base = 1;
    let mut total = 0;
    for b in s.bytes().rev() {
        let digit = match b {
            b'0' ..= b'9' => b - b'0',
            b'a' ..= b'f' => b - b'a' + 10,
            _ => panic!(),
        } as i64;
        total += base * digit;
        base *= 16;
    }
    total
}

fn run(input: &[(char, i64)]) -> i64 {
    let mut lines: Vec<Line> = Vec::new();
    let mut p0 = Vec2i64::zero();
    for (c, n) in input.iter().copied() {
        let n = n as i64;
        let delta = match c {
            'L' => Vec2i64::new(-n,  0),
            'R' => Vec2i64::new( n,  0),
            'U' => Vec2i64::new( 0, -n),
            'D' => Vec2i64::new( 0,  n),
            _ => panic!(),
        };

        let p1 = p0 + delta;
        lines.push(Line::new(p0, p1));
        p0 = p1;
    }

    let mut verts: HashMap<Vec2i64, Vec2i64> = HashMap::new();
    let mut horis: HashMap<Vec2i64, Vec2i64> = HashMap::new();

    let mut area = 0;
    for l in lines.iter() {
        if l.p0.x == l.p1.x {
            verts.insert(l.p0, l.p1);
            verts.insert(l.p1, l.p0);
        } else if l.p0.y == l.p1.y {
            horis.insert(l.p0, l.p1);
            horis.insert(l.p1, l.p0);
        } else { panic!() }
    }

    while let Some(min_p) = verts.keys().min_by(|lhs, rhs| point_cmp(*lhs, *rhs)) {
        let p0 = *min_p;
        let p1 = verts.remove(&p0).unwrap();
        verts.remove(&p1).unwrap();

        let h00 = p0;
        let h01 = horis.remove(&h00).unwrap();
        horis.remove(&h01).unwrap();

        let h10 = p1;
        let h11 = horis.remove(&h10).unwrap();
        horis.remove(&h11).unwrap();

        let v00 = h01;
        let v01 = verts[&v00];

        let v10 = h11;
        let v11 = verts[&v10];

        let mut removed_lines: Vec<Line> = Vec::new();
        removed_lines.push(Line::new(p0, p1));
        removed_lines.push(Line::new(h00, h01));
        removed_lines.push(Line::new(h10, h11));

        let mut delta = 0;

        if v01 == v10 {
            // enclosed
            verts.remove(&v00).unwrap();
            verts.remove(&v10).unwrap();

            removed_lines.push(Line::new(v00, v01));

            delta += (h01.x - h00.x + 1) * (p1.y - p0.y + 1);
        } else {

            let min_h = h01.x.min(h11.x);

            let interior_lines = verts.keys().filter(|p| {
                if p.y > p0.y && p.y < p1.y && p.x > p0.x && p.x <= min_h {
                    if **p != h01 && verts[*p] != h01 && **p != h11 && verts[*p] != h11 {
                        return true;
                    }
                }
                false
            }).sorted_by(|lhs, rhs| point_cmp(*lhs, *rhs)).cloned().to_vec();

            if interior_lines.len() > 0 {
                let interior_lines = interior_lines.iter().copied().filter(|p| p.x == interior_lines[0].x).to_vec();
                let interior_lines = interior_lines.into_iter().enumerate().filter_map(|(idx, p)| if idx & 1 > 0 { None } else { Some(p) }).to_vec();
                let interior_lines = interior_lines.into_iter().map(|p| Line::new(p, verts[&p])).to_vec();

                let target_x = interior_lines[0].p0.x;

                delta += (target_x - p0.x) * (p1.y - p0.y + 1);

                for l in interior_lines.iter() {
                    verts.remove(&l.p0).unwrap();
                    verts.remove(&l.p1).unwrap();
                    removed_lines.push(Line::new(l.p0, l.p1));

                    delta += l.p1.y - l.p0.y - 1;
                }

                let int_p0 = interior_lines[0].p0;
                let int_pn = interior_lines.last().unwrap().p1;

                if h01.x != target_x {
                    let new_p = Vec2i64::new(target_x, h01.y);
                    horis.insert(new_p, h01);
                    horis.insert(h01, new_p);
                    verts.insert(new_p, int_p0);
                    verts.insert(int_p0, new_p);
                }

                if h11.x != target_x {
                    let new_p = Vec2i64::new(target_x, h11.y);
                    horis.insert(new_p, h11);
                    horis.insert(h11, new_p);
                    verts.insert(new_p, int_pn);
                    verts.insert(int_pn, new_p);
                }

                if v00.x == target_x {
                    verts.remove(&v00).unwrap();
                    verts.remove(&v01).unwrap();
                    removed_lines.push(Line::new(v00, v01));

                    verts.insert(v01, int_p0);
                    verts.insert(int_p0, v01);

                    delta += 0.max(v01.y - v00.y);
                }

                if v10.x == target_x {
                    verts.remove(&v10).unwrap();
                    verts.remove(&v11).unwrap();
                    removed_lines.push(Line::new(v10, v11));

                    verts.insert(v11, int_pn);
                    verts.insert(int_pn, v11);

                    delta += 0.max(v10.y - v11.y);
                }

                for w in interior_lines.windows(2) {
                    let p01 = w[0].p1;
                    let p10 = w[1].p0;
                    verts.insert(p01, p10);
                    verts.insert(p10, p01);
                }
            } else {
                // no interior lines
                removed_lines.push(Line::new(v00, v01));
                removed_lines.push(Line::new(v10, v11));

                if h01.x == h11.x {
                    verts.remove(&v00).unwrap();
                    verts.remove(&v10).unwrap();
                    verts.insert(v01, v11).unwrap();
                    verts.insert(v11, v01).unwrap();

                    delta += (h01.x - h00.x) * (p1.y - p0.y + 1);
                    delta += 0.max(v01.y - v00.y);
                    delta += 0.max(v10.y - v11.y);
                } else if  h01.x > h11.x {
                    let new_p = Vec2i64::new(h11.x, v00.y);

                    verts.remove(&v10).unwrap();
                    verts.insert(v11, new_p).unwrap();
                    verts.insert(new_p, v11);
                    horis.insert(new_p, v00);
                    horis.insert(v00, new_p);

                    delta += (h11.x - h10.x) * (p1.y - p0.y + 1);
                    delta += 0.max(v10.y - v11.y);
                } else if h01.x < h11.x {
                    let new_p = Vec2i64::new(h01.x, v10.y);

                    verts.remove(&v00).unwrap();
                    verts.insert(v01, new_p).unwrap();
                    verts.insert(new_p, v01);
                    horis.insert(new_p, v10);
                    horis.insert(v10, new_p);

                    delta += (h01.x - h00.x) * (p1.y - p0.y + 1);
                    delta += 0.max(v01.y - v00.y);
                } else {
                    panic!();
                }
            }
        }

        area += delta;
    }

    area
}

#[test]
fn part1_2() {
    let input = file_lines("inputs/day18.txt").map(|l| {
        let mut split = l.split_whitespace();
        (split.next().unwrap().chars().next().unwrap(), split.next().unwrap().parse().unwrap())
    }).to_vec();

    let answer = run(&input);
    assert_eq!(72821, answer);
}

#[test]
fn part2() {
    let input = file_lines("inputs/day18.txt").map(|l| {
        let hex = l.split_whitespace().skip(2).next().unwrap();
        let hex = &hex[2..hex.len() - 1];

        let num = parse_hex(&hex[0 .. hex.len() - 1]);
        let dir = match &hex[hex.len() - 1 ..] {
            "0" => 'R',
            "1" => 'D',
            "2" => 'L',
            "3" => 'U',
            _ => panic!()
        };

        (dir, num)
    }).to_vec();

    let answer = run(&input);
    assert_eq!(127844509405501, answer);
}