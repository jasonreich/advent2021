pub fn line_parser(input: String) -> Option<Vec<bool>> {
  Some(input.chars().map(| c | {
    match c {
      '0' => false,
      '1' => true,
      _ => unimplemented!()
    }
  }).collect())
}

fn most_common(input: Vec<bool>) -> bool {
  let input_size = input.len();
  let count_true = input.iter().filter(| value | **value).count();
  let double_count_true = count_true * 2;
  if double_count_true == input_size {
    panic!()
  } else {
    count_true * 2 > input_size
  }
}

pub fn part1(input: Vec<Vec<bool>>) -> u32 {
  let mut epsilon: u32 = 0;
  let mut gamma: u32 = 0;
  let first = input.iter().next().unwrap();
  for i in 0..first.len() {
    if most_common(input.iter().map(| line | line[i] ).collect()) {
      epsilon += 1;
    } else {
      gamma += 1;
    }
    epsilon = epsilon << 1;
    gamma = gamma << 1;
  }
  epsilon = epsilon >> 1;
  gamma = gamma >> 1;
  epsilon * gamma
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::util::read_file;

  #[test]
  fn test_day03_parser() {
    let mut input = read_file("day03.example", line_parser).unwrap();
    assert_eq!(vec![false, false, true, false, false], input.next().unwrap());
  }

  #[test]
  fn test_most_common() {
    assert_eq!(true, most_common(vec![true, true, false, true]));
    assert_eq!(false, most_common(vec![false, false, false, true]));
  }

  #[test]
  fn example_day03_part1() {
    let input = read_file("day03.example", line_parser).unwrap().collect();
    assert_eq!(198, part1(input));
  }

  #[test]
  fn exec_day03_part1() {
    let input = read_file("day03.txt", line_parser).unwrap().collect();
    println!("Day 03, Part 1: {}", part1(input));
  }
}