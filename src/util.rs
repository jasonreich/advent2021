use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines, Result};

pub fn read_file<T>(path: &str, line_parser: fn (String) -> Option<T>) -> Result<impl Iterator<Item = T>> {
  let mut full_path = current_dir()?;
  full_path.push("fixtures");
  full_path.push(path);
  let file = File::open(full_path)?;
  let lines_of_file: Lines<BufReader<File>> = BufReader::new(file).lines();
  let lines_as_parsed = lines_of_file
    .filter_map(move | maybe_line | {
      let line = maybe_line.ok()?;
      line_parser(line)
    });
  Ok(lines_as_parsed)
}

pub fn read_file_as_i32(path: &str) -> Result<impl Iterator<Item = i32>> {
  read_file(path, |line| line.parse().ok())
}

#[cfg(test)]
mod test {
  use super::read_file_as_i32;

  #[test]
  fn simple_file() {
    let i = read_file_as_i32("myfile.txt").unwrap();
    let as_vec: Vec<i32> = i.collect();
    assert_eq!(vec![1,2,3,4,5], as_vec);
  }
}