pub type Shoal = Vec<u32>;

pub fn next_shoal_tick(input: Shoal) -> Shoal {
  input.iter().flat_map(| fish | {
    if *fish == 0 {
      vec![6, 8]
    } else {
      vec![*fish - 1]
    }
  }).collect()
}

pub fn part1(input: Shoal, iterations: u32) -> usize {
  let mut shoal = input;
  for _ in 0..iterations {
    shoal = next_shoal_tick(shoal);
  }
  shoal.len()
}

#[cfg(test)]
mod test {
  use crate::util::read_lines;
  use super::*;

  #[test]
  fn test_next_tick() {
    assert_eq!(vec![2,3,2,0,1], next_shoal_tick(vec![3,4,3,1,2]));
    assert_eq!(vec![1,2,1,6,8,0], next_shoal_tick(vec![2,3,2,0,1]));
  }

  #[test]
  fn example_day06_part1() {
    assert_eq!(26, part1(vec![3,4,3,1,2], 18));
    assert_eq!(5934, part1(vec![3,4,3,1,2], 80));
  }

  #[test]
  fn exec_day06_part1() {
    let input: Vec<u32> = read_lines("day06.txt", move |line| {
      Some(line.split(",").map(|value| value.parse().unwrap()).collect())
    }).unwrap().next().unwrap();

    println!("Day 06 Part 1 - {}", part1(input, 80));
  }
}