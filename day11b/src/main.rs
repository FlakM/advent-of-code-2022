use std::{cell::RefCell, rc::Rc};
use num::BigUint;

pub fn main() {
    let input = include_str!("../../day11a/input.txt");
    let response = perform(input);

    // benchmark code should be quiet
    if std::option_env!("BENCH").is_none() {
        println!("response: {} ", response)
    }
}


fn parse_monkeys(input: &str) -> Vec<Rc<RefCell<Monkey>>> {
    input
        .split("\n\n")
        .map(|i| {
            let mut l = i.lines();
            let _name = l.next();
            let starting = l.next().unwrap()[18..]
                .split(',')
                .map(|a| a.trim().parse::<BigUint>().unwrap())
                .collect::<Vec<_>>();
            let operation_txt = l.next().unwrap()[19..].to_string();
            let operation = Box::new(move |a| expr(&operation_txt, a));

            let test = l.next().unwrap()[21..].parse::<BigUint>().unwrap();
            let if_true = l.next().unwrap()[29..].parse::<usize>().unwrap();
            let if_false = l.next().unwrap()[30..].parse::<usize>().unwrap();

            Rc::new(RefCell::new(Monkey {
                starting_items: starting,
                operation,
                test_val: test.clone(),
                test: Box::new(move |a| a % &test == BigUint::from(0 as u128) ),
                monkey_if_true: if_true,
                monkey_if_false: if_false,
                inspected_counter: 0,
            }))
        })
        .collect()
}

fn perform(input: &str) -> usize {
    let monkeys = parse_monkeys(input);

    let  modulus: BigUint = monkeys
        .iter()
        .map(|m| m.borrow_mut().test_val.clone())
        .product();


    (0..10_000).for_each(|_| {
        for m in monkeys.iter() {
            let c = m.clone();
            let mut current = c.borrow_mut();
            while let Some(mut item) = current.starting_items.pop().take() {
                current.inspected_counter += 1;
                item = (current.operation)(item) % &modulus;
                if (current.test)(&item) {
                    monkeys
                        .get(current.monkey_if_true)
                        .unwrap()
                        .clone()
                        .borrow_mut()
                        .starting_items
                        .push(item);
                } else {
                    monkeys
                        .get(current.monkey_if_false)
                        .unwrap()
                        .clone()
                        .borrow_mut()
                        .starting_items
                        .push(item);
                }
            }
        }
    });

    let mut counts = monkeys
        .iter()
        .map(|m| m.borrow_mut().inspected_counter)
        .collect::<Vec<usize>>();
    counts.sort();
    counts.reverse();
    counts[0] * counts[1]
}

struct Monkey {
    starting_items: Vec<BigUint>,
    operation: Box<dyn Fn(BigUint) -> BigUint>,
    test_val: BigUint,
    test: Box<dyn Fn(&BigUint) -> bool>,
    monkey_if_true: usize,
    monkey_if_false: usize,
    inspected_counter: usize,
}

fn part_2_num(s:&str, a: BigUint) -> BigUint {
    match s {
        "old" => a,
        other => other.parse::<BigUint>().unwrap()
    }
}
fn expr(s:&str, num: BigUint) -> BigUint {
    let mut parts = s.split_whitespace();
    let a = part_2_num(parts.next().unwrap(), num.clone());
    let sign = parts.next().unwrap();
    let b = part_2_num(parts.next().unwrap(), num);

    match sign {
        "+" => a + b,
        "*" => a * b,
        "/" => a / b,
        "-" => a - b,
        _ => unreachable!()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let test_input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        assert_eq!(perform(test_input), 2713310158);
    }
}
