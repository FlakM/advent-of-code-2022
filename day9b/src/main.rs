use std::collections::HashMap;

pub fn main() {
    let input = include_str!("../../day9a/input.txt");
    let response = perform(input);

    // benchmark code should be quiet
    if std::option_env!("BENCH").is_none() {
        println!("response: {} ", response)
    }
}

fn move_pos(direction: u8, position: (isize, isize)) -> (isize, isize) {
    let (x, y) = position;
    match direction {
        b'U' => (x, y + 1),
        b'D' => (x, y - 1),
        b'R' => (x + 1, y),
        b'L' => (x - 1, y),
        _ => panic!("invalid direction: {direction}"),
    }
}

fn move_shadow(head: (isize, isize), tail: (isize, isize)) -> (isize, isize) {
    let (hx, hy) = head;
    let (tx, ty) = tail;
    if tx.abs_diff(hx).max(ty.abs_diff(hy)) > 1 {
        (tx + (hx - tx).signum(), ty + (hy - ty).signum())
    } else {
        (tx, ty)
    }
}

fn perform(input: &str) -> usize {
    let mut seen: HashMap<(isize, isize), bool> = HashMap::default();
    let mut knots = [(0, 0); 10];
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split_at(2))
        .map(|(direction, val)| (direction.as_bytes()[0], val.parse::<u8>().unwrap()))
        .for_each(|(direction, val)| {
            for _ in 0..val {
                knots[0] = move_pos(direction, knots[0]);
                for knot in 1..10 {
                    knots[knot] = move_shadow(knots[knot - 1], knots[knot]);
                }

                seen.entry(knots[9].clone()).or_insert(true);
            }
        });
    seen.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_moving_behaviour() {
        assert_eq!(move_shadow((0, 0), (0, 0)), (0, 0)); // covering
        assert_eq!(move_shadow((1, 0), (0, 0)), (0, 0)); // touching
        assert_eq!(move_shadow((1, 2), (0, 0)), (1, 1)); // on diagonal up right
        assert_eq!(move_shadow((0, 0), (1, 2)), (0, 1)); // move down left on diagonal
        assert_eq!(move_shadow((0, 0), (2, 0)), (1, 0)); // move left
        assert_eq!(move_shadow((0, 0), (-2, 0)), (-1, 0)); // move right
    }

    #[test]
    fn test_input() {
        let test_input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(perform(test_input), 10);
    }

    #[test]
    fn test_longer_input() {
        let test_input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(perform(test_input), 36);
    }
}
