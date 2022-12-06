use std::collections::{HashMap, VecDeque};

use regex::Regex;

pub fn main() {
    let input = include_str!("../../day5a/input.txt");
    let response = perform(input);

    // benchmark code should be quiet
    if std::option_env!("BENCH").is_none() {
        println!("response: {} ", response)
    }
}

fn perform(input: &str) -> String {
    let (crates, moves) = input.split_once("\n\n").unwrap();

    let mut crates = crates
        .lines()
        .take_while(|l| l.contains('['))
        .flat_map(|l| {
            let chars = l.chars().collect::<Vec<_>>();
            chars
                .chunks(4)
                .enumerate()
                .map(|(i, c)| (i + 1, c[1]))
                .collect::<Vec<_>>()
        })
        .fold(HashMap::new(), |mut acc, (i, c)| {
            if c != ' ' {
                let entry: &mut VecDeque<char> = acc.entry(i).or_default();
                entry.push_back(c)
            }
            acc
        });

    let regex: Regex = Regex::new(r"([0-9]+)").unwrap();
    moves
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut moves = regex.find_iter(l).map(|mat| mat.as_str());
            (
                moves.next().unwrap().parse::<usize>().unwrap(),
                moves.next().unwrap().parse::<usize>().unwrap(),
                moves.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .for_each(|(count, from, to)| {
            let drained = {
                let from_crate = crates.get_mut(&from).unwrap();
                from_crate.drain(0..count).rev().collect::<Vec<_>>()
            };
            for elem in drained {
                crates.get_mut(&to).unwrap().push_front(elem);
            }
        });

    let mut s = String::new();
    for i in 1..=crates.len() {
        s.push(crates[&i][0]);
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!(perform(input), "MCD".to_string())
    }
}
