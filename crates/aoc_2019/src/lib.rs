#![cfg(test)]

use intcode::*;

mod intcode;

mod day2 {
    use crate::IntCode;

    #[test]
    fn part1() {
        let mut int_code = IntCode::from_file("inputs/day2.txt");

        let mem = int_code.mem_mut();
        mem[1] = 12;
        mem[2] = 2;
        int_code.run();

        let answer = int_code.mem()[0];
        assert_eq!(answer, 3931283);
    }

    const TARGET: i64 = 19690720;
    #[test]
    fn part2() {
        let mut int_code = IntCode::from_file("inputs/day2.txt");

        let mut answer = 0;
        'outer: for i in 0..100 {
            for j in 0..100 {
                int_code.reset();
                let mem = int_code.mem_mut();
                mem[1] = i;
                mem[2] = j;
                int_code.run();
                if int_code.mem()[0] == TARGET {
                    answer = 100 * i + j;
                    break 'outer;
                }
            }
        }

        assert_eq!(answer, 6979);
    }
}

mod day5 {
    use crate::{IntCode, intcode::FnIo};

    use aoc_common::file_string;

    fn run(id: i64) -> i64 {
        let mut answer = 0;

        let mut read = || id;
        let mut write= |val| {
            answer = val;
        };
        let mut io = FnIo::new(&mut read, &mut write);

        let mut int_code = IntCode::from_file("inputs/day5.txt").with_io(&mut io);
        int_code.run();

        answer
    }

    #[test]
    fn part1() {
        let answer = run(1);
        assert_eq!(answer, 8332629);
    }

    #[test]
    fn part2() {
        let answer = run(5);
        assert_eq!(answer, 8805067);
    }
}