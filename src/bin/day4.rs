fn part1() -> usize {
    include_str!("day4-input.txt")
        .lines()
        .map(|line| {
            let mut split = line.split(",");

            let mut work1 = split.next().unwrap().split("-");
            let x1 = work1.next().unwrap().parse::<usize>().unwrap();
            let y1 = work1.next().unwrap().parse::<usize>().unwrap();

            let mut work2 = split.next().unwrap().split("-");
            let x2 = work2.next().unwrap().parse::<usize>().unwrap();
            let y2 = work2.next().unwrap().parse::<usize>().unwrap();

            ((x1, y1), (x2, y2))
        })
        .fold(0, |mut acc, ((x1, y1), (x2, y2))| {
            // check fully contained
            if (x1 <= x2 && y1 >= y2)
                || (x2 <= x1 && y2 >= y1)
                || (x1 <= x2 && y1 <= y2 && y2 <= x1)
                || (x2 <= x1 && y2 <= y1 && y1 <= x2)
            {
                acc += 1;
            }
            acc
        })
}

fn part2() -> usize {
    include_str!("day4-input.txt")
        .lines()
        .map(|line| {
            let mut split = line.split(",");

            let mut work1 = split.next().unwrap().split("-");
            let x1 = work1.next().unwrap().parse::<usize>().unwrap();
            let y1 = work1.next().unwrap().parse::<usize>().unwrap();

            let mut work2 = split.next().unwrap().split("-");
            let x2 = work2.next().unwrap().parse::<usize>().unwrap();
            let y2 = work2.next().unwrap().parse::<usize>().unwrap();

            ((x1, y1), (x2, y2))
        })
        .fold(0, |mut acc, ((x1, y1), (x2, y2))| {
            // check overlap
            if (x2 >= x1 && x2 <= y1)
                || (y2 >= x1 && y2 <= y1)
                || (x1 >= x2 && x1 <= y2)
                || (y1 >= x2 && y1 <= y2)
            {
                acc += 1;
            }
            acc
        })
}

fn main() {
    println!("part 1: {}", part1());
    println!("part 2: {}", part2());
}
