use itertools::Itertools;

fn main() {
    println!(
        "{}",
        include_bytes!("../../day3a/input.txt")
            .split(|b| *b == b'\n')
            .chunks(3)
            .into_iter()
            .map(|mut l| {
                let this = l.next();
                let other = l.next();
                let last = l.next();
                let offending = this.into_iter().flatten().find(|e| {
                    other.clone().into_iter().flatten().contains(e)
                        && last.clone().into_iter().flatten().contains(e)
                });
                match offending {
                    Some(n) => calculate_priority(*n) as usize,
                    None => 0,
                }
            })
            .sum::<usize>()
    );
}

fn calculate_priority(l: u8) -> u8 {
    if l < b'a' {
        //upercase
        l - b'A' + 27
    } else {
        l - b'a' + 1
    }
}
