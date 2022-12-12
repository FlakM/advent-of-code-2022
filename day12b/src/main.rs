use petgraph::algo::dijkstra;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::collections::HashMap;

pub fn main() {
    let input = include_str!("../../day12a/input.txt");
    let response = perform(input);

    // benchmark code should be quiet
    if std::option_env!("BENCH").is_none() {
        println!("response: {} ", response)
    }
}

fn is_valid_step(curr: u8, c: u8) -> bool {
    (curr == b'S' && c <= b'b') || (c == b'E' && curr >= b'y') || (c != b'E' && c <= curr + 1)
}

fn get_neighbours((y, x): (usize, usize), chars: &[&[u8]]) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];
    let curr = chars[y][x];
    if let Some(right) = chars
        .get(y)
        .and_then(|y| y.get(x + 1))
        .filter(|c| is_valid_step(curr, **c))
        .map(|_| (y, x + 1))
    {
        neighbours.push(right)
    };
    if x > 0 {
        if let Some(left) = chars
            .get(y)
            .and_then(|y| y.get(x - 1))
            .filter(|c| is_valid_step(curr, **c))
            .map(|_| (y, x - 1))
        {
            neighbours.push(left)
        };
    }
    if let Some(down) = chars
        .get(y + 1)
        .and_then(|y| y.get(x))
        .filter(|c| is_valid_step(curr, **c))
        .map(|_| (y + 1, x))
    {
        neighbours.push(down)
    };

    if y > 0 {
        if let Some(up) = chars
            .get(y - 1)
            .and_then(|y| y.get(x))
            .filter(|c| is_valid_step(curr, **c))
            .map(|_| (y - 1, x))
        {
            neighbours.push(up)
        };
    }

    neighbours
}

fn perform(input: &str) -> usize {
    let chars: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();

    let mut nodes: HashMap<(usize, usize), (NodeIndex, u8)> = HashMap::default();

    let mut end = None;
    let mut possible_as = vec![];

    let mut g = Graph::<(usize, usize, char), ()>::default();

    for y in 0..chars.len() {
        for x in 0..chars[0].len() {
            let curr = chars[y][x];
            let index = *nodes
                .entry((y, x))
                .or_insert((g.add_node((y, x, curr as char)), curr));
            match curr {
                b'a' | b'S' => {
                    possible_as.push(index.0);
                }
                b'E' => {
                    end = Some(index.0);
                }
                _ => {}
            }
            //if curr != b'a' {
                for (yn, xn) in get_neighbours((y, x), &chars) {
                    let n_index = *nodes
                        .entry((yn, xn))
                        .or_insert((g.add_node((yn, xn, chars[yn][xn] as char)), chars[yn][xn]));
                    g.update_edge(index.0, n_index.0, ());
                }
            //}
        }
    }

    //println!("{:?}", g);

    possible_as
        .into_iter()
        .map(|idx| {
            let node_map = dijkstra(&g, idx, Some(end.unwrap()), |_| 1);
            node_map
                .get(&end.unwrap())
                .filter(|p| **p != 0)
                .map(|a| *a)
        })
        .flatten()
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steps() {
        assert!(is_valid_step(b'a', b'b'));
        assert!(is_valid_step(b'a', b'a'));
        assert!(is_valid_step(b'y', b'E'));
        assert!(!is_valid_step(b'x', b'E'));
    }

    #[test]
    fn test_input() {
        let test_input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        assert_eq!(perform(test_input), 29);
    }
}
