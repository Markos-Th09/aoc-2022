use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    hash::{Hash, Hasher},
    sync::Arc,
};

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
    elevation: i32,
    dist_from_start: Option<i32>,
    maybe_total_dist: Option<i32>,
}

impl Coord {
    fn same_location(&self, other: &Coord) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Coord {}

impl Hash for Coord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .maybe_total_dist
            .cmp(&self.maybe_total_dist)
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn reconstruct_path(came_from: HashMap<Coord, (Coord, i32)>, current: Coord) -> Vec<Coord> {
    let mut total_path = vec![current];
    let mut parent_node = current;
    while let Some((parent, _)) = came_from.get(&parent_node) {
        total_path.push(*parent);
        parent_node = *parent;
    }

    total_path.into_iter().rev().collect()
}

// The heuristic for A* is the naive number of steps to get to the end, not taking into account the height.
fn distance_heuristic(coord: Coord, end_coord: Coord) -> i32 {
    (coord.x - end_coord.x).abs() + (coord.y - end_coord.y).abs()
}

// Gets nearby coordinates, making sure we don't index outside of the map.
fn get_nearby_coords(grid: &Vec<Vec<Coord>>, coord: Coord) -> Vec<Coord> {
    let mut coords = vec![];

    if coord.x - 1 >= 0 {
        coords.push(grid[coord.y as usize][coord.x as usize - 1]);
    }
    if coord.x + 1 < grid[0].len() as i32 {
        coords.push(grid[coord.y as usize][coord.x as usize + 1]);
    }
    if coord.y - 1 >= 0 {
        coords.push(grid[coord.y as usize - 1][coord.x as usize]);
    }
    if coord.y + 1 < grid.len() as i32 {
        coords.push(grid[coord.y as usize + 1][coord.x as usize]);
    }

    coords
}

// Finds the shortest path from start_coord to end_coord in the grid using A*.
// Wasn't easy to implement
fn find_node_path(grid: Arc<Vec<Vec<Coord>>>, start: Coord, end: Coord) -> Option<Vec<Coord>> {
    // Create the A* data structures.
    let mut pending_nodes: BinaryHeap<Coord> = BinaryHeap::new();
    let mut came_from: HashMap<Coord, (Coord, i32)> = HashMap::new(); // Include the cost for this pair.
    let mut score_from_start: HashMap<Coord, i32> = HashMap::new();

    // Initialize data with the start coordinate.
    pending_nodes.push(Coord {
        maybe_total_dist: Some(distance_heuristic(start, end)),
        ..start
    });
    score_from_start.insert(start, 0);

    // Loop until we run out of nodes to check.
    while let Some(current) = pending_nodes.pop() {
        if current.same_location(&end) {
            return Some(reconstruct_path(came_from, current));
        }

        // If we had previously found a better path to this node, we don't need to process this entry
        if let Some(&(_, best_dist_from_start)) = came_from.get(&current) {
            if let Some(current_dist_from_start) = current.dist_from_start {
                if current_dist_from_start > best_dist_from_start {
                    continue;
                }
            }
        }

        // Check each neighboring cell that we can actually step to to see if it might become a better path to the end.
        for neighbor in get_nearby_coords(&grid, current)
            .into_iter()
            .filter(|coord| current.elevation + 1 >= coord.elevation)
        {
            // The distance from start to the neighbor through the current node.
            // The distance between nodes is always 1 in this set.
            let new_dist_from_start = current.dist_from_start.unwrap_or(0) + 1;
            let best_dist_from_start = score_from_start.get(&neighbor);

            // If the path to the neighbor via current is better than the previous best distance to neighbor, let's change the route to use current
            if best_dist_from_start == None || new_dist_from_start < *best_dist_from_start.unwrap()
            {
                came_from.insert(neighbor, (current, new_dist_from_start));
                score_from_start.insert(neighbor, new_dist_from_start);

                let possible_end_distance =
                    Some(new_dist_from_start + distance_heuristic(neighbor, end));

                // Indicate we want to check this node again in the future
                pending_nodes.push(Coord {
                    dist_from_start: Some(new_dist_from_start),
                    maybe_total_dist: possible_end_distance,
                    ..neighbor
                });
            }
        }
    }

    // No path was found
    None
}

fn part1(input: &'static str) -> i32 {
    let mut start = None;
    let mut end = None;
    let grid: Vec<Vec<Coord>> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map({
                    |(j, c)| match c {
                        'E' | 'S' => {
                            let coord = Coord {
                                x: j as i32,
                                y: i as i32,
                                elevation: if c == 'E' { 26 } else { 1 },
                                dist_from_start: if c == 'E' { None } else { Some(0) },
                                maybe_total_dist: None,
                            };
                            if c == 'E' {
                                end = Some(coord);
                            } else {
                                start = Some(coord);
                            }
                            coord
                        }
                        c => Coord {
                            x: j as i32,
                            y: i as i32,
                            elevation: c as i32 - 96,
                            dist_from_start: None,
                            maybe_total_dist: None,
                        },
                    }
                })
                .collect()
        })
        .collect();

    find_node_path(Arc::new(grid), start.unwrap(), end.unwrap())
        .unwrap()
        .len() as i32
        - 1
}

fn part2(input: &'static str) -> usize {
    let mut possible_starts = vec![];
    let mut end = None;
    let grid: Vec<Vec<Coord>> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map({
                    |(j, c)| match c {
                        'E' | 'S' => {
                            let coord = Coord {
                                x: j as i32,
                                y: i as i32,
                                elevation: if c == 'E' { 26 } else { 1 },
                                dist_from_start: if c == 'E' { None } else { Some(0) },
                                maybe_total_dist: None,
                            };
                            if c == 'E' {
                                end = Some(coord);
                            } else {
                                possible_starts.push(coord);
                            }
                            coord
                        }
                        c => {
                            let coord = Coord {
                                x: j as i32,
                                y: i as i32,
                                elevation: c as i32 - 96,
                                dist_from_start: None,
                                maybe_total_dist: None,
                            };
                            if c == 'a' {
                                possible_starts.push(coord);
                            }
                            coord
                        }
                    }
                })
                .collect()
        })
        .collect();
    let grid = Arc::new(grid);
    let end = end.unwrap();

    let iter = possible_starts.iter();

    iter.map(|start| {
        std::thread::spawn({
            let grid = grid.clone();
            let start = start.clone();
            let end = end.clone();
            move || {
                let path = find_node_path(grid, start, end)?;
                Some(path.len() as i32 - 1)
            }
        })
    })
    .flat_map(|t| t.join().unwrap())
    .min()
    .unwrap() as usize
}

fn main() {
    let input = include_str!("day12-input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test() {
        let input = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";
        assert_eq!(31, part1(input));
        assert_eq!(29, part2(input));
    }
}
