use std::{collections::HashSet, iter::FromIterator};

use crate::util::read_file;
use itertools::Itertools;
use regex::Regex;

type Coord = (i32, i32);
type Fold = (char, i32);

pub struct Puzzle {
    coords: Vec<Coord>,
    folds: Vec<Fold>,
}

pub fn parse_puzzle(file: &str) -> Puzzle {
    let coord_matcher = Regex::new(r"^([0-9]+),([0-9]+)$").unwrap();
    let fold_matcher = Regex::new(r"^fold along ([xy])=([0-9]+)$").unwrap();

    let mut on_folds = false;
    let mut coords: Vec<Coord> = vec![];
    let mut folds: Vec<Fold> = vec![];

    for line in read_file(file).unwrap() {
        if line.is_empty() {
            on_folds = true;
        } else if !on_folds {
            let captures = coord_matcher.captures(line.as_str()).unwrap();
            coords.push((captures[1].parse().unwrap(), captures[2].parse().unwrap()))
        } else {
            let captures = fold_matcher.captures(line.as_str()).unwrap();
            folds.push((
                captures[1].chars().nth(0).unwrap(),
                captures[2].parse().unwrap(),
            ))
        }
    }

    Puzzle { coords, folds }
}

pub fn part1(input: Puzzle) -> usize {
    let (fd, fp) = input.folds[0];
    input
        .coords
        .iter()
        .map(|(x, y)| match fd {
            'x' => (if *x > fp { fp - (*x - fp) } else { *x }, *y),
            'y' => (*x, if *y > fp { fp - (*y - fp) } else { *y }),
            _ => unimplemented!(),
        })
        .unique()
        .count()
}

pub fn part2(input: Puzzle) -> Vec<Coord> {
    input.folds.iter().fold(input.coords, |coords, (fd, fp)| {
        coords
            .iter()
            .map(|(x, y)| match fd {
                'x' => (if *x > *fp { *fp - (*x - *fp) } else { *x }, *y),
                'y' => (*x, if *y > *fp { fp - (*y - *fp) } else { *y }),
                _ => unimplemented!(),
            })
            .unique()
            .collect()
    })
}

pub fn print_puzzle(input: Vec<Coord>) {
    let input_set: HashSet<Coord> = HashSet::from_iter(input.iter().copied());
    let max_x = input.iter().map(|c| c.0).max().unwrap();
    let max_y = input.iter().map(|c| c.1).max().unwrap();
    for y in 0..=max_y {
        println!(
            "{}",
            (0..=max_x)
                .map(|x| {
                    if input_set.contains(&(x, y)) {
                        '#'
                    } else {
                        ' '
                    }
                })
                .join("")
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_puzzle() {
        let input = parse_puzzle("day13.example");
        assert_eq!(18, input.coords.len());
        assert_eq!(2, input.folds.len());
    }

    #[test]
    fn example_day13_part1() {
        let input = parse_puzzle("day13.example");
        assert_eq!(17, part1(input));
    }

    #[test]
    fn exec_day13_part1() {
        let input = parse_puzzle("day13.txt");
        println!("Day 13 Part 1 - {}", part1(input));
    }

    #[test]
    fn example_day13_part2() {
        let input = parse_puzzle("day13.example");
        print_puzzle(part2(input));
    }

    #[test]
    fn exec_day13_part2() {
        let input = parse_puzzle("day13.txt");
        print_puzzle(part2(input));
    }
}
