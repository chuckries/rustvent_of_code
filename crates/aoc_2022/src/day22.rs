use aoc_common::{file_lines, IteratorExt, Vec2i32, Vec3i32, Vec2us};

enum Dir {
    Walk(i32),
    Turn(char),
}

fn input() -> (Vec<Vec<char>>, Vec<Dir>, Vec2i32, Vec2i32) {
    let mut lines = file_lines("inputs/day22.txt");

    let mut map: Vec<Vec<char>> = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        map.push(line.chars().to_vec());
    }

    let start_x = map[0].iter().enumerate().find(|(_, c)| **c == '.').unwrap().0;
    let start_pos = Vec2i32::new(start_x as i32, 0);
    let start_dir = Vec2i32::unit_x();

    let mut dirs: Vec<Dir> = Vec::new();
    let line = lines.next().unwrap().chars().to_vec();
    let mut start = 0;
    let mut current = 0;

    while start < line.len() {
        if current == line.len() || matches!(line[current], 'L' | 'R') {
            dirs.push(Dir::Walk(line[start..current].iter().collect::<String>().parse().unwrap()));
            if current < line.len() {
                dirs.push(Dir::Turn(line[current]));
            }
            current += 1;
            start = current;
        } else {
            current += 1;
        }
    }

    (map, dirs, start_pos, start_dir)
}

#[test]
fn part1() {
    let (map, dirs, mut pos, mut dir) = input();

    for d in dirs {
        match d {
            Dir::Turn(c) => {
                dir = match c {
                    'L' => dir.rotate_left(),
                    'R' => dir.rotate_right(),
                    _ => panic!(),
                };
            },
            Dir::Walk(steps) => {
                for _ in 0..steps {
                    let mut next = pos + dir;

                    if next.y < 0 || next.y >= map.len() as i32 || next.x < 0 || next.x >= map[next.y as usize].len() as i32 || map[next.y as usize][next.x as usize] == ' ' {
                        let mut rev = pos;
                        loop {
                            let next_rev = rev - dir;
                            if next_rev.y < 0 || next_rev.y >= map.len() as i32 || next_rev.x < 0 || next_rev.x >= map[next_rev.y as usize].len() as i32 || map[next_rev.y as usize][next_rev.x as usize] == ' ' {
                                break;
                            }
                            rev = next_rev;
                        }
                        next = rev;
                    }

                    if map[next.y as usize][next.x as usize] == '#' {
                        break;
                    }

                    pos = next;
                }
            }
        }
    }

    let answer = ((pos.y + 1) * 1000) + ((pos.x + 1) * 4) + match (dir.x, dir.y) {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => panic!()
    };

    assert_eq!(answer, 36518);
}

fn rot_x_3(v: Vec3i32) -> Vec3i32 {
    Vec3i32::new(
        v.x,
        v.z,
        -v.y
    )
}

fn rot_x_neg_3(v: Vec3i32) -> Vec3i32 {
    Vec3i32::new(
        v.x,
        -v.z,
        v.y
    )
}

fn rot_y_3(v: Vec3i32) -> Vec3i32 {
    Vec3i32::new(
        v.z,
        v.y,
        -v.x,
    )
}

fn rot_y_neg_3(v: Vec3i32) -> Vec3i32 {
    Vec3i32::new(
        -v.z,
        v.y,
        v.x,
    )
}

#[derive(Clone, Copy)]
struct Orientation {
    left: Vec3i32,
    up: Vec3i32,
}

impl Orientation {
    fn new() -> Self {
        Self {
            left: -Vec3i32::unit_x(),
            up: Vec3i32::unit_y(),
        }
    }

    fn get_front_face_and_rotation(&self) -> (usize, usize) {
        match (self.left.into(), self.up.into()) {
            // front is front
            ((-1,  0,  0), ( 0,  1,  0)) => (0, 0),
            (( 0, -1,  0), (-1,  0,  0)) => (0, 1),
            (( 1,  0,  0), ( 0, -1,  0)) => (0, 2),
            (( 0,  1,  0), ( 1,  0,  0)) => (0, 3),

            // back is front
            (( 1,  0,  0), ( 0,  1,  0)) => (1, 0),
            (( 0,  1,  0), (-1,  0,  0)) => (1, 1),
            ((-1,  0,  0), ( 0, -1,  0)) => (1, 2),
            (( 0, -1,  0), ( 1,  0,  0)) => (1, 3),

            // up is front
            ((-1,  0,  0), ( 0,  0,  1)) => (2, 0),
            (( 0, -1,  0), ( 0,  0,  1)) => (2, 1),
            (( 1,  0,  0), ( 0,  0,  1)) => (2, 2),
            (( 0,  1,  0), ( 0,  0,  1)) => (2, 3),

            // down is front
            ((-1,  0,  0), ( 0,  0, -1)) => (3, 0),
            (( 0, -1,  0), ( 0,  0, -1)) => (3, 1),
            (( 1,  0,  0), ( 0,  0, -1)) => (3, 2),
            (( 0,  1,  0), ( 0,  0, -1)) => (3, 3),

            // left is front
            (( 0,  0,  1), ( 0,  1,  0)) => (4, 0),
            (( 0,  0,  1), (-1,  0,  0)) => (4, 1),
            (( 0,  0,  1), ( 0, -1,  0)) => (4, 2),
            (( 0,  0,  1), ( 1,  0,  0)) => (4, 3),

            // right is front
            (( 0,  0, -1), ( 0,  1,  0)) => (5, 0),
            (( 0,  0, -1), (-1,  0,  0)) => (5, 1),
            (( 0,  0, -1), ( 0, -1,  0)) => (5, 2),
            (( 0,  0, -1), ( 1,  0,  0)) => (5, 3),

            _ => panic!(),
        }
    }

    fn rot_x(&self) -> Self {
        Self {
            left: rot_x_3(self.left),
            up: rot_x_3(self.up)
        }
    }

    fn rot_x_neg(&self) -> Self {
        Self {
            left: rot_x_neg_3(self.left),
            up: rot_x_neg_3(self.up)
        }
    }

    fn rot_y(&self) -> Self {
        Self {
            left: rot_y_3(self.left),
            up: rot_y_3(self.up),
        }
    }

    fn rot_y_neg(&self) -> Self {
        Self {
            left: rot_y_neg_3(self.left),
            up: rot_y_neg_3(self.up),
        }
    }
}

struct FaceView<'a, const N: usize> {
    cube: &'a Cube<N>,
    face: usize,
    rotation: usize,
}

impl<'a, const N: usize> FaceView<'a, N> {
    fn idx(&self, p: Vec2i32) -> char {
        let idx = self.map_idx(p);
        self.cube.map[idx.y][idx.x]
    }

    fn map_idx(&self, p: Vec2i32) -> Vec2us {
        let rot = (self.cube.faces[self.face].initial_rotation + 4 - self.rotation) % 4;

        let offset = match rot {
            0 => p,
            1 => Vec2i32::new(p.y, N as i32 - p.x - 1),
            2 => Vec2i32::new(N as i32 - p.x - 1, N as i32 - p.y - 1),
            3 => Vec2i32::new(N as i32 - p.y - 1, p.x),
            _ => panic!()
        };

        self.cube.faces[self.face].map_origin + offset.cast()
    }
}

#[derive(Clone, Copy)]
struct CubeFace<const N: usize> {
    map_origin: Vec2us,
    initial_rotation: usize,
}

struct Cube<const N: usize> {
    map: Vec<Vec<char>>,
    faces: Vec<CubeFace<N>>
}

impl<const N: usize> Cube<N> {
    fn new(map: Vec<Vec<char>>) -> Self {

        let orientation = Orientation::new();
        let mut faces:  [Option<CubeFace<N>>; 6] = [None; 6];

        // explore the map, creating oriented faces as we go.
        let start_x = map[0].iter().enumerate().find(|(_, c)| **c != ' ').unwrap().0;
        let pos = Vec2i32::new(start_x as i32, 0);

        fn explore_face<const T: usize>(p: Vec2i32, orientation: Orientation, map: &Vec<Vec<char>>, faces: &mut[Option<CubeFace<T>>], n: i32) {
            if p.y < 0 || p.y as usize >= map.len() || p.x < 0 || p.x as usize >= map[p.y as usize].len() || map[p.y as usize][p.x as usize] == ' ' {
                return;
            }

            let (face, rotation) = orientation.get_front_face_and_rotation();
            if faces[face].is_some() {
                return;
            }

            faces[face] = Some(CubeFace { 
                map_origin: p.cast(), 
                initial_rotation: rotation,
            });

            explore_face(p - Vec2i32::new(n, 0), orientation.rot_y(), map, faces, n);
            explore_face(p + Vec2i32::new(n, 0), orientation.rot_y_neg(), map, faces, n);
            explore_face(p - Vec2i32::new(0, n), orientation.rot_x_neg() , map, faces, n);
            explore_face(p + Vec2i32::new(0, n), orientation.rot_x() , map, faces, n);
        }

        explore_face(pos, orientation, &map, &mut faces, N as i32);

        Self {
            map,
            faces: faces.into_iter().map(|f| f.unwrap()).collect(),
        }
    }

    fn print_front_face(&self, orientation: Orientation) {
        let face = self.get_front_face(orientation);

        for j in 0..N {
            for i in 0..N {
                print!("{}", face.idx((i as i32, j as i32).into()));
            }
            println!();
        }
    }
}

impl<'a, const N: usize> Cube<N> {
    fn get_front_face(&'a self, orientation: Orientation) -> FaceView<'a, N> {
        let (face, rotation) = orientation.get_front_face_and_rotation();
        FaceView {
            cube: self,
            face,
            rotation
        }
    }
}

#[test]
fn part2() {
    let (map, dirs, _, mut dir) = input();

    const N: usize = 50;

    let mut orientation = Orientation::new();
    let cube: Cube<N> = Cube::new(map);

    let mut pos = Vec2i32::zero();

    while cube.get_front_face(orientation).idx(pos) == '#' {
        pos.x += 1;
    }

    for d in dirs {
        match d {
            Dir::Turn(c) => {
                dir = match c {
                    'L' => dir.rotate_left(),
                    'R' => dir.rotate_right(),
                    _ => panic!(),
                };
            },
            Dir::Walk(steps) => {
                for _ in 0..steps {
                    let mut next = pos + dir;
                    let mut next_orientation = orientation;

                    if next.x < 0 {
                        next_orientation = next_orientation.rot_y();
                        next.x = N as i32 - 1;
                    } else if next.x >= N as i32 {
                        next_orientation = next_orientation.rot_y_neg();
                        next.x = 0;
                    } else if next.y < 0 {
                        next_orientation = next_orientation.rot_x_neg();
                        next.y = N as i32 - 1;
                    } else if next.y >= N as i32 {
                        next_orientation = next_orientation.rot_x();
                        next.y = 0;
                    }

                    if cube.get_front_face(next_orientation).idx(next) == '#' {
                        break;
                    }

                    pos = next;
                    orientation = next_orientation;
                }
            }
        }
    }

    let map_pos = cube.get_front_face(orientation).map_idx(pos);

    let (face, rot) = orientation.get_front_face_and_rotation();
    let rot = (cube.faces[face].initial_rotation + 4 - rot) % 4;
    for _ in 0..rot {
        dir = dir.rotate_left();
    }

    let answer = ((map_pos.y + 1) * 1000) + ((map_pos.x + 1) * 4) + match (dir.x, dir.y) {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => panic!()
    };

    assert_eq!(answer, 0);
}