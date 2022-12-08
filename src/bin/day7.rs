use std::collections::HashMap;

static DISK_SIZE: usize = 70000000;
static SPACE_REQUIRED: usize = 30000000;

fn parse_fs() -> HashMap<String, usize> {
    let lines: Vec<&str> = include_str!("day7-input.txt").lines().collect();

    let mut cwd = Vec::new();
    let mut fs: HashMap<String, usize> = HashMap::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        if line.starts_with("$") {
            let mut parts = line[2..].split_whitespace();
            let cmd = parts.next().unwrap();

            match cmd {
                "cd" => {
                    let dir = parts.next().unwrap();
                    match dir {
                        ".." => {
                            cwd.pop();
                        }
                        "/" => {
                            cwd.clear();
                        }
                        _ => cwd.push(dir),
                    }
                }
                "ls" => {
                    let path = cwd.join("/");
                    let mut sum = 0;
                    i += 1;

                    while i < lines.len() && !lines[i].starts_with("$") {
                        let line = lines[i];
                        if !line.starts_with("dir") {
                            let mut parts = line.split_whitespace();

                            let size = parts.next().unwrap().parse::<usize>().unwrap();

                            sum += size;
                        }
                        i += 1;
                    }
                    i -= 1;
                    fs.insert(path.clone(), sum);

                    for i in 0..cwd.len().saturating_sub(1) {
                        let dir = &cwd[0..cwd.len().saturating_sub(1 + i)].join("/");

                        fs.entry(dir.to_string()).and_modify(|size| {
                            *size += sum;
                        });
                    }
                    if !path.is_empty() {
                        fs.entry("".to_owned()).and_modify(|size| {
                            *size += sum;
                        });
                    }
                }
                _ => {}
            }
        }
        i += 1;
    }
    fs
}

fn part1() -> usize {
    let fs = parse_fs();
    fs.values().filter(|&&x| x <= 100000).sum()
}

fn part2() -> usize {
    let fs = parse_fs();
    let free_space = DISK_SIZE - *fs.get("").unwrap();
    let space_to_free = SPACE_REQUIRED - free_space;

    let mut values: Vec<(&String, &usize)> = fs.iter().collect();

    values.sort_by(|(_, a), (_, b)| a.cmp(b));

    for (_, size) in values {
        if *size > space_to_free {
            return *size;
        }
    }
    unreachable!()
}

fn main() {
    println!("part1: {:?}", part1());
    println!("part2: {:?}", part2());
}
