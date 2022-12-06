use bounded_vec_deque::BoundedVecDeque;
use std::{collections::HashSet, hash::Hash};

pub fn main() {
    let input = include_str!("../../day6a/input.txt");
    let response =  perfom(input);

    // benchmark code should be quiet
    if std::option_env!("BENCH").is_none() {
        println!("response: {} ", response)
    }
}

fn perfom(input: &str) -> usize {
    let mut buf = BoundedVecDeque::new(4);
    let mut begin_buf = BoundedVecDeque::new(14);
    let mut len = 0;
    for c in input.chars() {
        len += 1;
        buf.push_front(c);
        begin_buf.push_front(c);
        if begin_buf.len() == 14
            && has_unique_elements(begin_buf.iter())
            && buf.len() == 4
            && has_unique_elements(buf.iter())
        {
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
        assert_eq!(perfom(test_input), 19);
    }
}
