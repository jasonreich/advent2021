use crate::util::read_lines;
use std::collections::HashSet;

type Coord = (u32, u32);

fn range(start: u32, end: u32) -> Vec<u32> {
    if start <= end {
        (start..=end).collect()
    } else {
        (end..=start).rev().collect()
    }
}

pub fn read_puzzle(filename: &str, with_diag: bool) -> impl Iterator<Item = Coord> {
    let intervals = read_lines(filename, |line| {
        let values: Vec<u32> = line
            .split(" -> ")
            .flat_map(|coord| coord.split(","))
            .map(|value| value.parse().unwrap())
            .collect();

        Some(((values[0], values[1]), (values[2], values[3])))
    })
    .unwrap();

    intervals.flat_map(move |((sx, sy), (tx, ty))| -> Vec<Coord> {
        if sx == tx {
            range(sy, ty).iter().map(|y| (sx, *y)).collect()
        } else if sy == ty {
            range(sx, tx).iter().map(|x| (*x, sy)).collect()
        } else if with_diag {
            range(sx, tx)
                .iter()
                .zip(range(sy, ty).iter())
                .map(|(&x, &y)| (x, y))
                .collect()
        } else {
            vec![]
        }
    })
}

pub fn find_danger(input: impl Iterator<Item = Coord>) -> u32 {
    let mut count = 0;
    let mut seen_once: HashSet<Coord> = HashSet::new();
    let mut seen_many: HashSet<Coord> = HashSet::new();
    for coord in input {
        if seen_once.contains(&coord) && !seen_many.contains(&coord) {
            seen_many.insert(coord);
            count += 1;
        }
        seen_once.insert(coord);
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_day05_part1() {
        assert_eq!(5, find_danger(read_puzzle("day05.example", false)));
    }

    #[test]
    fn exec_day05_part1() {
        println!(
            "Day 05, Part 1: {}",
            find_danger(read_puzzle("day05.txt", false))
        );
    }

    #[test]
    fn example_day05_part2() {
        assert_eq!(12, find_danger(read_puzzle("day05.example", true)));
    }

    #[test]
    fn exec_day05_part2() {
        println!(
            "Day 05, Part 2: {}",
            find_danger(read_puzzle("day05.txt", true))
        );
    }
}
