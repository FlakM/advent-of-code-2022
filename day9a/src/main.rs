use std::collections::HashMap;

pub fn main() {
    let input = include_str!("../input.txt");
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

fn move_shadow(direction: u8, head: (isize, isize), tail: (isize, isize)) -> (isize, isize) {
    let (x1, y1) = head;
    let (x2, y2) = tail;
    
    if (x1-x2).abs()< 2 && (y1-y2).abs()< 2 {
        return tail;
    };

    match direction {
        b'U' => (x1, y2 + 1),
        b'D' => (x1, y2 - 1),
        b'R' => (x2 + 1, y1),
        b'L' => (x2 - 1, y1),
        _ => panic!("invalid direction: {direction}"),
    }
}

fn perform(input: &str) -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut seen: HashMap<(isize, isize), bool> = HashMap::default();
    //seen.entry((0, 0)).or_insert(true);
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split_at(2))
        .map(|(direction, val)| {
            (direction.as_bytes()[0], val.parse::<u8>().unwrap())
        })
        .for_each(|(direction, val)| {
            for _ in 0..val {
                head = move_pos(direction, head);
                tail = move_shadow(direction, head, tail);
                seen.entry(tail.clone()).or_insert(true);
            }
        });

    seen.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_moving_behaviour() {
        assert_eq!(move_shadow(b'R', (0, 0), (0, 0)), (0, 0)); // covering
        assert_eq!(move_shadow(b'R', (1, 0), (0, 0)), (0, 0)); // touching
        assert_eq!(move_shadow(b'U', (1, 2), (0, 0)), (1, 1)); // on diagonal up right
        assert_eq!(move_shadow(b'D', (0, 0), (1, 2)), (0, 1)); // move down left on diagonal
        assert_eq!(move_shadow(b'L', (0, 0), (2, 0)), (1, 0)); // move left
        assert_eq!(move_shadow(b'R', (0, 0), (-2, 0)), (-1, 0)); // move right
    }

    #[test]
    fn check_pos_parsing() {
        assert_eq!(move_pos(b'D', (0, 0)), (0, -1));
        assert_eq!(move_pos(b'U', (0, 0)), (0, 1));
        assert_eq!(move_pos(b'R', (0, 0)), (1, 0));
        assert_eq!(move_pos(b'L', (0, 0)), (-1, 0));
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
        assert_eq!(perform(test_input), 13);
    }
}
