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
        .unwrap(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_puzzle() {
        let input = read_puzzle("day12.example1");
        assert_eq!(2, input.get_vec("start").unwrap().len());
    }
}
