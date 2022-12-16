use std::{
    cmp::{max, min},
    collections::BTreeSet,
    convert::identity,
    ops::Range,
};

struct Intervals<T>(Vec<Range<T>>);
impl<T> Intervals<T> {
    fn new() -> Self {
        Intervals(Vec::new())
    }
}
impl<T: Copy + Ord> Intervals<T> {
    fn add(&mut self, range: Range<T>) {
        let i = self
            .0
            .binary_search_by_key(&range.start, |range| range.end)
            .unwrap_or_else(identity);
        let j = self
            .0
            .binary_search_by_key(&range.end, |range| range.start)
            .unwrap_or_else(identity);
        let range = if i < j {
            min(range.start, self.0[i].start)..max(range.end, self.0[j - 1].end)
        } else {
            range
        };
        self.0.splice(i..j, [range]);
    }
}

#[derive(Debug)]
struct Scan {
    sensor: (isize, isize),
    beacon: (isize, isize),
}

fn part1(input: &'static str, row: isize) -> isize {
    let (acc, set) = input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");

            let mut sensor = parts.next().unwrap()[10..]
                .split(", ")
                .map(|s| s[2..].parse::<isize>().unwrap());
            let sensor_x = sensor.next().unwrap();
            let sensor_y = sensor.next().unwrap();
            let sensor = (sensor_x, sensor_y);

            let mut beacon = parts.next().unwrap()[21..]
                .split(", ")
                .map(|s| s[2..].parse::<isize>().unwrap());
            let beacon_x = beacon.next().unwrap();
            let beacon_y = beacon.next().unwrap();
            let beacon = (beacon_x, beacon_y);
            Scan { sensor, beacon }
        })
        .fold(
            (Intervals::new(), BTreeSet::new()),
            |(mut acc, mut set), Scan { sensor, beacon }| {
                let (x0, y0) = sensor;
                let (x1, y1) = beacon;

                let dx = (x1 - x0).abs() + (y1 - y0).abs() - (row - y0).abs();

                if dx >= 0 {
                    acc.add(x0 - dx..x0 + dx + 1);
                }
                if y1 == row {
                    set.insert(x1);
                }

                (acc, set)
            },
        );

    acc.0
        .iter()
        .map(|range| range.end - range.start)
        .sum::<isize>()
        - set.len() as isize
}

fn part2(input: &'static str, searching_space: isize) -> u64 {
    let data: Vec<_> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");

            let mut sensor = parts.next().unwrap()[10..]
                .split(", ")
                .map(|s| s[2..].parse::<isize>().unwrap());
            let sensor_x = sensor.next().unwrap();
            let sensor_y = sensor.next().unwrap();
            let sensor = (sensor_x, sensor_y);

            let mut beacon = parts.next().unwrap()[21..]
                .split(", ")
                .map(|s| s[2..].parse::<isize>().unwrap());
            let beacon_x = beacon.next().unwrap();
            let beacon_y = beacon.next().unwrap();
            let beacon = (beacon_x, beacon_y);
            Scan { sensor, beacon }
        })
        .collect();

    (0..=searching_space)
        .filter_map(|y| {
            data.iter()
                .fold(Intervals::new(), |mut acc, Scan { sensor, beacon }| {
                    let (x0, y0) = sensor;
                    let (x1, y1) = beacon;

                    let dx = (x1 - x0).abs() + (y1 - y0).abs() - (y - y0).abs();
                    let lo = max(0, x0 - dx);
                    let hi = min(searching_space, x0 + dx);
                    if lo <= hi {
                        acc.add(lo..hi + 1);
                    }
                    acc
                })
                .0
                .into_iter()
                .chain([searching_space + 1..searching_space + 1])
                .scan(0, |acc, range| {
                    let x = Some(*acc).filter(|x| x < &range.start);
                    *acc = range.end;
                    Some(x)
                })
                .find_map(|x| x.map(|x| 4000000 * x as u64 + y as u64))
        })
        .next()
        .unwrap()
}

fn main() {
    let input = include_str!("day15-input.txt");
    println!("part1: {}", part1(input, 2000000));
    println!("part2: {}", part2(input, 4000000));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    static TEST_INPUT: &'static str = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
    #[test]
    fn test() {
        assert_eq!(26, part1(TEST_INPUT, 10));
        assert_eq!(56000011, part2(TEST_INPUT, 20));
    }
}
