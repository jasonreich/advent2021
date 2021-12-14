use std::collections::HashMap;

use crate::util::read_file;

pub struct Puzzle {
    template: String,
    substitutions: HashMap<(char, char), char>,
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
}
