use std::{cell::RefCell, rc::Rc};

pub fn main() {
    let input = include_str!("../input.txt");
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
                .map(|a| a.trim().parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let operation_txt = l.next().unwrap()[19..].to_string();
            let operation = Box::new(move |a| {
                let expr: meval::Expr = operation_txt.parse().unwrap();
                let func = expr.bind("old").unwrap();
                func(a as f64) as usize
            });

            let test = l.next().unwrap()[21..].parse::<usize>().unwrap();
            let if_true = l.next().unwrap()[29..].parse::<usize>().unwrap();
            let if_false = l.next().unwrap()[30..].parse::<usize>().unwrap();

            Rc::new(RefCell::new(Monkey {
                starting_items: starting,
                operation,
                test: Box::new(move |a| a % test == 0),
                monkey_if_true: if_true,
                monkey_if_false: if_false,
                inspected_counter: 0,
            }))
        })
        .collect()
}

fn perform(input: &str) -> usize {
    let  monkeys = parse_monkeys(input);

    (0..20).for_each(|_| {
        for m in monkeys.iter() {
            let c = m.clone();
            let mut current = c.borrow_mut();
            while let Some(mut item) = current.starting_items.pop() {
                current.inspected_counter +=1;
                item = (current.operation)(item);
                item /= 3;
                if (current.test)(item) {
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

    let mut counts = monkeys.iter().map(|m| m.borrow_mut().inspected_counter).collect::<Vec<usize>>();
    counts.sort();
    counts[counts.len()-1] * counts[counts.len()-2]
}

struct Monkey {
    starting_items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> bool>,
    monkey_if_true: usize,
    monkey_if_false: usize,
    inspected_counter: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_parsing() {
        let input = "Monkey 0:
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
    If false: throw to monkey 0";
        let monkeys = parse_monkeys(input);
        assert_eq!(monkeys.len(), 2);
        let binding = monkeys[0].clone();
        let head = binding.borrow_mut();
        assert_eq!(head.starting_items, vec![79, 98]);
        assert_eq!((head.operation)(1), 19);
        assert_eq!((head.test)(23), true);
        assert_eq!((head.test)(2), false);
        assert_eq!(head.monkey_if_true, 2);
        assert_eq!(head.monkey_if_false, 3);
    }

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
        assert_eq!(perform(test_input), 10605);
    }
}
