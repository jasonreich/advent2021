pub fn part1(mut input: impl Iterator<Item = i32>) -> i32 {
  let mut count: i32 = 0;
  let mut last: i32 = input.next().unwrap();

  for value in input {
    if value > last {
      count += 1
    }
    last = value
  }
  
  count
}

pub fn part2(mut input: impl Iterator<Item = i32>) -> i32 {
  let mut oldest = 0;
  let mut middle = input.next().unwrap();
  let mut newest = input.next().unwrap();

  let windowed = input.map(| input | {
    oldest = middle;
    middle = newest;
    newest = input;
    oldest + middle + newest
  });
  
  part1(windowed)
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::util::read_file_as_i32;
  
  static EXAMPLE_INPUT: &[i32; 10] = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

  #[test]
  fn example_day01_part1() {
    assert_eq!(7, part1(EXAMPLE_INPUT.iter().cloned()))
  }

  #[test]
  fn test_day01_part1() {
    let i = read_file_as_i32("day01.txt").unwrap();
    let count = part1(i);
    println!("Day 1 Part 1 – Count: {}", count);
  }

  #[test]
  fn example_day01_part2() {
    assert_eq!(5, part2(EXAMPLE_INPUT.iter().cloned()));
  }

  #[test]
  fn exec_day01_part2() {
    let i = read_file_as_i32("day01.txt").unwrap();
    let count = part2(i);
    println!("Day 1 Part 2 – Count: {}", count);
  }
}