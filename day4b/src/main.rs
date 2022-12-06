pub fn main() {
    let response = count_overlapping(include_str!("../../day4a/input.txt"));
    // benchmark code should be quiet
    if std::option_env!("BENCH").is_none() {
        println!("response: {} ", response)
    }
}

fn convert(a: (&str, &str)) -> (usize, usize) {
    let (a, b) = a;
    (a.trim().parse().unwrap(), b.trim().parse().unwrap())
}

fn count_overlapping(s: &str) -> usize {
    s.split('\n')
        .filter(|l| !l.is_empty())
        .filter(|l| {
            let (first, second) = l.split_once(',').unwrap();
            let (a, b) = convert(first.split_once('-').unwrap());
            let (c, d) = convert(second.split_once('-').unwrap());
            b >= c && d >= a
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let input = "2-4,6-8
       2-3,4-5
       5-7,7-9
       2-8,3-7
       6-6,4-6
       2-6,4-8";
        assert_eq!(count_overlapping(input), 4);
    }
}
