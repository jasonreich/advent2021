pub fn count_increases(input: impl Iterator<Item = i32>) -> i32 {
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

pub fn count_increases2(input: impl Iterator<Item = i32>) -> i32 {
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
  
  count_increases(windowed)
}

#[cfg(test)]
mod test {
  use super::*;
  use super::super::util::read_file;
  
  #[test]
  fn example() {
    let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let output = 7;

    assert_eq!(output, count_increases(input.into_iter()))
  }

  #[test]
  fn part1() {
    let i = read_file("day1.txt").unwrap();
    let count = count_increases(i);
    println!("Count: {}", count);
  }

  #[test]
  fn example2() {
    let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let output = 5;

    assert_eq!(output, count_increases2(input.into_iter()))
  }

  #[test]
  fn part2() {
    let i = read_file("day1.txt").unwrap();
    let count = count_increases2(i);
    println!("Count: {}", count);
  }
}