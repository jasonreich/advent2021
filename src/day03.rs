pub fn line_parser(input: String) -> Option<Vec<bool>> {
    Some(
        input
            .chars()
            .map(|c| match c {
                '0' => false,
                '1' => true,
                _ => unimplemented!(),
            })
            .collect(),
    )
}

fn most_common(input: Vec<bool>) -> Option<bool> {
    let input_size = input.len();
    let count_true = input.iter().filter(|value| **value).count();
    if count_true * 2 == input_size {
        None
    } else {
        Some(count_true * 2 > input_size)
    }
}

pub fn part1(input: Vec<Vec<bool>>) -> u32 {
    let line_length = input[0].len();
    let mut epsilon: u32 = 0;
    for i in 0..line_length {
        if most_common(input.iter().map(|line| line[i]).collect()).unwrap() {
            epsilon += 1;
        }
        epsilon <<= 1;
    }
    epsilon >>= 1;
    let gamma = !epsilon & ((1 << line_length) - 1);
    epsilon * gamma
}

pub fn part2(input: Vec<Vec<bool>>, use_least_common: bool) -> u32 {
    let line_length = input[0].len();
    let mut filter: Vec<bool> = vec![];
    let mut result = 0;

    for i in 0..line_length {
        let filtered_input: Vec<Vec<bool>> = input
            .iter()
            .filter(|line| filter.iter().zip(line.iter()).all(|(x, y)| x == y))
            .cloned()
            .collect();
        let common = if filtered_input.len() > 1 {
            let indexed_input = filtered_input.iter().map(|line| line[i]).collect();
            use_least_common != most_common(indexed_input).unwrap_or(true)
        } else {
            filtered_input[0][i]
        };
        filter.push(common);
        result = (result << 1) + (common as u32);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::read_lines;

    #[test]
    fn test_day03_parser() {
        let mut input = read_lines("day03.example", line_parser).unwrap();
        assert_eq!(
            vec![false, false, true, false, false],
            input.next().unwrap()
        );
    }

    #[test]
    fn test_most_common() {
        assert_eq!(Some(true), most_common(vec![true, true, false, true]));
        assert_eq!(Some(false), most_common(vec![false, false, false, true]));
        assert_eq!(Some(false), most_common(vec![false, false, false, false]));
        assert_eq!(Some(true), most_common(vec![true, true, true, true]));
        assert_eq!(None, most_common(vec![false, false, true, true]));
    }

    #[test]
    fn example_day03_part1() {
        let input = read_lines("day03.example", line_parser).unwrap().collect();
        assert_eq!(198, part1(input));
    }

    #[test]
    fn exec_day03_part1() {
        let input = read_lines("day03.txt", line_parser).unwrap().collect();
        println!("Day 03, Part 1: {}", part1(input));
    }

    #[test]
    fn example_day03_part2() {
        let input: Vec<Vec<bool>> = read_lines("day03.example", line_parser).unwrap().collect();
        assert_eq!(23, part2(input.clone(), false));
        assert_eq!(10, part2(input.clone(), true));
    }

    #[test]
    fn exec_day03_part2() {
        let input: Vec<Vec<bool>> = read_lines("day03.txt", line_parser).unwrap().collect();
        println!(
            "Day 03, Part 1: {}",
            part2(input.clone(), false) * part2(input, true)
        );
    }
}
