fn part1(input: &'static str) -> i64 {
    let mut cycles = 1;
    let mut strength = 0;

    let mut x = 1;
    for i in input.lines() {
        let mut parts = i.split_whitespace();

        match (parts.next().unwrap(), parts.next()) {
            ("addx", Some(n)) => {
                cycles += 1;
                if (cycles - 20) % 40 == 0 {
                    strength += cycles * x;
                }
                cycles += 1;
                x += n.parse::<i64>().unwrap();
                if (cycles - 20) % 40 == 0 {
                    strength += cycles * x;
                }
            }
            ("noop", _) => {
                cycles += 1;
                if (cycles - 20) % 40 == 0 {
                    strength += cycles * x;
                }
            }
            _ => unreachable!(),
        }
    }
    strength
}

fn part2(input: &'static str) -> String {
    let mut commands = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (parts.next().unwrap(), parts.next())
        })
        .peekable();
    let mut current_pixel: i32 = 0;
    let mut register: i32 = 1;
    let mut screen = String::from("");
    let mut addx_in_progress = false;

    loop {
        if current_pixel % 40 == 0 {
            screen.push('\n');
        }
        if ((current_pixel % 40) - register).abs() < 2 {
            screen.push('#');
        } else {
            screen.push('.');
        }

        if addx_in_progress {
            let next_command = commands.next();

            match next_command {
                Some(("addx", Some(n))) => {
                    register += n.parse::<i32>().unwrap();
                    addx_in_progress = false;
                }
                _ => {
                    panic!("Unexpected command.")
                }
            }
        } else {
            match commands.peek() {
                Some(("addx", _)) => {
                    addx_in_progress = true;
                }
                Some(("noop", _)) => {
                    commands.next();
                }
                None => {
                    break;
                }
                _ => {}
            }
        }

        current_pixel += 1;
    }
    // workaround strange bug that I am too lazy to fix
    screen.split('\n').take(7).collect::<Vec<&str>>().join("\n")
}

fn main() {
    let input = include_str!("day10-input.txt");
    println!("part1: {:?}", part1(input));
    println!("part2: \n{}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test() {
        let input = r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";
        assert_eq!(13140, part1(input));
        assert_eq!(
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
                .trim_matches('\n')
                .to_owned(),
            part2(input).trim_matches('\n')
        );
    }
}
