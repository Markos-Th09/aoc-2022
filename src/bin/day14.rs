use std::collections::HashMap;

#[derive(Debug)]
enum Material {
    Rock,
    Sand,
    Air,
}

fn part1(input: &'static str) -> usize {
    let mut grid = HashMap::new();

    grid.insert((500, 0), Material::Sand);
    let max = input
        .lines()
        .map(|line| {
            let coords = line
                .split(" -> ")
                .map(|x| {
                    let mut parts = x.split(",");
                    let x = parts.next().unwrap().parse::<usize>().unwrap();
                    let y = parts.next().unwrap().parse::<usize>().unwrap();
                    (x, y)
                })
                .collect::<Vec<_>>();

            coords.windows(2).for_each(|window| {
                let (x1, y1) = window[0];
                let (x2, y2) = window[1];
                if x1 == x2 {
                    for y in y1.min(y2)..=y1.max(y2) {
                        grid.insert((x1, y), Material::Rock);
                    }
                } else {
                    for x in x1.min(x2)..=x1.max(x2) {
                        grid.insert((x, y1), Material::Rock);
                    }
                }
            });
            *coords.iter().map(|(_, y)| y).max().unwrap()
        })
        .max()
        .unwrap();

    let (mut x, mut y) = (500, 0);
    let mut count = 0;

    loop {
        match grid.entry((x, y + 1)).or_insert(Material::Air) {
            Material::Air => {
                grid.insert((x, y), Material::Air);
                y += 1;
                grid.insert((x, y), Material::Sand);
            }
            Material::Rock | Material::Sand => {
                match grid.entry((x - 1, y + 1)).or_insert(Material::Air) {
                    Material::Air => {
                        grid.insert((x, y), Material::Air);
                        x -= 1;
                        y += 1;
                        grid.insert((x, y), Material::Sand);
                    }
                    _ => match grid.entry((x + 1, y + 1)).or_insert(Material::Air) {
                        Material::Air => {
                            grid.insert((x, y), Material::Air);
                            x += 1;
                            y += 1;
                            grid.insert((x, y), Material::Sand);
                        }
                        _ => {
                            count += 1;
                            (x, y) = (500, 0);
                        }
                    },
                }
            }
        }
        if y >= max {
            break;
        }
    }

    count
}

fn part2(input: &'static str) -> usize {
    let mut grid = HashMap::new();

    grid.insert((500, 0), Material::Sand);
    let max = input
        .lines()
        .map(|line| {
            let coords = line
                .split(" -> ")
                .map(|x| {
                    let mut parts = x.split(",");
                    let x = parts.next().unwrap().parse::<usize>().unwrap();
                    let y = parts.next().unwrap().parse::<usize>().unwrap();
                    (x, y)
                })
                .collect::<Vec<_>>();

            coords.windows(2).for_each(|window| {
                let (x1, y1) = window[0];
                let (x2, y2) = window[1];
                if x1 == x2 {
                    for y in y1.min(y2)..=y1.max(y2) {
                        grid.insert((x1, y), Material::Rock);
                    }
                } else {
                    for x in x1.min(x2)..=x1.max(x2) {
                        grid.insert((x, y1), Material::Rock);
                    }
                }
            });
            *coords.iter().map(|(_, y)| y).max().unwrap()
        })
        .max()
        .unwrap()
        + 2;

    let (mut x, mut y) = (500, 0);
    let mut count = 0;
    loop {
        if y < max {
            match grid.entry((x, y + 1)).or_insert(Material::Air) {
                Material::Air => {
                    grid.insert((x, y), Material::Air);
                    y += 1;
                    grid.insert((x, y), Material::Sand);
                }
                Material::Rock | Material::Sand => {
                    match grid.entry((x - 1, y + 1)).or_insert(Material::Air) {
                        Material::Air => {
                            grid.insert((x, y), Material::Air);
                            x -= 1;
                            y += 1;
                            grid.insert((x, y), Material::Sand);
                        }
                        _ => match grid.entry((x + 1, y + 1)).or_insert(Material::Air) {
                            Material::Air => {
                                grid.insert((x, y), Material::Air);
                                x += 1;
                                y += 1;
                                grid.insert((x, y), Material::Sand);
                            }
                            _ => {
                                count += 1;
                                if (x, y) == (500, 0) {
                                    break;
                                }
                                (x, y) = (500, 0);
                            }
                        },
                    }
                }
            }
        } else {
            (x, y) = (500, 0);
            continue;
        }
    }

    count
}

fn main() {
    let input = include_str!("day14-input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    static TEST_INPUT: &'static str = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    #[test]
    fn test() {
        assert_eq!(24, part1(TEST_INPUT));
        assert_eq!(93, part2(TEST_INPUT));
    }
}
