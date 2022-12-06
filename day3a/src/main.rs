pub fn main() {
    let response = include_bytes!("../input.txt")
        .split(|b| *b == b'\n')
        .map(|l| {
            let h = l.len() / 2;
            let this = &l[0..h];
            let other = &l[h..l.len()];
            let offending = this.iter().find(|e| other.contains(e));
            match offending {
                Some(n) => calculate_priority(*n) as usize,
                None => 0,
            }
        })
        .sum::<usize>();

    // benchmark code should be quiet
    if std::option_env!("BENCH").is_none() {
        println!("response: {} ", response)
    }
}

fn calculate_priority(l: u8) -> u8 {
    if l < b'a' {
        //upercase
        l - b'A' + 27
    } else {
        l - b'a' + 1
    }
}
