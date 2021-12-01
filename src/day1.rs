pub fn count_increases(input: Vec<i32>) -> i32 {
  let mut count: i32 = 0;
  let mut last: i32 = i32::MAX;

  for value in input.iter() {
    if *value > last {
      count += 1
    }
    last = *value
  }
  
  count
}

#[cfg(test)]
mod test {
  #[test]
  fn example() {
    let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let output = 7;

    assert_eq!(output, super::count_increases(input))
  }
}