pub fn part1(input: impl Iterator<Item = i32>) -> i32 {
  let mut count: i32 = 0;
  let mut last: i32 = i32::MAX;

  for value in input {
    if value > last {
      count += 1
    }
    last = value
  }
  
  count
}

pub fn part2(input: impl Iterator<Item = i32>) -> i32 {
  let mut oldest = 0;
  let mut middle = 0;
  let mut newest = 0;

  let windowed = input.enumerate().filter_map(| (index, input) | {
    oldest = middle;
    middle = newest;
    newest = input;
    if index >= 2 {
      Some(oldest + middle + newest)
    } else {
      None
    }
  });
  
  part1(windowed)
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::util::read_file;
  
  #[test]
  fn example_part1() {
    let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let output = 7;

    assert_eq!(output, part1(input.into_iter()))
  }

  #[test]
  fn test_part1() {
    let i = read_file("day1.txt").unwrap();
    let count = part1(i);
    println!("Count: {}", count);
  }

  #[test]
  fn example_part2() {
    let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let output = 5;

    assert_eq!(output, part2(input.into_iter()))
  }

  #[test]
  fn exec_part2() {
    let i = read_file("day1.txt").unwrap();
    let count = part2(i);
    println!("Count: {}", count);
  }
}