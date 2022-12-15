use std::{fmt::Debug, iter::Peekable, str::FromStr, vec};
#[derive(PartialEq, Eq, Clone)]
enum Term {
    Item(usize),
    List(Vec<Term>),
}

impl Term {
    fn parse_helper(chars: &mut Peekable<impl Iterator<Item = char>>) -> Option<Self> {
        let c = chars.next();
        match c {
            Some('[') => {
                let mut items = Vec::new();
                while let Some(term) = Self::parse_helper(chars) {
                    items.push(term);
                    if let Some(']') = chars.next() {
                        break;
                    }
                }
                Some(Term::List(items))
            }
            Some(c) if c.is_ascii_digit() => {
                let mut num = c.to_string();
                while let Some(&c) = chars.peek() {
                    if c == ',' || c == ']' {
                        break;
                    } else {
                        num.push(chars.next().unwrap());
                    }
                }

                num.parse().map(Term::Item).ok()
            }
            _ => None,
        }
    }
}

impl FromStr for Term {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Term::parse_helper(&mut s.chars().peekable()).ok_or("Failed to parse".to_owned())
    }
}

impl Debug for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Item(i) => write!(f, "{}", i),
            Term::List(items) => {
                write!(f, "[")?;
                for (idx, item) in items.iter().enumerate() {
                    write!(f, "{:?}", item)?;
                    if idx != items.len() - 1 {
                        write!(f, ",")?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}

impl Ord for Term {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Term::Item(a), Term::Item(b)) => a.cmp(b),
            (Term::List(left), Term::List(right)) => left
                .iter()
                .zip(right.iter())
                .find_map(|(a, b)| {
                    let result = a.cmp(b);
                    result.is_ne().then_some(result)
                })
                .unwrap_or_else(|| left.len().cmp(&right.len())),
            (Term::Item(_), Term::List(_)) => Term::List(vec![self.clone()]).cmp(other),
            (Term::List(_), Term::Item(_)) => self.cmp(&Term::List(vec![other.clone()])),
        }
    }
}

impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(input: &'static str) -> usize {
    input
        .split("\n\n")
        .map(|pair| {
            let mut pair = pair.lines();
            let a = Term::from_str(pair.next().unwrap()).unwrap();
            let b = Term::from_str(pair.next().unwrap()).unwrap();
            (a, b)
        })
        .enumerate()
        .filter_map(|(idx, (a, b))| if a < b { Some(idx + 1) } else { None })
        .sum()
}
fn part2(input: &'static str) -> usize {
    let mut input: Vec<_> = input
        .lines()
        .filter_map(|line| (!line.is_empty()).then(|| Term::from_str(line)))
        .flatten()
        .collect();
    input.extend([
        Term::List(vec![Term::List(vec![Term::Item(2)])]),
        Term::List(vec![Term::List(vec![Term::Item(6)])]),
    ]);
    input.sort();

    let mut key = 1;
    let mut found = false;
    for (idx, term) in input.iter().enumerate() {
        if term == &Term::List(vec![Term::List(vec![Term::Item(2)])])
            || term == &Term::List(vec![Term::List(vec![Term::Item(6)])])
        {
            key *= idx + 1;
            if found {
                break;
            }
            found = true;
        }
    }

    key
}

fn main() {
    let input = include_str!("day13-input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{part1, part2, Term};
    static TEST_INPUT: &'static str = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";
    #[test]
    fn test() {
        // debugging
        assert_eq!(
            TEST_INPUT,
            TEST_INPUT
                .split("\n\n")
                .map(|pair| {
                    let mut pair = pair.lines();
                    let a = Term::from_str(pair.next().unwrap()).unwrap();
                    let b = Term::from_str(pair.next().unwrap()).unwrap();
                    format!("{:?}\n{:?}\n", a, b)
                })
                .collect::<Vec<_>>()
                .join("\n")
        );
        assert_eq!(13, part1(TEST_INPUT));
        assert_eq!(140, part2(TEST_INPUT));
    }
}
