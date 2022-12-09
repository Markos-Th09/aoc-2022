fn part1() -> usize {
    let trees: Vec<Vec<_>> = include_str!("day8-input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    let (width, height) = (trees[0].len(), trees.len());
    let edges = (width * 2 + height * 2) - 4;

    let mut inner = 0;

    for i in 1..width - 1 {
        for j in 1..height - 1 {
            let tree = trees[i][j];

            if trees[i][0..j].iter().all(|&t| t < tree)
                || trees[i][j + 1..height].iter().all(|&t| t < tree)
                || trees[0..i].iter().all(|row| row[j] < tree)
                || trees[i + 1..width].iter().all(|row| row[j] < tree)
            {
                inner += 1;
            }
        }
    }

    inner + edges
}

fn get_trees(tree: usize, iter: impl Iterator<Item = usize>) -> usize {
    let mut score = 0;
    for other in iter {
        if other < tree {
            score += 1;
        } else {
            score += 1;
            break;
        }
    }
    score
}

fn part2() -> usize {
    let trees: Vec<Vec<_>> = include_str!("day8-input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    let (width, height) = (trees[0].len(), trees.len());
    let mut max_score = 0;

    for i in 1..width - 1 {
        for j in 1..height - 1 {
            let tree = trees[i][j];

            let left = get_trees(tree, trees[i][0..j].iter().rev().cloned());
            let right = get_trees(tree, trees[i][j + 1..].iter().cloned());
            let up = get_trees(tree, trees[0..i].iter().map(|row| row[j]).rev());
            let down = get_trees(tree, trees[i + 1..].iter().map(|row| row[j]));

            let score = left * right * up * down;

            if score > max_score {
                max_score = score;
            }
        }
    }

    max_score
}

fn main() {
    println!("Part 1: {:?}", part1());
    println!("Part 2: {:?}", part2());
}
