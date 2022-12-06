use std::{collections::HashSet, hash::Hash};

use bounded_vec_deque::BoundedVecDeque;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", perfom(input))
}

fn perfom(input: &str) -> usize {
    let mut buf = BoundedVecDeque::new(4);
    let mut len = 0;
    for c in input.chars() {
        len += 1;
        buf.push_front(c);
        if buf.len() == 4 && has_unique_elements(buf.iter()) {
            return len;
        }
    }
    unreachable!()
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let test_input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(perfom(test_input), 7);
    }
}
