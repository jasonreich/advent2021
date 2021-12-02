#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction {
    Forward,
    Up,
    Down,
}

pub fn parse_direction(input: String) -> Option<Direction> {
    match input.as_ref() {
        "forward" => Some(Direction::Forward),
        "up" => Some(Direction::Up),
        "down" => Some(Direction::Down),
        _ => None,
    }
}

pub fn parse_record(line: String) -> Option<Record> {
    let mut parts = line.split(' ');
    let direction_part = parts.next()?;
    let direction = parse_direction(direction_part.to_string())?; 
    let number_part = parts.next()?;
    let number = number_part.to_string().parse().ok()?;
    Some((direction, number))
}

pub type Record = (Direction, i32);

pub fn part1(input: impl Iterator<Item = Record>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for (direction, magnitude) in input {
        match direction {
            Direction::Forward => horizontal += magnitude,
            Direction::Up => depth -= magnitude,
            Direction::Down => depth += magnitude,
        }
    }

    horizontal * depth
}

pub fn part2(input: impl Iterator<Item = Record>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (direction, magnitude) in input {
        match direction {
            Direction::Forward => {
                horizontal += magnitude;
                depth += magnitude * aim;
            },
            Direction::Up => aim -= magnitude,
            Direction::Down => aim += magnitude,
        }
    }

    horizontal * depth
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::read_file;

    #[test]
    fn day02_parser() {
        let expected: Vec<(Direction, i32)> = vec![
            (Direction::Forward, 5),
            (Direction::Down, 5),
            (Direction::Forward, 8),
            (Direction::Up, 3),
            (Direction::Down, 8),
            (Direction::Forward, 2),
        ];
        let result: Vec<(Direction, i32)> = read_file("day2.example", parse_record).unwrap().collect();

        assert_eq!(expected, result);
    }

    #[test]
    fn example_day02_part1() {
        let input = read_file("day2.example", parse_record).unwrap();
        assert_eq!(150, part1(input))
    }

    #[test]
    fn exec_day02_part1() {
        let input = read_file("day2.txt", parse_record).unwrap();
        println!("Day 2 Part 1: {}", part1(input))
    }

    #[test]
    fn example_day02_part2() {
        let input = read_file("day2.example", parse_record).unwrap();
        assert_eq!(900, part2(input))
    }

    #[test]
    fn exec_day02_part2() {
        let input = read_file("day2.txt", parse_record).unwrap();
        println!("Day 2 Part 2: {}", part2(input))
    }
}