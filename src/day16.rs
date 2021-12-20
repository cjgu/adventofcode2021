use common::load_file;
use itertools::max;
use itertools::min;
use itertools::Itertools;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("day16 <file>");
        process::exit(1);
    }
    let lines: Vec<String> = load_file(&args[1]);

    let line = &lines[0];

    let bits = hex_to_bits(line);

    let (packets, _) = parse(bits);

    println!("Part A: {:}", sum_packet_versions(&packets[0]));
    println!("Part A: {:}", evaluate(&packets[0]));
}

fn sum_packet_versions(packet: &Packet) -> u64 {
    match &packet.content {
        PacketContent::Operator { subpackets } => {
            let sum: u64 = subpackets
                .into_iter()
                .map(|p| sum_packet_versions(&p))
                .sum();
            sum + packet.version
        }
        _ => packet.version,
    }
}

fn evaluate(packet: &Packet) -> u64 {
    match &packet.content {
        PacketContent::Literal { value } => {
            return *value;
        }
        PacketContent::Operator { subpackets } => {
            let sub: Vec<u64> = subpackets.into_iter().map(|p| evaluate(p)).collect();

            match packet.packet_type {
                PacketType::Sum => sub.into_iter().sum(),
                PacketType::Product => sub.into_iter().product(),
                PacketType::Max => max(sub.into_iter()).unwrap(),
                PacketType::Min => min(sub.into_iter()).unwrap(),
                PacketType::GreaterThan => {
                    if sub[0] > sub[1] {
                        1
                    } else {
                        0
                    }
                }
                PacketType::LessThan => {
                    if sub[0] < sub[1] {
                        1
                    } else {
                        0
                    }
                }
                PacketType::EqualTo => {
                    if sub[0] == sub[1] {
                        1
                    } else {
                        0
                    }
                }
                _ => {
                    unreachable!()
                }
            }
        }
    }
}

#[derive(Debug)]
enum PacketType {
    Sum = 0,
    Product = 1,
    Min = 2,
    Max = 3,
    Literal = 4,
    GreaterThan = 5,
    LessThan = 6,
    EqualTo = 7,
}

impl TryFrom<u64> for PacketType {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PacketType::Sum),
            1 => Ok(PacketType::Product),
            2 => Ok(PacketType::Min),
            3 => Ok(PacketType::Max),
            4 => Ok(PacketType::Literal),
            5 => Ok(PacketType::GreaterThan),
            6 => Ok(PacketType::LessThan),
            7 => Ok(PacketType::EqualTo),
            _ => Err("Invalid Packet Type ID"),
        }
    }
}

fn hex_to_bits(input: &str) -> Vec<u8> {
    let bin = hex_to_bin(input);
    let bits = bin_to_bits(&bin);

    bits
}

fn hex_to_bin(input: &str) -> Vec<u8> {
    input
        .chars()
        .map(|c| c.to_digit(16).unwrap() as u8)
        .collect_vec()
}

fn bin_to_bits(input: &Vec<u8>) -> Vec<u8> {
    let mut res = Vec::new();

    for bin in input {
        let bits = vec![
            (bin & 0b1000) >> 3,
            (bin & 0b0100) >> 2,
            (bin & 0b0010) >> 1,
            (bin & 0b0001),
        ];

        res.extend(bits.into_iter());
    }

    res
}

#[derive(Debug)]
struct Packet {
    version: u64,
    packet_type: PacketType,

    content: PacketContent,
}

#[derive(Debug)]
enum PacketContent {
    Literal { value: u64 },
    Operator { subpackets: Vec<Packet> },
}

#[derive(Debug)]
enum State {
    PacketVersion,
    PacketTypeId,
    Literal,
    Operator,
}

fn bits_to_u(bits: &Vec<u8>) -> u64 {
    let mut acc: u64 = 0;

    for bit_index in (0..bits.len()).rev() {
        acc += bits[bit_index] as u64 * (1 << (bits.len() - bit_index - 1))
    }

    acc
}

fn parse(input: Vec<u8>) -> (Vec<Packet>, usize) {
    parse_internal(input, None, 0)
}

fn parse_internal(input: Vec<u8>, limit: Option<usize>, level: usize) -> (Vec<Packet>, usize) {
    // packet format
    // packet version: 3 bit
    // packet type id: 3 bit
    // type id 4 => literal value, contains a single binary number
    // groups for 4 bits, prefixed 1, or 0 for last group
    // type id X => Operator.
    // 1 or more subpackets

    // length type id 1 bit
    // 0
    // 15 bits total length of the sub packets
    // 1
    // 11 bits with number of sub packes

    let mut state = State::PacketVersion;
    let mut packets = Vec::new();

    let mut bit_pos = 0;
    let mut cur_packet_version = 0;
    let mut cur_packet_type_id = 0;

    loop {
        if bit_pos + 3 > input.len() && bits_to_u(&input[bit_pos..].to_vec()) == 0 {
            break;
        }

        if let Some(limit) = limit {
            if packets.len() >= limit {
                return (packets, bit_pos);
            }
        }

        match state {
            State::PacketVersion => {
                if bit_pos + 3 > input.len() {
                    break;
                }
                cur_packet_version = bits_to_u(&input[bit_pos..bit_pos + 3].to_vec());

                bit_pos += 3;
                state = State::PacketTypeId
            }
            State::PacketTypeId => {
                cur_packet_type_id = bits_to_u(&input[bit_pos..bit_pos + 3].to_vec());
                bit_pos += 3;
                match cur_packet_type_id {
                    4 => state = State::Literal,
                    _ => state = State::Operator,
                }
            }
            State::Operator => {
                let length_type_bit = input[bit_pos];

                bit_pos += 1;
                match length_type_bit {
                    0 => {
                        let bit_length = bits_to_u(&input[bit_pos..bit_pos + 15].to_vec());
                        bit_pos += 15;

                        let (subpackets, _) = parse_internal(
                            input[bit_pos..bit_pos + bit_length as usize].to_vec(),
                            None,
                            level + 1,
                        );
                        bit_pos += bit_length as usize;

                        packets.push(Packet {
                            version: cur_packet_version,
                            packet_type: PacketType::try_from(cur_packet_type_id).unwrap(),
                            content: PacketContent::Operator { subpackets },
                        });
                        state = State::PacketVersion;
                    }
                    1 => {
                        let num_sub_packets = bits_to_u(&input[bit_pos..bit_pos + 11].to_vec());
                        bit_pos += 11;

                        let mut subpackets: Vec<Packet> = vec![];
                        for _ in 0..num_sub_packets {
                            let (parsed_packets, end_pos) =
                                parse_internal(input[bit_pos..].to_vec(), Some(1), level + 1);
                            subpackets.extend(parsed_packets.into_iter());
                            bit_pos = bit_pos + end_pos;
                        }
                        packets.push(Packet {
                            version: cur_packet_version,
                            packet_type: PacketType::try_from(cur_packet_type_id).unwrap(),
                            content: PacketContent::Operator { subpackets },
                        });
                        state = State::PacketVersion;
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }
            State::Literal => {
                // continously take 5 bits
                let mut literal: Vec<u8> = Vec::new();
                loop {
                    let group = input[bit_pos..bit_pos + 5].to_vec();
                    bit_pos += 5;
                    literal.extend(group[1..5].iter());

                    if group[0] == 0 {
                        // collect, end of packet
                        let value = bits_to_u(&literal);
                        packets.push(Packet {
                            version: cur_packet_version,
                            packet_type: PacketType::try_from(cur_packet_type_id).unwrap(),
                            content: PacketContent::Literal { value },
                        });

                        state = State::PacketVersion;
                        break;
                    }
                }
            }
        };

        if bit_pos >= input.len() {
            break;
        }

        if level == 0 && packets.len() >= 1 {
            break;
        }
    }

    (packets, bit_pos)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bits_to_u() {
        assert_eq!(bits_to_u(&vec![0]), 0);
        assert_eq!(bits_to_u(&vec![1]), 1);
        assert_eq!(bits_to_u(&vec![1, 1]), 3);
        assert_eq!(bits_to_u(&vec![1, 0, 0]), 4);
        assert_eq!(bits_to_u(&vec![0, 0, 0, 1, 0, 1, 0, 0]), 20);
    }

    #[test]
    fn test_hex_to_bits() {
        assert_eq!(hex_to_bits("0"), vec![0, 0, 0, 0]);
        assert_eq!(hex_to_bits("1"), vec![0, 0, 0, 1]);
        assert_eq!(hex_to_bits("A"), vec![1, 0, 1, 0]);
        assert_eq!(hex_to_bits("12"), vec![0, 0, 0, 1, 0, 0, 1, 0]);
    }

    #[test]
    fn test_hex_to_bin() {
        assert_eq!(hex_to_bin("0"), vec![0]);
        assert_eq!(hex_to_bin("1"), vec![1]);
        assert_eq!(hex_to_bin("A"), vec![10]);
    }

    #[test]
    fn test_parse_literal() {
        let input = vec![
            1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0,
        ];
        let (packets, _) = parse(input);
        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].version, 6);

        if let PacketContent::Literal { value: v } = &packets[0].content {
            assert_eq!(*v, 2021);
        }
    }

    #[test]
    fn test_parse_operator() {
        let hex = "38006F45291200";
        let bits = hex_to_bits(hex);
        let (packets, _) = parse(bits);

        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].version, 1);

        if let PacketContent::Operator { subpackets: x } = &packets[0].content {
            assert_eq!(x.len(), 2);

            assert_eq!(x[0].version, 6);
            if let PacketContent::Literal { value: v } = &x[0].content {
                assert_eq!(*v, 10);
            }

            assert_eq!(x[1].version, 2);
            if let PacketContent::Literal { value: v } = &x[1].content {
                assert_eq!(*v, 20);
            }
        }
    }

    #[test]
    fn test_parse_operator_num_packets() {
        let input = vec![
            1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0,
        ];

        let (packets, _) = parse(input);

        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].version, 7);

        if let PacketContent::Operator { subpackets: x } = &packets[0].content {
            assert_eq!(x.len(), 3)
        }
    }

    #[test]
    fn test_nested_operators() {
        let hex = "A0016C880162017C3686B18A3D4780";

        let bits = hex_to_bits(hex);
        let (packets, _) = parse(bits);

        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].version, 5);

        if let PacketContent::Operator { subpackets: x } = &packets[0].content {
            assert_eq!(x.len(), 1);
            if let PacketContent::Operator { subpackets: y } = &x[0].content {
                assert_eq!(y.len(), 1);
                if let PacketContent::Operator { subpackets: z } = &y[0].content {
                    assert_eq!(z.len(), 5);
                }
            }
        }
    }
}
