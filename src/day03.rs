pub fn line_parser(input: String) -> Option<Vec<bool>> {
  Some(input.chars().map(| c | {
    match c {
      '0' => false,
      '1' => true,
      _ => unimplemented!()
    }
  }).collect())
}

fn most_common(input: Vec<bool>) -> Option<bool> {
  let input_size = input.len();
  let count_true = input.iter().filter(| value | **value).count();
  if count_true == input_size || count_true == 0 {
    None
  } else {
    Some(count_true * 2 > input_size)
  }
}

pub fn part1(input: Vec<Vec<bool>>) -> u32 {
  let line_length = input.iter().next().unwrap().len();
  let mut epsilon: u32 = 0;
  for i in 0..line_length {
    if most_common(input.iter().map(| line | line[i] ).collect()).unwrap() {
      epsilon += 1;
    }
    epsilon = epsilon << 1;
  }
  epsilon = epsilon >> 1;
  let gamma = !epsilon & ((1 << line_length) - 1);
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
    assert_eq!(Some(true), most_common(vec![true, true, false, true]));
    assert_eq!(Some(false), most_common(vec![false, false, false, true]));
    assert_eq!(None, most_common(vec![false, false, false, false]));
    assert_eq!(None, most_common(vec![true, true, true, true]));
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