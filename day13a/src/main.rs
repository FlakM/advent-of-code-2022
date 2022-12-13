use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

pub fn main() {
    let input = include_str!("../input.txt");
    let response = perform(input);

    // benchmark code should be quiet
    if std::option_env!("BENCH").is_none() {
        println!("response: {} ", response)
    }
}
// 283 is too low
//
fn perform(input: &str) -> usize {
    input
        .split("\n\n")
        .into_iter()
        .enumerate()
        .map(|(i, e)| {
            let (left, right) = e.split_once('\n').unwrap();
            let left = left.parse::<Packet>().unwrap();
            let right = right.parse::<Packet>().unwrap();
            if left.is_lower(&right) {
                i + 1
            } else {
                0
            }
        })
        .sum()
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum Packet {
    Number(u64),
    // todo add pair utility method here
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

impl Packet {
    // returns true is a < b
    fn is_lower(&self, b: &Packet) -> bool {
        self.cmp(b).is_lt()
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
            Complex(vec![Complex(vec![Number(4), Number(4)]), Number(4), Number(4), Number(4)])
        );
        assert_eq!(
            "[[]]".parse::<Packet>().unwrap(),
            Complex(vec![Complex(vec![])])
        );
    }

    #[test]
    fn repr() {
        let left: Packet = "[[2,3,4]]".parse().unwrap();
        let right: Packet = "[4]".parse().unwrap();
        assert!(!right.is_lower(&left));
    }

    #[test]
    fn test_comparing() {
        let left: Packet = "[1,2,3]".parse().unwrap();
        let right: Packet = "[1,2,3]".parse().unwrap();
        assert!(!left.is_lower(&right));

        let left: Packet = "[1,2,3,[4]]".parse().unwrap();
        let right: Packet = "[1,2,3,[4]]".parse().unwrap();
        assert!(!left.is_lower(&right));

        let left: Packet = "[1,1,3,1,1]".parse().unwrap();
        let right: Packet = "[1,1,5,1,1]".parse().unwrap();
        assert!(left.is_lower(&right));

        let left: Packet = "[[1],[2,3,4]]".parse().unwrap();
        let right: Packet = "[[1],4]".parse().unwrap();
        assert!(left.is_lower(&right));

        let left: Packet = "[9]".parse().unwrap();
        let right: Packet = "[[8,7,6]]".parse().unwrap();
        assert!(!left.is_lower(&right));

        let left: Packet = "[8]".parse().unwrap();
        let right: Packet = "[[8,7,6]]".parse().unwrap();
        assert!(left.is_lower(&right));

        let left: Packet = "[[8,7,6]]".parse().unwrap();
        let right: Packet = "[8]".parse().unwrap();
        assert!(!left.is_lower(&right));

        let left: Packet = "[[4,4],4,4]".parse().unwrap();
        let right: Packet = "[[4,4],4,4,4]".parse().unwrap();
        assert!(left.is_lower(&right));

        let left: Packet = "[7,7,7,7]".parse().unwrap();
        let right: Packet = "[7,7,7]".parse().unwrap();
        assert!(!left.is_lower(&right));

        let left: Packet = "[]".parse().unwrap();
        let right: Packet = "[3]".parse().unwrap();
        assert!(left.is_lower(&right));

        let left: Packet = "[[[]]]".parse().unwrap();
        let right: Packet = "[[]]".parse().unwrap();
        assert!(!left.is_lower(&right));

        let left: Packet = "[[]]".parse().unwrap();
        let right: Packet = "[[[]]]".parse().unwrap();
        assert!(left.is_lower(&right));

        let left: Packet = "[1,[2,[3,[4,[5,6,7]]]],8,9]".parse().unwrap();
        let right: Packet = "[1,[2,[3,[4,[5,6,0]]]],8,9]".parse().unwrap();
        assert!(!left.is_lower(&right));
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
        assert_eq!(perform(test_input), 13);
    }
}
