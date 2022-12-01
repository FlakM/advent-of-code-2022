fn main() {
    let input = include_str!("../../day1a/input.txt");
    println!("response: {} ", find_top_3_heavy_loaders(input))
}

fn find_top_3_heavy_loaders(s: &str) -> usize {
    let mut elfs: Vec<usize> = s
        .split("\n\n")
        .map(|s| {
            s.split('\n')
                .filter(|s| !s.is_empty())
                .map(|l| l.trim().parse::<usize>().unwrap())
                .sum()
        })
        .collect();
    elfs.sort_by(|a, b| b.cmp(a));
    elfs.iter().take(3).sum()
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
        assert_eq!(find_top_3_heavy_loaders(input), 45000);
    }
}
