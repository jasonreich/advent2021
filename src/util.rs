use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines, Result};

pub fn read_file(path: &str) -> Result<impl Iterator<Item = i32>> {
  let mut full_path = current_dir()?;
  full_path.push("fixtures");
  full_path.push(path);
  let file = File::open(full_path)?;
  let lines_of_file: Lines<BufReader<File>> = BufReader::new(file).lines();
  let lines_as_parsed = lines_of_file
    .filter_map(| maybe_line | {
      let line = maybe_line.ok()?;
      line.parse().ok()
    });
  Ok(lines_as_parsed)
}

#[cfg(test)]
mod test {
  use super::read_file;

  #[test]
  fn simple_file() {
    let i = read_file("myfile.txt").unwrap();
    let as_vec: Vec<i32> = i.collect();
    assert_eq!(vec![1,2,3,4,5], as_vec);
  }
}