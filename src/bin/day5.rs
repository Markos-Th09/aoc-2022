fn solution(part2: bool) -> String {
    let lines: Vec<&str> = include_str!("day5-input.txt").lines().collect();
    let mut crates = lines
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|&s| s)
        .collect::<Vec<&str>>();

    crates.pop();
    let crates = crates
        .iter()
        .map(|line| {
            let chars = line.chars().collect::<Vec<char>>();
            chars
                .chunks(4)
                .map(|chunk| {
                    if chunk.len() == 4
                        && (chunk[0], chunk[1], chunk[2], chunk[3]) == (' ', ' ', ' ', ' ')
                    {
                        "   ".to_string()
                    } else {
                        chunk[0].to_string() + &chunk[1].to_string() + &chunk[2].to_string()
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut crates = (0..crates[0].len())
        .map(|i| {
            crates
                .iter()
                .filter_map(|row| {
                    if row[i] != "   " {
                        Some(row[i].clone())
                    } else {
                        None
                    }
                })
                .rev()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let instructions = lines
        .iter()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|&s| {
            let mut iter = s
                .split_whitespace()
                .skip(1)
                .step_by(2)
                .map(|s| s.parse::<usize>().unwrap());
            (
                iter.next().unwrap(),
                iter.next().unwrap() - 1,
                iter.next().unwrap() - 1,
            )
        });

    for (num_crates, src, dest) in instructions {
        let len = crates[src].len();
        let (remaining_crates, crates_to_move) = crates[src].split_at_mut(len - num_crates);
        if !part2 {
            crates_to_move.reverse()
        };
        let crates_to_move = crates_to_move.to_vec();

        crates[src] = remaining_crates.to_vec();
        crates[dest].append(&mut crates_to_move.to_vec());
    }

    crates
        .iter()
        .fold(String::with_capacity(crates.len()), |acc, row| {
            if let Some(last) = row.last() {
                acc + last[1..=1].as_ref()
            } else {
                acc
            }
        })
}

fn main() {
    println!("part 1: {:?}", solution(false));
    println!("part 2: {:?}", solution(true));
}
