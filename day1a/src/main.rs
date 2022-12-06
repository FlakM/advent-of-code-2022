pub fn main() {
    let input = include_str!("../input.txt");
    let response = find_heavy_loader(input);

    // benchmark code should be quiet
    if std::option_env!("BENCH").is_none() {
        println!("response: {} ", response)
    }
}

fn find_heavy_loader(s: &str) -> usize {
    s.split("\n\n")
        .map(|s| {
            s.split('\n')
                .filter(|s| !s.is_empty())
                .map(|l| l.trim().parse::<usize>().unwrap())
                .sum()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(find_heavy_loader(input), 24000);
    }
}
