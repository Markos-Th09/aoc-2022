use std::collections::{HashMap, HashSet};

static LETTERS: &'static [char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn part1() -> usize {
    let priorities: HashMap<char, usize> =
        HashMap::from_iter(LETTERS.iter().enumerate().map(|(i, &x)| (x, i + 1)));

    include_str!("day3-input.txt").lines().fold(0, |acc, x| {
        let idx = x.len() / 2;
        let p1 = &x[0..idx];
        let p2 = &x[idx..];

        let set: HashSet<char> = HashSet::from_iter(p1.chars());
        let set2 = HashSet::from_iter(p2.chars());
        let dup = set.intersection(&set2).next().unwrap();

        acc + *priorities.get(&dup).unwrap()
    })
}

fn part2() -> usize {
    let lines: Vec<&str> = include_str!("day3-input.txt").lines().collect();

    let priorities: HashMap<char, usize> =
        HashMap::from_iter(LETTERS.iter().enumerate().map(|(i, &x)| (x, i + 1)));

    lines.chunks(3).fold(0, |acc, elfs| {
        let elf1: HashSet<char> = HashSet::from_iter(elfs[0].chars());
        let elf2: HashSet<char> = HashSet::from_iter(elfs[1].chars());
        let elf3: HashSet<char> = HashSet::from_iter(elfs[2].chars());

        let i_set = elf1
            .intersection(&elf2)
            .map(|&c| c)
            .collect::<HashSet<char>>();

        acc + *priorities
            .get(i_set.intersection(&elf3).next().unwrap())
            .unwrap()
    })
}

fn main() {
    println!("part 1: {}", part1());
    println!("part 2: {}", part2());
}
