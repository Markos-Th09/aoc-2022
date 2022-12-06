fn main() {
    let mut elfs: Vec<usize> = include_str!("day1-input.txt")
        .split("\n\n")
        .map(|cals| cals.lines().flat_map(|x| x.parse::<usize>()).sum())
        .collect();
    elfs.sort_by(|a, b| b.cmp(a));

    println!("max: {}", elfs[0]);
    println!("top 3 sum: {}", elfs[0..3].iter().sum::<usize>());
}
