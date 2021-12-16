use std::{str::Chars, cell::Cell};

use itertools::Itertools;

pub fn parse_hexstring(input: &str) -> String {
    input.chars().map(|c| {
        let digit = c.to_digit(16).unwrap();
        format!("{:04b}", digit)
    }).join("")
}

struct BinarySequence<'a> {
    chars: Cell<Chars<'a>>
}

impl<'a> BinarySequence<'a> {
    fn new(input: Chars<'a>) -> BinarySequence<'a> {
        BinarySequence { chars: Cell::new(input) }
    }

    fn eat(&mut self, _: char, n: usize) -> Option<u64> {
        let substring: String = self.chars.get_mut().take(n).join("");
        // print!("{}", repeat(label).take(n).join(""));
        u64::from_str_radix(&substring, 2).ok()
    }

    fn take(&mut self, n: usize) -> String {
        self.chars.get_mut().take(n).join("")
    }
}

fn packet(sequence: &mut BinarySequence) -> Option<(u64, u64)> {
    let mut total_version = sequence.eat('V', 3)?;
    let packet_type = sequence.eat('T', 3)?;

    if packet_type == 4 {
        let mut literal = 0;
        // Literal
        loop {
            let keep_reading = sequence.eat('R', 1)?;
            literal <<= 4;
            literal += sequence.eat('L', 4)?;

            if keep_reading == 0 {
                break;
            }
        }
        Some((total_version, literal))
    } else {
        // Operator
        let length_type = sequence.eat('I', 1)?;
        let mut subpackets: Vec<u64> = vec![];

        if length_type == 0 {
            let bit_length = sequence.eat('l', 15)?;
            let substring = sequence.take(bit_length as usize);
            let mut subsequence = BinarySequence::new(substring.chars());
            while let Some(subpacket) = packet(&mut subsequence) {
                total_version += subpacket.0;
                subpackets.push(subpacket.1);
            }
        } else {
            let packet_count = sequence.eat('c', 11)?;
            for _ in 0..packet_count {
                let subpacket = packet(sequence).unwrap();
                total_version += subpacket.0;
                subpackets.push(subpacket.1);
            }
        }

        let result: u64 = match packet_type {
            0 => subpackets.iter().sum(),
            1 => subpackets.iter().product(),
            2 => *subpackets.iter().min().unwrap(),
            3 => *subpackets.iter().max().unwrap(),
            5 => (subpackets[0] > subpackets[1]) as u64,
            6 => (subpackets[0] < subpackets[1]) as u64,
            7 => (subpackets[0] == subpackets[1]) as u64,
            _ => unimplemented!(),
        };
        Some((total_version, result))
    }
}

pub fn part1(input: String) -> u64 {
    // println!("{}", input);
    let mut sequence = BinarySequence::new(input.chars());
    let result = packet(&mut sequence);
    // println!();
    result.unwrap().0
}

pub fn part2(input: String) -> u64 {
    // println!("{}", input);
    let mut sequence = BinarySequence::new(input.chars());
    let result = packet(&mut sequence);
    // println!();
    result.unwrap().1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_hexstring() {
        assert_eq!("0001", parse_hexstring("1"));
        assert_eq!("00111000000000000110111101000101001010010001001000000000", parse_hexstring("38006F45291200"));
        assert_eq!("11101110000000001101010000001100100000100011000001100000", parse_hexstring("EE00D40C823060"));
    }

    #[test]
    fn example_day16_part1() {
        assert_eq!(6, part1(parse_hexstring("D2FE28")));
        assert_eq!(7 + 2 + 4 + 1, part1(parse_hexstring("EE00D40C823060")));
        assert_eq!(16, part1(parse_hexstring("8A004A801A8002F478")));
        assert_eq!(12, part1(parse_hexstring("620080001611562C8802118E34")));
        assert_eq!(23, part1(parse_hexstring("C0015000016115A2E0802F182340")));
        assert_eq!(31, part1(parse_hexstring("A0016C880162017C3686B18A3D4780")));
    }

    #[test]
    fn exec_day16_part1() {
        let input = "420D4900B8F31EFE7BD9DA455401AB80021504A2745E1007A21C1C862801F54AD0765BE833D8B9F4CE8564B9BE6C5CC011E00D5C001098F11A232080391521E4799FC5BB3EE1A8C010A00AE256F4963B33391DEE57DA748F5DCC011D00461A4FDC823C900659387DA00A49F5226A54EC378615002A47B364921C201236803349B856119B34C76BD8FB50B6C266EACE400424883880513B62687F38A13BCBEF127782A600B7002A923D4F959A0C94F740A969D0B4C016D00540010B8B70E226080331961C411950F3004F001579BA884DD45A59B40005D8362011C7198C4D0A4B8F73F3348AE40183CC7C86C017997F9BC6A35C220001BD367D08080287914B984D9A46932699675006A702E4E3BCF9EA5EE32600ACBEADC1CD00466446644A6FBC82F9002B734331D261F08020192459B24937D9664200B427963801A094A41CE529075200D5F4013988529EF82CEFED3699F469C8717E6675466007FE67BE815C9E84E2F300257224B256139A9E73637700B6334C63719E71D689B5F91F7BFF9F6EE33D5D72BE210013BCC01882111E31980391423FC4920042E39C7282E4028480021111E1BC6310066374638B200085C2C8DB05540119D229323700924BE0F3F1B527D89E4DB14AD253BFC30C01391F815002A539BA9C4BADB80152692A012CDCF20F35FDF635A9CCC71F261A080356B00565674FBE4ACE9F7C95EC19080371A009025B59BE05E5B59BE04E69322310020724FD3832401D14B4A34D1FE80233578CD224B9181F4C729E97508C017E005F2569D1D92D894BFE76FAC4C5FDDBA990097B2FBF704B40111006A1FC43898200E419859079C00C7003900B8D1002100A49700340090A40216CC00F1002900688201775400A3002C8040B50035802CC60087CC00E1002A4F35815900903285B401AA880391E61144C0004363445583A200CC2C939D3D1A41C66EC40";
        println!("Day 16 Part 1 - {}", part1(parse_hexstring(input)));
    }

    #[test]
    fn example_day16_part2() {
        assert_eq!(3, part2(parse_hexstring("C200B40A82")));
        assert_eq!(0, part2(parse_hexstring("F600BC2D8F")));
        assert_eq!(0, part2(parse_hexstring("9C005AC2F8F0")));
        assert_eq!(1, part2(parse_hexstring("9C0141080250320F1802104A08")));
    }

    #[test]
    fn exec_day16_part2() {
        let input = "420D4900B8F31EFE7BD9DA455401AB80021504A2745E1007A21C1C862801F54AD0765BE833D8B9F4CE8564B9BE6C5CC011E00D5C001098F11A232080391521E4799FC5BB3EE1A8C010A00AE256F4963B33391DEE57DA748F5DCC011D00461A4FDC823C900659387DA00A49F5226A54EC378615002A47B364921C201236803349B856119B34C76BD8FB50B6C266EACE400424883880513B62687F38A13BCBEF127782A600B7002A923D4F959A0C94F740A969D0B4C016D00540010B8B70E226080331961C411950F3004F001579BA884DD45A59B40005D8362011C7198C4D0A4B8F73F3348AE40183CC7C86C017997F9BC6A35C220001BD367D08080287914B984D9A46932699675006A702E4E3BCF9EA5EE32600ACBEADC1CD00466446644A6FBC82F9002B734331D261F08020192459B24937D9664200B427963801A094A41CE529075200D5F4013988529EF82CEFED3699F469C8717E6675466007FE67BE815C9E84E2F300257224B256139A9E73637700B6334C63719E71D689B5F91F7BFF9F6EE33D5D72BE210013BCC01882111E31980391423FC4920042E39C7282E4028480021111E1BC6310066374638B200085C2C8DB05540119D229323700924BE0F3F1B527D89E4DB14AD253BFC30C01391F815002A539BA9C4BADB80152692A012CDCF20F35FDF635A9CCC71F261A080356B00565674FBE4ACE9F7C95EC19080371A009025B59BE05E5B59BE04E69322310020724FD3832401D14B4A34D1FE80233578CD224B9181F4C729E97508C017E005F2569D1D92D894BFE76FAC4C5FDDBA990097B2FBF704B40111006A1FC43898200E419859079C00C7003900B8D1002100A49700340090A40216CC00F1002900688201775400A3002C8040B50035802CC60087CC00E1002A4F35815900903285B401AA880391E61144C0004363445583A200CC2C939D3D1A41C66EC40";
        println!("Day 16 Part 2 - {}", part2(parse_hexstring(input)));
    }
}
