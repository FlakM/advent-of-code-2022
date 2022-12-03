fn main() {
    println!(
        "{}",
        include_bytes!("../../day3a/input.txt")
            .split(|b| *b == b'\n')
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|l| {
                let common = l[0].iter().find(|e| l[1].contains(e) && l[2].contains(e));
                match common {
                    Some(n) => calculate_priority(*n) as usize,
                    None => panic!("missing common element"),
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
