use std::{ops::{RangeInclusive, RangeBounds, Bound}, collections::HashSet};

use itertools::Itertools;
use regex::{Captures, Regex};

use crate::util::read_lines;

pub struct Instruction {
    is_on: bool,
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
    z_range: RangeInclusive<i32>,
}

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(
        r"^(on|off) x=([-0-9]+)\.\.([-0-9]+),y=([-0-9]+)\.\.([-0-9]+),z=([-0-9]+)\.\.([-0-9]+)$"
    )
    .unwrap();
}

fn make_range(captures: &Captures, first_index: usize) -> Option<RangeInclusive<i32>> {
    let lower = captures[first_index].parse().ok()?;
    let upper = captures[first_index + 1].parse().ok()?;
    Some(lower..=upper)
}

pub fn parse_puzzle(file: &str) -> impl Iterator<Item = Instruction> {
    read_lines(file, |line| {
        let captures = LINE_REGEX.captures(line.as_str())?;
        Some(Instruction {
            is_on: &captures[1] == "on",
            x_range: make_range(&captures, 2)?,
            y_range: make_range(&captures, 4)?,
            z_range: make_range(&captures, 6)?,
        })
    })
    .unwrap()
}

fn limit_range(input: RangeInclusive<i32>) -> RangeInclusive<i32> {
  match (input.start_bound(), input.end_bound()) {
    (Bound::Included(lower), Bound::Included(upper)) => {
      *lower.max(&-50)..=*upper.min(&50)
    }
    _ => unimplemented!()
  }
}

pub fn part1(input: impl Iterator<Item = Instruction>) -> usize {
  let mut on_set = HashSet::new();
  for Instruction {is_on, x_range, y_range, z_range} in input {
    let x_range = limit_range(x_range);
    let y_range = limit_range(y_range);
    let z_range = limit_range(z_range);
    for ((x, y), z) in x_range.cartesian_product(y_range).cartesian_product(z_range) {
      if is_on {
        on_set.insert((x, y, z));
      } else {
        on_set.remove(&(x, y, z));
      }
    }
  }
  on_set.len()
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_parse_puzzle() {
        let input = parse_puzzle("day22.example").collect_vec();
        assert_eq!(22, input.len());
        assert_eq!(true, input[0].is_on);
        assert_eq!(967, input[21].x_range.clone().collect_vec()[0]);
    }

    #[test]
    fn example_day22_part1() {
      let input = parse_puzzle("day22.example");
      assert_eq!(590784, part1(input));
    }

    #[test]
    fn exec_day22_part1() {
      let input = parse_puzzle("day22.txt");
      println!("Day 22 Part 1 - {}", part1(input));
    }
}
