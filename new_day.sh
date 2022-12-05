#!/usr/bin/env bash
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

if [[ $# -eq 0 ]] ; then
    echo 'Pass a day number ie 5'
    exit 1
fi


cd "$SCRIPT_DAY"
    cargo new --bin "day$1a"
    cat << EOF >"day$1a/src/main.rs"
fn main() {
    let input = include_str!("../input.txt");
    println!("{input}")
}

fn perfom(input: &str) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let test_input = "";
        assert_eq!(perfom(test_input), 0);
    }
}
EOF
    touch "day$1a/input.txt"

    cargo new --bin "day$1b"
    cat << EOF >"day$1b/src/main.rs"
fn main() {
    let input = include_str!("../../day$1a/input.txt");
    println!("{input}")
}

fn perfom(input: &str) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let test_input = "";
        assert_eq!(perfom(test_input), 0);
    }
}
EOF

cd -
