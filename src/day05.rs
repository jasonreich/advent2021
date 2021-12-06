use std::collections::HashSet;
use crate::util::read_lines;

type Coord = (u32, u32);

fn range(start: u32, end: u32) -> impl Iterator<Item = u32> {
  if start <= end {
    start..=end
  } else {
    end..=start
  }
}

pub fn read_puzzle(filename: &str) -> impl Iterator<Item = Coord> {
  let intervals = read_lines(filename, | line | {
    let values: Vec<u32> = line.split(" -> ")
      .flat_map(| coord | coord.split(","))
      .map(| value | { value.parse().unwrap() })
      .collect();

    Some(((values[0], values[1]), (values[2], values[3])))
  }).unwrap();

  intervals.flat_map(|((sx, sy), (tx, ty))| -> Vec<Coord> {
    if sx == tx {
      range(sy, ty).map(|y| (sx, y)).collect()
    } else if sy == ty {
      range(sx, tx).map(|x| (x, sy)).collect()
    } else {
      vec![]
    }
  })
}

pub fn part1(input: impl Iterator<Item = Coord>) -> u32 {
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
  fn example_day05_part01() {
    assert_eq!(5, part1(read_puzzle("day05.example")));
  }

  #[test]
  fn exec_day05_part01() {
    println!("Day 05, Part 1: {}", part1(read_puzzle("day05.txt")));
  }
}