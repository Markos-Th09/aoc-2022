static ROCK: u64 = 1;
static PAPER: u64 = 2;
static SCISSORS: u64 = 3;

fn part1() -> u64 {
    include_str!("day2-input.txt")
        .lines()
        .map(|x| {
            let split = x.split(" ").collect::<Vec<&str>>();
            let mut score = 0;
            /*
            A|X rock
            B|Y paper
            C|Z scissors
            */
            match split[0] {
                "A" => {
                    if split[1] == "Y" {
                        score += 6;
                    } else if split[1] == "X" {
                        score += 3;
                    }
                }
                "B" => {
                    if split[1] == "Z" {
                        score += 6;
                    } else if split[1] == "Y" {
                        score += 3;
                    }
                }
                "C" => {
                    if split[1] == "X" {
                        score += 6;
                    } else if split[1] == "Z" {
                        score += 3;
                    }
                }
                _ => unreachable!(),
            }

            match split[1] {
                "X" => score += 1,
                "Y" => score += 2,
                "Z" => score += 3,
                _ => unreachable!(),
            }

            score
        })
        .sum()
}

fn part2() -> u64 {
    include_str!("day2-input.txt")
        .lines()
        .map(|x| {
            let split = x.split(" ").collect::<Vec<&str>>();
            let mut score = 0;
            /*
            A rock 1
            B paper 2
            C scissors 3

            X lose
            Y draw
            Z win
            */

            match split[1] {
                "X" => {}
                "Y" => score += 3,
                "Z" => score += 6,
                _ => unreachable!(),
            }

            match split[0] {
                "A" => match split[1] {
                    "X" => score += SCISSORS,
                    "Y" => score += ROCK,
                    "Z" => score += PAPER,
                    _ => unreachable!(),
                },
                "B" => match split[1] {
                    "X" => {
                        score += ROCK // rock
                    }
                    "Y" => {
                        score += PAPER // paper
                    }
                    "Z" => {
                        score += SCISSORS // scissors
                    }
                    _ => unreachable!(),
                },
                "C" => match split[1] {
                    "X" => score += PAPER,
                    "Y" => score += SCISSORS,
                    "Z" => score += ROCK,
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }

            score
        })
        .sum()
}

fn main() {
    println!("part 1: {}", part1());
    println!("part 2: {}", part2());
}
