const PLAYERS: usize = 424;
const LAST_SCORE: usize = 71482;

#[derive(Copy, Clone)]
struct Node {
    next: usize,
    previous: usize,
    value: usize,
}

fn run_linked(players: usize, until: usize) -> usize {
    let mut circle = Vec::with_capacity(until);
    circle.push(Node { next: 0, previous: 0, value: 0});

    let mut scores = vec![0; players];
    let mut current_player_idx = 0;
    
    let mut current_node_idx = 0;

    for marble in 1 ..= until {
        if marble % 23 == 0 {
            // add marble to score
            scores[current_player_idx] += marble;

            // find marble to remove
            let mut to_remove_idx = current_node_idx;
            for _ in 0..7 {
                to_remove_idx = circle[to_remove_idx].previous;
            }

            // set new current, which is after removed
            current_node_idx = circle[to_remove_idx].next;

            // add marble to remove to score
            scores[current_player_idx] += circle[to_remove_idx].value;

            // remove the to_remove marble from the linked list
            let to_remove = circle[to_remove_idx];
            circle[to_remove.previous].next = to_remove.next;
            circle[to_remove.next].previous = to_remove.previous;

            // if the removed marble is the last in the array, just pop it off
            // otherwise replace the marble's location with the last marble in the array
            // and fixed up the linked list indices
            if to_remove_idx == circle.len() - 1 {
                circle.pop().unwrap();
            } else {
                let last = circle.pop().unwrap();
                circle[to_remove_idx] = last;
                circle[last.previous].next = to_remove_idx;
                circle[last.next].previous = to_remove_idx;
            }
        } else {
            let next_idx = circle[current_node_idx].next;
            let new_idx = circle.len();

            let new_node = Node {
                next: circle[next_idx].next,
                previous: next_idx,
                value: marble
            };

            circle[new_node.next].previous = new_idx;
            circle[next_idx].next = new_idx;
            circle.push(new_node);

            current_node_idx = new_idx;
        }

        current_player_idx += 1;
        current_player_idx %= scores.len();
    }

    scores.into_iter().max().unwrap()
}

#[test]
fn part1() {
    let answer = run_linked(PLAYERS, LAST_SCORE);
    assert_eq!(408679, answer);
}

#[test]
fn part2() {
    let answer = run_linked(PLAYERS, LAST_SCORE * 100);
    assert_eq!(3443939356, answer);
}