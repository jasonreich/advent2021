#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction {
    Forward,
    Up,
    Down,
}

pub fn parse_direction(input: String) -> Direction {
    match input.as_ref() {
        "forward" => Direction::Forward,
        "up" => Direction::Up,
        "down" => Direction::Down,
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::read_file;

    #[test]
    fn parser() {
        let expected: Vec<(Direction, i32)> = vec![
            (Direction::Forward, 5),
            (Direction::Down, 5),
            (Direction::Forward, 8),
            (Direction::Up, 3),
            (Direction::Down, 8),
            (Direction::Forward, 2),
        ];
        let result: Vec<(Direction, i32)> = read_file("day2.example", |line| {
            let mut parts = line.split(' ');
            let direction_part = parts.next()?;
            let number_part = parts.next()?;
            let number = number_part.to_string().parse().ok()?;
            Some((parse_direction(direction_part.to_string()), number))
        }).unwrap().collect();

        assert_eq!(expected, result);
    }
}