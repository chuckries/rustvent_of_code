const ROW: usize = 2947;
const COL: usize = 3029;

const START: usize = 20151125;
const MULTI: usize = 252533;
const DIV: usize = 33554393;

#[test]
fn part1() {
    let base_row = ROW + COL - 1;
    let base_num = (base_row * (base_row - 1)) / 2 + 1;
    let seq_num = base_num + COL - 1;

    let mut seq = START;
    for _ in 1..seq_num {
        seq *= MULTI;
        seq %= DIV;
    }

    assert_eq!(seq, 19980801);
}