use std::{fs, str::Chars};

use itertools::Itertools;

#[derive(Debug)]
enum PacketType {
    Literal {
        value: usize,
        length: usize,
    },
    Operator {
        operation: usize,
        packets: Vec<Packet>,
        length: usize,
    },
}

impl PacketType {
    fn from(iter: &mut Chars, type_id: usize) -> Self {
        match type_id {
            4 => PacketType::parse_literal_value(iter),
            _ => PacketType::parse_operator(iter, type_id),
        }
    }

    fn len(&self) -> usize {
        match self {
            PacketType::Literal { length, .. } => *length,
            PacketType::Operator { length, .. } => *length,
        }
    }

    fn parse_literal_value(iter: &mut Chars) -> Self {
        let mut counter = 6; // version + type id
        let mut buf = "".to_string();

        loop {
            counter += 5;

            let group = read_bits(iter, 5);
            buf.push_str(&group[1..]);

            if &group[0..1] == "0" {
                break;
            }
        }

        let value = usize::from_str_radix(&buf, 2).unwrap();
        PacketType::Literal {
            value,
            length: counter,
        }
    }

    fn parse_operator(iter: &mut Chars, type_id: usize) -> Self {
        let mut counter = 7; // ver + type id + len type

        // 0 -> number of bits
        // 1 -> number of packets
        let length_type = parse_bits(iter, 1);

        let mut packets = Vec::new();

        match length_type {
            0 => {
                let packets_len = parse_bits(iter, 15);
                let mut n = 0;

                counter += 15 + packets_len;

                while n < packets_len {
                    let packet = Packet::from(iter);
                    n += packet.len();

                    packets.push(packet);
                }
            }
            1 => {
                let subpackets_num = parse_bits(iter, 11);
                counter += 11;

                for _ in 0..subpackets_num {
                    packets.push(Packet::from(iter));
                }

                counter += &packets.iter().map(|packet| packet.len()).sum::<usize>();
            }
            _ => panic!(),
        }

        PacketType::Operator {
            operation: type_id,
            packets,
            length: counter,
        }
    }

    fn process(&self) -> usize {
        if let PacketType::Literal { value, .. } = self {
            return *value;
        }

        if let PacketType::Operator {
            operation, packets, ..
        } = self
        {
            let values = packets.iter().map(|p| p.process()).collect_vec();

            return match operation {
                0 => values.iter().sum::<usize>(),
                1 => values.iter().product::<usize>(),
                2 => *values.iter().min().unwrap(),
                3 => *values.iter().max().unwrap(),
                5 => {
                    if values[0] > values[1] {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if values[0] < values[1] {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if values[0] == values[1] {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!(),
            };
        }

        panic!()
    }
}

#[derive(Debug)]
struct Packet {
    version: usize,
    contents: PacketType,
}

impl Packet {
    fn from(iter: &mut Chars) -> Self {
        let version = parse_bits(iter, 3);
        let type_id = parse_bits(iter, 3);

        Packet {
            version,
            contents: PacketType::from(iter, type_id),
        }
    }

    fn len(&self) -> usize {
        self.contents.len()
    }

    fn sum_version(&self) -> usize {
        let mut output = self.version;

        if let PacketType::Operator { packets, .. } = &self.contents {
            for packet in packets {
                output += packet.sum_version();
            }
        }

        output
    }

    fn process(&self) -> usize {
        self.contents.process()
    }
}

fn main() {
    let input = fs::read_to_string("res/day16.txt")
        .unwrap()
        .trim()
        .chars()
        .map(|c| format!("{:04b}", usize::from_str_radix(&c.to_string(), 16).unwrap()))
        .collect::<String>();

    let mut iter = input.chars();
    let packet = Packet::from(&mut iter);

    println!("Part 1 answer: {}", packet.sum_version());
    println!("Part 2 answer: {}", packet.process());
}

fn parse_bits(iter: &mut Chars, n: usize) -> usize {
    let buf = read_bits(iter, n);
    usize::from_str_radix(&buf, 2).unwrap()
}

fn read_bits(iter: &mut Chars, n: usize) -> String {
    let mut buf = "".to_string();

    for _ in 0..n {
        buf.push(iter.next().unwrap());
    }

    buf
}
