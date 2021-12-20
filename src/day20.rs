use std::convert::TryInto;

use itertools::Itertools;
use ndarray::{Array2, Axis};

use crate::util::read_file;

pub type Algorithm = Vec<bool>;
pub type Image = Array2<bool>;

pub struct Puzzle {
  algorithm: Algorithm,
  input: Image,
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

fn get_around(input: &Image, x: usize, y: usize, dx: i32, dy: i32) -> Option<bool> {
  let x: usize = ((x as i32) + dx).try_into().ok()?;
  let y: usize = ((y as i32) + dy).try_into().ok()?;
  input.get((y, x)).copied()
}

// target = 012345678
// source =xx0123456

fn region(input: &Image, x: usize, y: usize, default: bool) -> usize {
  (-2..=-0).cartesian_product(-2..=-0)
    .map(|(dy, dx)| {
      get_around(input, x, y, dx, dy).unwrap_or(default)
    })
    .fold(0, |acc,bit| (acc << 1) + bit as usize)
}

pub fn decompress(image: Image, algorithm: &Algorithm, default: bool) -> Image {
  if let [width, height] = image.shape() {
    Array2::from_shape_fn(
      (width + 2, height + 2),
      | (y, x) | {
        let region_value = region(&image, x, y, default);
        algorithm[region_value]
      },
    )
  } else {
    panic!();
  }
}

pub fn display(image: &Image) {
  image.axis_iter(Axis(0)).for_each(|line| {
    println!("{}", line.iter().map(|c| if *c { '#' } else {'.'}).join(""));
  });
  println!();
}

pub fn part1(puzzle: Puzzle) -> usize {
  let mut image = puzzle.input;

  let mut default = false;

  for _ in 0..2 {
    image = decompress(image, &puzzle.algorithm, default);
    if puzzle.algorithm[0] {
      default = !default;
    }
  }

  display(&image);

  image.iter().filter(|pixel| **pixel).count()
}

#[cfg(test)]
mod test {
  use ndarray::array;

  use super::*;

  #[test]
  fn test_parse_puzzle() {
    let puzzle = parse_puzzle("day20.example");

    assert_eq!(512, puzzle.algorithm.len());
    assert_eq!(true, puzzle.algorithm[34]);
    assert_eq!(true, puzzle.input[[0,0]]);
  }

  #[test]
  fn test_region() {
    let image = array![
      [false, false, false],
      [true, false, false],
      [false, true, false]
    ];

    assert_eq!(34, region(&image, 2, 2, false));
    assert_eq!(511, region(&image, 100, 100, true));
  }

  #[test]
  fn example_day20_part1() {
    let puzzle = parse_puzzle("day20.example");

    assert_eq!(35, part1(puzzle));
  }

  #[test]
  fn exec_day20_part1() {
    let puzzle = parse_puzzle("day20.txt");

    println!("Day 20 Part 1 - {}", part1(puzzle));
  }
}