use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

type AdjList<'a> = HashMap<&'a Valve, Vec<&'a Valve>>;
type DistanceMatrix<'a> = HashMap<String, HashMap<String, i32>>;

fn find_path<'a>(
    valves: &'a AdjList,
    to_open: &Vec<&'a Valve>,
    distances: &'a DistanceMatrix,
    start: &'a Valve,
    minutes: i32,
    path: &Vec<&'a Valve>,
    no_overlap: &Vec<&'a Valve>,
) -> PathFit<'a> {
    let mut paths: Vec<PathFit> = Vec::new();

    for valve in to_open {
        if no_overlap.contains(valve) {
            continue;
        }

        let distance = distances[&start.name][&valve.name];
        if distance >= minutes {
            continue;
        }
        let minutes_left = minutes - distance - 1;
        let flow = valve.flow * minutes_left;
        let next_to_open = &to_open.iter().filter(|v| *v != valve).map(|v| *v).collect();

        let mut next_path = path.clone();
        next_path.push(valve);
        let full_path = find_path(
            valves,
            next_to_open,
            distances,
            valve,
            minutes_left,
            &next_path,
            &no_overlap,
        );
        let mut add_path = path.clone();
        add_path.extend(full_path.path);
        paths.push(PathFit {
            path: add_path,
            flow: full_path.flow + flow,
        });
    }

    let mut best_path = PathFit {
        path: Vec::new(),
        flow: 0,
    };
    for path_fit in paths {
        if path_fit.flow > best_path.flow {
            best_path = path_fit;
        }
    }
    best_path
}

fn distance_matrix<'a>(valves: &'a AdjList) -> DistanceMatrix<'a> {
    let mut distances = HashMap::new();
    for start in valves.keys() {
        let start = *start;
        let distances_from = distances
            .entry(start.name.clone())
            .or_insert(HashMap::new());
        let mut to_visit = BinaryHeap::new();
        let mut visited = HashSet::new();

        to_visit.push(Visit {
            valve: start,
            distance: 0,
        });

        while let Some(Visit { valve, distance }) = to_visit.pop() {
            if !visited.insert(valve) {
                continue;
            }

            if let Some(neighbours) = valves.get(valve) {
                for neighbour in neighbours {
                    let neighbour = *neighbour;
                    let new_dist = distance + 1;
                    let use_dist = distances_from
                        .get(&neighbour.name)
                        .map_or(true, |&current_dist| current_dist > new_dist);
                    if use_dist {
                        distances_from.insert(neighbour.name.clone(), new_dist);
                        to_visit.push(Visit {
                            valve: neighbour,
                            distance: new_dist,
                        });
                    }
                }
            }
        }
    }
    distances
}

#[derive(Debug, Clone)]
struct PathFit<'a> {
    path: Vec<&'a Valve>,
    flow: i32,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Valve {
    name: String,
    flow: i32,
}

#[derive(Debug)]
struct Visit<'a> {
    valve: &'a Valve,
    distance: i32,
}

impl<'a> Ord for Visit<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<'a> PartialOrd for Visit<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for Visit<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<'a> Eq for Visit<'a> {}

fn part1(input: &'static str) -> i32 {
    let mut neighbours = HashMap::new();
    let valves: Vec<_> = input
        .lines()
        .map(|line| {
            let mut parts = line.split("; ");
            let info = parts.next().unwrap();
            let name = info[6..8].to_string();
            let flow = info[23..].parse::<i32>().unwrap();
            let tunnels: Vec<_> = parts.next().unwrap()[22..]
                .split(", ")
                .map(|tunnel| tunnel.trim())
                .collect();
            neighbours.insert(name.clone(), tunnels);

            Valve { name, flow }
        })
        .collect();

    let mut adj: HashMap<&Valve, Vec<&Valve>> = HashMap::new();
    for valve in valves.iter() {
        let neighbour_list = neighbours.get(&valve.name).unwrap();
        for neighbour in neighbour_list {
            let nvalve = valves
                .iter()
                .filter(|v| &v.name == neighbour)
                .next()
                .unwrap();
            adj.entry(valve).or_insert(Vec::new()).push(nvalve);
        }
    }

    let distances = distance_matrix(&adj);
    let start = adj.keys().find(|&k| k.name == "AA").unwrap();
    let valves_to_open: Vec<_> = adj.keys().filter(|v| v.flow > 0).cloned().collect();

    let path = find_path(
        &adj,
        &valves_to_open,
        &distances,
        start,
        30,
        &vec![start],
        &Vec::new(),
    );

    path.flow
}

fn part2(input: &'static str) -> i32 {
    let mut neighbours = HashMap::new();
    let valves: Vec<_> = input
        .lines()
        .map(|line| {
            let mut parts = line.split("; ");
            let info = parts.next().unwrap();
            let name = info[6..8].to_string();
            let flow = info[23..].parse::<i32>().unwrap();
            let tunnels: Vec<_> = parts.next().unwrap()[22..]
                .split(", ")
                .map(|tunnel| tunnel.trim())
                .collect();
            neighbours.insert(name.clone(), tunnels);

            Valve { name, flow }
        })
        .collect();

    let mut adj: HashMap<&Valve, Vec<&Valve>> = HashMap::new();
    for valve in valves.iter() {
        let neighbour_list = neighbours.get(&valve.name).unwrap();
        for neighbour in neighbour_list {
            let nvalve = valves
                .iter()
                .filter(|v| &v.name == neighbour)
                .next()
                .unwrap();
            adj.entry(valve).or_insert(Vec::new()).push(nvalve);
        }
    }

    let distances = distance_matrix(&adj);
    let start = adj.keys().find(|&k| k.name == "AA").unwrap();
    let valves_to_open: Vec<_> = adj.keys().filter(|v| v.flow > 0).cloned().collect();

    let human_path = find_path(
        &adj,
        &valves_to_open,
        &distances,
        start,
        26,
        &vec![start],
        &Vec::new(),
    );

    let elephant_path = find_path(
        &adj,
        &valves_to_open,
        &distances,
        start,
        26,
        &vec![start],
        &human_path.path,
    );

    human_path.flow + elephant_path.flow
}

fn main() {
    let input = include_str!("day16-input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    static TEST_INPUT: &'static str = r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
    #[test]
    fn test() {
        assert_eq!(1651, part1(TEST_INPUT));
        assert_eq!(1707, part2(TEST_INPUT));
    }
}
