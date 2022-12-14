use std::{collections::VecDeque, str::FromStr};

struct Monkey {
    items: VecDeque<u64>,
    test: u64,
    operation: Box<dyn Fn(u64) -> u64>,
    if_true_monkey: usize,
    if_false_monkey: usize,
    items_inspected: u64,
}

fn get_operation<'a>(line: &'a str) -> Box<dyn Fn(u64) -> u64> {
    let mut parts = line.split_whitespace();
    let lhs = parts.next().unwrap();
    let op = parts.next().unwrap();
    let rhs = parts.next().unwrap();

    match op {
        "+" => match (lhs, rhs) {
            ("old", "old") => Box::new(move |old| old + old),
            ("old", _) => {
                let rhs = rhs.parse::<u64>().unwrap();
                Box::new(move |old| old + rhs)
            }
            _ => unreachable!(),
        },
        "*" => match (lhs, rhs) {
            ("old", "old") => Box::new(move |old| old * old),
            ("old", _) => {
                let rhs = rhs.parse::<u64>().unwrap();
                Box::new(move |old| old * rhs)
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();

        let items: VecDeque<u64> = lines[1]
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|i| i.parse().unwrap())
            .collect();

        let operation = get_operation(&lines[2].split(": ").nth(1).unwrap()[6..]);

        let test: u64 = lines[3].split(": ").nth(1).unwrap()[13..].parse().unwrap();
        let if_true_monkey: usize = lines[4].split(": ").nth(1).unwrap()[16..].parse().unwrap();
        let if_false_monkey: usize = lines[5].split(": ").nth(1).unwrap()[16..].parse().unwrap();

        Ok(Monkey {
            items,
            test,
            operation,
            if_true_monkey,
            if_false_monkey,
            items_inspected: 0,
        })
    }
}

fn part1(input: &'static str) -> u64 {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|line| Monkey::from_str(line).unwrap())
        .collect();
    let len = monkeys.len();

    for _ in 0..20 {
        for i in 0..len {
            loop {
                let item = match monkeys[i].items.pop_front() {
                    Some(item) => item,
                    None => break,
                };
                let item = (monkeys[i].operation)(item);
                let item = item / 3;

                let idx = if item % monkeys[i].test == 0 {
                    monkeys[i].if_true_monkey
                } else {
                    monkeys[i].if_false_monkey
                };

                monkeys[idx].items.push_back(item);
                monkeys[i].items_inspected += 1;
            }
        }
    }

    monkeys.sort_by(|a, b| b.items_inspected.cmp(&a.items_inspected));
    monkeys[..2].iter().map(|m| m.items_inspected).product()
}

fn part2(input: &'static str) -> u64 {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|line| Monkey::from_str(line).unwrap())
        .collect();
    let len = monkeys.len();

    // take the lcm of all the divisors
    // I hate that aoc made me reverse engineer this
    let absolute_limit: u64 = monkeys.iter().map(|m| m.test).product();

    for _ in 0..10_000 {
        for i in 0..len {
            loop {
                let item = match monkeys[i].items.pop_front() {
                    Some(item) => item,
                    None => break,
                };
                let item = (monkeys[i].operation)(item);
                let item = item % absolute_limit;

                let idx = if item % monkeys[i].test == 0 {
                    monkeys[i].if_true_monkey
                } else {
                    monkeys[i].if_false_monkey
                };

                monkeys[idx].items.push_back(item);
                monkeys[i].items_inspected += 1;
            }
        }
    }

    monkeys.sort_by(|a, b| b.items_inspected.cmp(&a.items_inspected));
    monkeys[..2].iter().map(|m| m.items_inspected).product()
}

fn main() {
    let input = include_str!("day11-input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let input = r"Monkey 0:
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
    If false: throw to monkey 1
";
        assert_eq!(10605, part1(input));
        assert_eq!(2713310158, part2(input));
    }
}
