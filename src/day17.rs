use std::{ops::{RangeInclusive}};

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Probe {
  x: i32,
  y: i32,
  dx: i32,
  dy: i32,
}

pub fn step(Probe { x, y, dx, dy }: &Probe) -> Probe {
  Probe { 
    x: x + dx, 
    y: y + dy, 
    dx: (dx.abs() - 1).max(0) * dx.signum(), 
    dy: dy - 1
  }
}

pub struct ProbeIter {
  curr: Probe
}

impl Iterator for ProbeIter {
    type Item = Probe;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr = step(&self.curr);
        Some(self.curr.clone())
    }
}

impl Probe {
  fn iter(&self) -> ProbeIter {
    ProbeIter { curr: self.clone() }
  }
}

pub fn execute1((dx, dy): (i32, i32), x_target: &RangeInclusive<i32>, y_target: &RangeInclusive<i32>) -> Option<i32> {
  let mut max_y = 0;
  for probe in (Probe { x: 0, y: 0, dx, dy }).iter()  {
    let in_space = (0..200).contains(&probe.x) && (-100000..100000).contains(&probe.y);
    let in_target = x_target.contains(&probe.x) && y_target.contains(&probe.y);
    max_y = max_y.max(probe.y);
    if !in_space {
      return None
    } else if in_target {
      return Some(max_y)
    }
  }
  None
}

pub fn part1(x_target: RangeInclusive<i32>, y_target: RangeInclusive<i32>) -> i32 {
  (0..10000).cartesian_product(0..10000).flat_map(|v| execute1(v, &x_target, &y_target)).max().unwrap()
}

pub fn part2(x_target: RangeInclusive<i32>, y_target: RangeInclusive<i32>) -> usize {
  (-1000..1000).cartesian_product(-1000..1000).flat_map(|v| execute1(v, &x_target, &y_target)).count()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  pub fn examples_day17_part1() {
    assert_eq!(45, part1(20..=30, -10..=-5));
  }

  #[test]
  pub fn exec_day17_part1() {
    println!("Day 17 Part 1 - {}", part1(48..=70, -189..=-148));
  }

  #[test]
  pub fn examples_day17_part2() {
    assert_eq!(112, part2(20..=30, -10..=-5));
  }

  #[test]
  pub fn exec_day17_part2() {
    println!("Day 17 Part 2 - {}", part2(48..=70, -189..=-148));
  }
}