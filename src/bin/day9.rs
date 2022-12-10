use std::collections::HashSet;

type Pos = (isize, isize);

fn simulate_rope(input: &'static str, length: usize) -> usize {
    let moves = input.lines();
    let mut rope = vec![(0, 0); length];
    let mut visited: HashSet<Pos> = HashSet::new();
    for mov in moves {
        let (x, y, units) = match mov.split_at(2) {
            ("R ", n) => (1, 0, n),
            ("L ", n) => (-1, 0, n),
            ("U ", n) => (0, 1, n),
            ("D ", n) => (0, -1, n),
            (_, _) => unreachable!(),
        };

        for _ in 0..units.parse::<usize>().unwrap() {
            rope[0].0 += x;
            rope[0].1 += y;
            for i in 1..length {
                if let Some(pos) = move_adjacent(&rope[i], &rope[i - 1]) {
                    rope[i] = pos;
                } else {
                    break;
                }
            }
            visited.insert(*rope.last().unwrap());
        }
    }
    visited.len()
}

fn move_adjacent(tail: &Pos, head: &Pos) -> Option<Pos> {
    let dx = tail.0 - head.0;
    let dy = tail.1 - head.1;

    if (dx == 2 || dx == -2) && (dy == 2 || dy == -2) {
        Some((head.0 + dx.clamp(-1, 1), head.1 + dy.clamp(-1, 1)))
    } else if dx == 2 || dx == -2 {
        Some((head.0 + dx.clamp(-1, 1), head.1))
    } else if dy == 2 || dy == -2 {
        Some((head.0, head.1 + dy.clamp(-1, 1)))
    } else {
        None // already adjacent
    }
}

fn main() {
    let input = include_str!("day9-input.txt");
    println!("part1: {:?}", simulate_rope(input, 2));
    println!("part2: {:?}", simulate_rope(input, 10));
}

#[cfg(test)]
mod tests {
    use crate::simulate_rope;

    #[test]
    fn test_day9() {
        let input = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(13, simulate_rope(input, 2));
        assert_eq!(1, simulate_rope(input, 10));
    }
}
