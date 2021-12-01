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

#[cfg(test)]
mod test {
  use super::count_increases;
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
}