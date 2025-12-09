use aoc_common::{OrdWrapper, PriorityQueue, PriorityQueueBuilder, Vec3i64, file_lines};

fn input() -> (Vec<Vec3i64>, PriorityQueue<(usize, usize), OrdWrapper<f64>>) {
    let points: Vec<Vec3i64> = file_lines("inputs/day08.txt").map(|l| {
        l.split(',').map(|s| s.parse().unwrap()).collect()
    }).collect();

    let mut builder: PriorityQueueBuilder<(usize, usize), OrdWrapper<f64>> = PriorityQueueBuilder::with_capacity(points.len()  * points.len() - 1);
    for i in 0..points.len() - 1 {
        for j in i + 1 .. points.len() {
            let p0 = points[i];
            let p1 = points[j];
            let diff = p0 - p1;
            let squared = Vec3i64::new(diff.x * diff.x, diff.y * diff.y, diff.z * diff.z);
            let dist = f64::sqrt(squared.x as f64 + squared.y as f64 + squared.z as f64);
            builder.push((i, j), OrdWrapper(dist));
        }
    }

    let queue: PriorityQueue<(usize, usize), OrdWrapper<f64>> = builder.build();
    (points, queue)
}

struct SetList {
    lookup: Vec<Option<usize>>,
    sets: Vec<Vec<usize>>,
}

impl SetList {
    fn new(cap: usize) -> Self {
        Self {
            lookup: vec![None; cap],
            sets: Vec::with_capacity(cap),
        }
    }

    fn insert(&mut self, i: usize, j: usize) -> usize {
        let i_set_idx = self.lookup[i];
        let j_set_idx = self.lookup[j];

        match (i_set_idx, j_set_idx) {
            (None, None) => {
                let idx = self.sets.len();
                let new_set = vec![i, j];
                self.sets.push(new_set);
                self.lookup[i] = Some(idx);
                self.lookup[j] = Some(idx);
                idx
            }
            (Some(i_set_idx), None) => {
                self.sets[i_set_idx].push(j);
                self.lookup[j] = Some(i_set_idx);
                i_set_idx
            },
            (None, Some(j_set_idx)) => {
                self.sets[j_set_idx].push(i);
                self.lookup[i] = Some(j_set_idx);
                j_set_idx
            },
            (Some(i_set_idx), Some(j_set_idx)) => {
                if i_set_idx == j_set_idx {
                    i_set_idx
                } else {
                    let (src, dst) = if self.sets[i_set_idx].len() < self.sets[j_set_idx].len() {
                        (i_set_idx, j_set_idx)
                    } else {
                        (j_set_idx, i_set_idx)
                    };

                    let [src_vec, dst_vec] = self.sets.get_disjoint_mut([src, dst]).unwrap();
                    for idx in src_vec.iter() {
                        self.lookup[*idx] = Some(dst);
                    }
                    dst_vec.append(src_vec);
                    dst
                }
            },
        }
    }

    fn sets(&self) -> &[Vec<usize>] {
        &self.sets
    }
}

#[test]
fn part1() {
    const SIZE: usize = 1000;

    let (points, queue) = input();
    let mut set_list = SetList::new(points.len());
    for ((i, j), _) in queue.into_iter_sorted().take(SIZE) {
        set_list.insert(i, j);
    }

    let mut max0 = 0;
    let mut max1 = 0;
    let mut max2 = 0;

    for set in set_list.sets() {
        let len = set.len();
        if len == 0 { continue; }
        if len > max0 {
            max2 = max1;
            max1 = max0;
            max0 = len;
        } else if len > max1 {
            max2 =  max1;
            max1 = len;
        } else if len > max2 {
            max2 = len;
        }
    }

    let answer = max0 * max1 * max2;
    assert_eq!(163548, answer);
}

#[test]
fn part2() {
    let (points, queue) = input();
    let mut set_list = SetList::new(points.len());
    let mut answer = 0;
    for ((i, j), _) in queue.into_iter_sorted() {
        let set_idx = set_list.insert(i, j);
        if set_list.sets()[set_idx].len() == points.len() {
            answer = points[i].x * points[j].x;
            break;
        }
    }
    
    assert_eq!(772452514, answer);
}