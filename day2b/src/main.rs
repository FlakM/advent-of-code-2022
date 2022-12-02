use std::cmp::Ordering;

#[derive(Clone, Copy)]
enum RpS {
    Rocks,
    Scissors,
    Paper,
}

enum Strategy {
    Lose,
    Draw,
    Win,
}

impl Strategy {
    fn pick_for_strategy(&self, p: &RpS) -> RpS {
        use RpS::*;
        let all = [Rocks, Scissors, Paper];
        match self {
            Strategy::Lose => all.into_iter().find(|a| a.cmp(p).is_lt()).unwrap(),
            Strategy::Draw => *p,
            Strategy::Win => all.into_iter().find(|a| a.cmp(p).is_gt()).unwrap(),
        }
    }
}

static DRAW_BONUS: usize = 3;
static WIN_BONUS: usize = 6;

impl RpS {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            // if self wins it Greater
            RpS::Rocks => match other {
                RpS::Rocks => Ordering::Equal,
                RpS::Scissors => Ordering::Greater,
                RpS::Paper => Ordering::Less,
            },

            RpS::Scissors => match other {
                RpS::Rocks => Ordering::Less,
                RpS::Scissors => Ordering::Equal,
                RpS::Paper => Ordering::Greater,
            },

            RpS::Paper => match other {
                RpS::Rocks => Ordering::Greater,
                RpS::Scissors => Ordering::Less,
                RpS::Paper => Ordering::Equal,
            },
        }
    }
    fn get_self_weight(&self) -> usize {
        match self {
            RpS::Rocks => 1,
            RpS::Paper => 2,
            RpS::Scissors => 3,
        }
    }

    fn calculate(&self, other: RpS) -> usize {
        let bonus = match self.cmp(&other) {
            Ordering::Less => 0,
            Ordering::Equal => DRAW_BONUS,
            Ordering::Greater => WIN_BONUS,
        };
        bonus + self.get_self_weight()
    }
}

fn parse_from_opponent(s: &str) -> RpS {
    use RpS::*;
    match s {
        "A" => Rocks,
        "B" => Paper,
        "C" => Scissors,
        _ => panic!("invalid option for opponent [{}]", s),
    }
}

fn parse_from_our(s: &str) -> Strategy {
    use Strategy::*;
    match s {
        "X" => Lose,
        "Y" => Draw,
        "Z" => Win,
        _ => panic!("invalid option for our [{}]", s),
    }
}

fn calculate_score(s: &str) -> usize {
    s.split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.split(' ').filter(|a| !a.is_empty()))
        .map(|mut a| {
            (
                parse_from_opponent(a.next().unwrap()),
                parse_from_our(a.next().unwrap()),
            )
        })
        .map(|(t, s)| s.pick_for_strategy(&t).calculate(t))
        .sum()
}

fn main() {
    let input = include_str!("../../day2a/input.txt");
    println!("Result: {}", calculate_score(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "A Y
B X
C Z";
        assert_eq!(calculate_score(input), 12)
    }
}
