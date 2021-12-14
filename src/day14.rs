use std::collections::HashMap;

use itertools::{Itertools, MinMaxResult};

use crate::util::read_file;

type Bigram = (char, char);
type Substititions = HashMap<Bigram, char>;

pub struct Puzzle {
    template: String,
    substitutions: Substititions,
}

pub fn parse_puzzle(file: &str) -> Puzzle {
    let mut lines = read_file(file).unwrap();

    let template = lines.next().unwrap();

    lines.next();

    let substitutions: Substititions = lines
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            ((chars[0], chars[1]), chars[6])
        })
        .collect();

    Puzzle {
        template,
        substitutions,
    }
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

pub fn part2(input: &Puzzle, steps: u32) -> usize {
    // Initialise
    let mut bigram_counts = input.template.chars().tuple_windows().counts();

    // Iterator
    for _ in 0..steps {
        bigram_counts = bigram_counts
            .iter()
            .flat_map(|((a, b), count)| {
                if let Some(c) = input.substitutions.get(&(*a, *b)) {
                    vec![((*a, *c), *count), ((*c, *b), *count)]
                } else {
                    vec![((*a, *b), *count)]
                }
            })
            .into_grouping_map()
            .sum();
    }

    // Summarise
    let mut counts: HashMap<char, usize> = bigram_counts
        .iter()
        .map(|((a, _), count)| (*a, *count))
        .into_grouping_map()
        .sum();
    *counts.get_mut(&input.template.chars().last().unwrap()).unwrap() += 1;

    if let MinMaxResult::MinMax(lower, upper) = counts.values().minmax() {
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

    #[test]
    fn example_day14_part2() {
        let input = parse_puzzle("day14.example");
        assert_eq!(1588, part2(&input, 10));
        assert_eq!(2188189693529, part2(&input, 40));
    }

    #[test]
    fn exec_day14_part2() {
        let input = parse_puzzle("day14.txt");
        println!("Day 14 Part 1 - {}", part2(&input, 10));
        println!("Day 14 Part 2 - {}", part2(&input, 40));
    }
}
