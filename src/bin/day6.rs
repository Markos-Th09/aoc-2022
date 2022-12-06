use std::collections::HashSet;

fn solution(size: usize) -> usize {
    include_str!("day6-input.txt")
        .chars()
        .collect::<Vec<char>>()
        .windows(size)
        .take_while(|chars| chars.iter().collect::<HashSet<_>>().len() != size)
        .count()
        + size
}

fn main() {
    println!("part 1: {:?}", solution(4));
    println!("part 2: {:?}", solution(14));
}
