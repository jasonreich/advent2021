use std::collections::HashMap;
use std::hash::Hash;

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

pub fn increase<K: Eq+Hash+Copy>(m: &mut HashMap<K, usize>, k: &K, i: usize) {
    let old_value = *m.get(k).unwrap_or(&0);
    m.insert(*k, old_value + i);
}

pub fn part2(input: &Puzzle, steps: u32) -> usize {
    let mut bigram_counts: HashMap<Bigram, usize> = HashMap::new();

    // Initialise
    let template: Vec<char> = input.template.chars().collect();
    for i in 0..(template.len() - 1) {
        let bigram = (template[i], template[i+1]);
        increase(&mut bigram_counts, &bigram, 1);
    }

    for _ in 0..steps {
        let mut new_counts: HashMap<Bigram, usize> = HashMap::new();
        for (bigram, count) in bigram_counts {
            if let Some(c) = input.substitutions.get(&bigram) {
                let (a, b) = bigram;
                increase(&mut new_counts, &(a, *c), count);
                increase(&mut new_counts, &(*c, b), count);
            } else {
                increase(&mut new_counts, &bigram, count);
            }
        }
        bigram_counts = new_counts;
    }

    let mut counts: HashMap<char, usize> = HashMap::new();
    for ((a, _), count) in bigram_counts {
        increase(&mut counts, &a, count);
    }
    increase(&mut counts, template.last().unwrap(), 1);

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
