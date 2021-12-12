use itertools::Itertools;
use multimap::MultiMap;
use std::iter::FromIterator;

use crate::util::read_lines;

type Puzzle = MultiMap<String, String>;

pub fn read_puzzle(file: &str) -> Puzzle {
    MultiMap::from_iter(
        read_lines(file, |line| {
            let mut components = line.split('-');
            Some((
                components.next()?.to_string(),
                components.next()?.to_string(),
            ))
        })
        .unwrap()
        .flat_map(|(src, tgt)| vec![(src.clone(), tgt.clone()), (tgt, src)]),
    )
}

pub fn is_big(cave: &String) -> bool {
    cave.chars().all(|c| c.is_uppercase())
}

pub fn part1(input: Puzzle) -> u32 {
    let mut count = 0;
    let mut paths = vec![vec!["start"]];
    while let Some(path) = paths.pop() {
        let end = path[0];
        if end != "end" {
            let next = input.get_vec(end);
            if next.is_some() {
                next.unwrap()
                    .iter()
                    .filter(|cave| is_big(cave) || path.iter().all(|c| c != cave))
                    .for_each(|cave| {
                        let mut new_cave = path.clone();
                        new_cave.insert(0, cave);
                        paths.push(new_cave)
                    });
            }
        } else {
            count += 1;
        }
    }
    count
}

pub fn part2(input: Puzzle) -> u32 {
    let mut count = 0;
    let mut paths = vec![vec!["start"]];
    while let Some(path) = paths.pop() {
        let end = path[0];
        if end != "end" {
            let used_small = path.iter().filter(|&&cave| !is_big(&cave.to_string())).duplicates().count() > 0;
            let next = input.get_vec(end);
            if next.is_some() {
                next.unwrap()
                    .iter()
                    .filter(|cave| *cave != "start")
                    .filter(|cave| is_big(cave) || !used_small || path.iter().all(|c| c != cave))
                    .for_each(|cave| {
                        let mut new_cave = path.clone();
                        new_cave.insert(0, cave);
                        paths.push(new_cave)
                    });
            }
        } else {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_puzzle() {
        let input = read_puzzle("day12.example1");
        assert_eq!(2, input.get_vec("start").unwrap().len());
    }

    #[test]
    fn test_is_big() {
        assert!(is_big(&"ABC".to_string()));
        assert!(!is_big(&"abc".to_string()));
    }

    #[test]
    fn example1_day12_part1() {
        let input = read_puzzle("day12.example1");
        assert_eq!(10, part1(input));
    }

    #[test]
    fn example2_day12_part1() {
        let input = read_puzzle("day12.example2");
        assert_eq!(19, part1(input));
    }

    #[test]
    fn example3_day12_part1() {
        let input = read_puzzle("day12.example3");
        assert_eq!(226, part1(input));
    }

    #[test]
    fn exec_day12_part1() {
        let input = read_puzzle("day12.txt");
        println!("Day 12 Part 1 - {}", part1(input));
    }

    #[test]
    fn example1_day12_part2() {
        let input = read_puzzle("day12.example1");
        assert_eq!(36, part2(input));
    }

    #[test]
    fn example2_day12_part2() {
        let input = read_puzzle("day12.example2");
        assert_eq!(103, part2(input));
    }

    #[test]
    fn example3_day12_part2() {
        let input = read_puzzle("day12.example3");
        assert_eq!(3509, part2(input));
    }

    #[test]
    fn exec_day12_part2() {
        let input = read_puzzle("day12.txt");
        println!("Day 12 Part 2 - {}", part2(input));
    }
}
