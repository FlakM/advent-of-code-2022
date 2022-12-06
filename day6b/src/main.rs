pub fn main() {
    let input = include_bytes!("../../day6a/input.txt");
    let response = perfom(input);

    // benchmark code should be quiet
    if std::option_env!("BENCH").is_none() {
        println!("response: {} ", response)
    }
}

fn perfom(input: &[u8]) -> usize {
    input
        .windows(14)
        .position(|slice| {
            (1..slice.len()).all(|i| !slice[i..].contains(&slice[i - 1]))
                && (slice.len() - 4..slice.len()).all(|i| !slice[i..].contains(&slice[i - 1]))
        })
        .unwrap()
        + 14
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let test_input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes();
        assert_eq!(perfom(test_input), 19);
    }
}
