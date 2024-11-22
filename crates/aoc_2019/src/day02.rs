use intcode::IntCode;

#[test]
fn part1() {
    let mut int_code = IntCode::from_file("inputs/day02.txt");

    let mem = int_code.mem_mut();
    mem[1] = 12;
    mem[2] = 2;
    int_code.run_to_halt().unwrap();

    let answer = int_code.mem()[0];
    assert_eq!(answer, 3931283);
}

const TARGET: i64 = 19690720;
#[test]
fn part2() {
    let mut int_code = IntCode::from_file("inputs/day02.txt");

    let mut answer = 0;
    'outer: for i in 0..100 {
        for j in 0..100 {
            int_code.reset();
            let mem = int_code.mem_mut();
            mem[1] = i;
            mem[2] = j;
            int_code.run_to_halt().unwrap();
            if int_code.mem()[0] == TARGET {
                answer = 100 * i + j;
                break 'outer;
            }
        }
    }

    assert_eq!(answer, 6979);
}