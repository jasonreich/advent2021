use std::collections::HashMap;

use itertools::{Itertools, MinMaxResult};

use crate::util::read_file;

type Substititions = HashMap<(char, char), char>;

pub struct Puzzle {
    template: String,
    substitutions: Substititions,
}

pub fn parse_puzzle(file: &str) -> Puzzle {
    let mut lines = read_file(file).unwrap();
    
    let template = lines.next().unwrap();
    
    lines.next();

    let mut substitutions = HashMap::new();

    while let Some(line) = lines.next() {
        let chars: Vec<char> = line.chars().collect();
        substitutions.insert((chars[0], chars[1]), chars[6]);
    }

    Puzzle { template, substitutions }
}

pub fn step(template: String, substitutions: &Substititions) -> String {
    let input: Vec<char> = template.chars().collect();
    let mut output: Vec<char> = Vec::new();

    for i in 0..(input.len() - 1) {
        output.push(input[i]);
        if let Some(c) = substitutions.get(&(input[i], input[i + 1])) {
            output.push(*c);
        }
    }
    output.push(*input.last().unwrap());

    output.iter().collect()
}

pub fn part1(input: Puzzle) -> usize {
    let mut template = input.template;
    for _ in 0..10 {
        template = step(template, &input.substitutions)
    }

    let output = template.chars().counts();
    if let MinMaxResult::MinMax(lower, upper) = output.values().minmax() {
        upper - lower
    } else {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        let puzzle = parse_puzzle("day14.example");
        assert_eq!(4, puzzle.template.len());
        assert_eq!(16, puzzle.substitutions.len());
        assert_eq!('B', puzzle.substitutions[&('C', 'H')])
    }

    #[test]
    fn test_step() {
        let puzzle = parse_puzzle("day14.example");
        assert_eq!("NCNBCHB", step(puzzle.template, &puzzle.substitutions));
    }

    #[test]
    fn example_day14_part1() {
        let input = parse_puzzle("day14.example");
        assert_eq!(1588, part1(input));
    }

    #[test]
    fn exec_day14_part1() {
        let input = parse_puzzle("day14.txt");
        println!("Day 14 Part 1 - {}", part1(input));
    }
}
