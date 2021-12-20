use itertools::Itertools;
use ndarray::{Array2};

use crate::util::read_file;

pub struct Puzzle {
  algorithm: Vec<bool>,
  input: Array2<bool>,
}

pub fn parse_puzzle(file: &str) -> Puzzle {
  let mut lines = read_file(file).unwrap();

  let algorithm = lines.next().unwrap().chars().map(|c| c == '#').collect_vec();

  lines.next();

  let image = lines.collect_vec();
  let width = image[0].len();
  let height = image.len();

  let input = Array2::from_shape_vec(
    (width, height), 
    image.iter().flat_map(|line| line.chars().collect_vec()).map(|c| c == '#').collect_vec()
  ).unwrap();

  Puzzle { algorithm, input }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_parse_puzzle() {
    let puzzle = parse_puzzle("day20.example");

    assert_eq!(512, puzzle.algorithm.len());
    assert_eq!(true, puzzle.input[[0,0]]);
  }
}