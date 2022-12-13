use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

pub fn main() {
    let input = include_str!("../../day13a/input.txt");
    let response = perform(input);

    // benchmark code should be quiet
    if std::option_env!("BENCH").is_none() {
        println!("response: {} ", response)
    }
}

fn perform(input: &str) -> usize {
    let mut signals = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .into_iter()
        .map(|l| l.parse::<Packet>().unwrap())
        .collect::<Vec<_>>();

    let additional = [
        "[[2]]".parse::<Packet>().unwrap(),
        "[[6]]".parse::<Packet>().unwrap(),
    ];

    signals.extend_from_slice(&additional);
    signals.sort();

    signals
        .iter()
        .enumerate()
        .filter(|(_, a)| additional.contains(a))
        .map(|(i, _)| i + 1)
        .product()
}

#[derive(PartialEq, Clone, Eq)]
pub enum Packet {
    Number(u64),
    Complex(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Packet::*;
        match (self, other) {
            (Number(a), Number(b)) => a.cmp(b),
            (Complex(a), Complex(b)) => match a.iter().cmp(b) {
                r if r != Ordering::Equal => r,
                _ => a.len().cmp(&b.len()),
            },
            (Number(_), Complex(b)) if b.len() == 1 => self.cmp(&b[0]),
            (Number(a), Complex(_)) => Complex(vec![Number(*a)]).cmp(other),
            (Complex(_), Number(_)) => other.cmp(self).reverse(),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

mod parsing {
    use super::*;
    use nom::{
        branch::alt,
        character::complete::{self, char},
        combinator::map,
        multi::separated_list0,
        sequence::delimited,
        IResult,
    };

    fn packet_value(input: &str) -> IResult<&str, Packet> {
        alt((
            map(complete::u64, Packet::Number),
            map(packet, |packet| packet),
        ))(input)
    }

    pub fn packet(input: &str) -> IResult<&str, Packet> {
        delimited(
            char('['),
            separated_list0(char(','), packet_value),
            char(']'),
        )(input)
        .map(|(s, elements)| (s, Packet::Complex(elements)))
    }
}

impl FromStr for Packet {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parsing::packet(s).map(|a| a.1).map_err(|err| {
            eprintln!("{}", err);
            panic!("unable to parse");
        })
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(a) => write!(f, "{}", a),
            Self::Complex(elems) => {
                write!(f, "[")?;
                for e in elems {
                    e.fmt(f)?;
                }
                write!(f, "]")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use Packet::*;
    #[test]
    fn test_deserialization() {
        assert_eq!(
            "[1,1,3,1,1]".parse::<Packet>().unwrap(),
            Packet::Complex(vec![Number(1), Number(1), Number(3), Number(1), Number(1)])
        );
        assert_eq!(
            "[[4,4],4,4,4]".parse::<Packet>().unwrap(),
            Complex(vec![
                Complex(vec![Number(4), Number(4)]),
                Number(4),
                Number(4),
                Number(4)
            ])
        );
        assert_eq!(
            "[[]]".parse::<Packet>().unwrap(),
            Complex(vec![Complex(vec![])])
        );
    }

    #[test]
    fn test_input() {
        let test_input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
        assert_eq!(perform(test_input), 140);
    }
}
