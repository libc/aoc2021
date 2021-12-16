use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Bit {
    Zero,
    One,
}

#[derive(Debug)]
enum Packet {
    Operator {
        version: i64,
        packet_type: i64,
        subpackets: Vec<Packet>,
    },
    Literal {
        version: i64,
        number: i64,
    },
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/day16.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let bitstring: Vec<Bit> = contents
        .chars()
        .flat_map(|c| match c {
            '0' => vec![Bit::Zero, Bit::Zero, Bit::Zero, Bit::Zero],
            '1' => vec![Bit::Zero, Bit::Zero, Bit::Zero, Bit::One],
            '2' => vec![Bit::Zero, Bit::Zero, Bit::One, Bit::Zero],
            '3' => vec![Bit::Zero, Bit::Zero, Bit::One, Bit::One],
            '4' => vec![Bit::Zero, Bit::One, Bit::Zero, Bit::Zero],
            '5' => vec![Bit::Zero, Bit::One, Bit::Zero, Bit::One],
            '6' => vec![Bit::Zero, Bit::One, Bit::One, Bit::Zero],
            '7' => vec![Bit::Zero, Bit::One, Bit::One, Bit::One],
            '8' => vec![Bit::One, Bit::Zero, Bit::Zero, Bit::Zero],
            '9' => vec![Bit::One, Bit::Zero, Bit::Zero, Bit::One],
            'A' => vec![Bit::One, Bit::Zero, Bit::One, Bit::Zero],
            'B' => vec![Bit::One, Bit::Zero, Bit::One, Bit::One],
            'C' => vec![Bit::One, Bit::One, Bit::Zero, Bit::Zero],
            'D' => vec![Bit::One, Bit::One, Bit::Zero, Bit::One],
            'E' => vec![Bit::One, Bit::One, Bit::One, Bit::Zero],
            'F' => vec![Bit::One, Bit::One, Bit::One, Bit::One],
            _ => vec![],
        })
        .collect();

    let packets = Parser::new(&bitstring).parse();
    println!("answer1 {:?}", sum_versions(&packets));
    println!("answer2 {:?}", eval(&packets[0]));

    Ok(())
}

struct Parser {
    bits: VecDeque<Bit>,
}

impl Parser {
    fn new(bits: &Vec<Bit>) -> Parser {
        Parser {
            bits: bits.iter().cloned().collect(),
        }
    }

    fn parse(&mut self) -> Vec<Packet> {
        let mut packets = Vec::new();
        loop {
            let packet = self.parse_packet();
            if packet.is_none() {
                return packets;
            }

            packets.push(packet.unwrap());
        }
    }

    fn parse_packet(&mut self) -> Option<Packet> {
        if self.bits.len() < 8 {
            return None;
        }

        let version = self.bits(3);
        if version.is_none() {
            return None;
        }
        let version = version.unwrap();

        let packet_type = self.bits(3).unwrap();
        if packet_type == 4 {
            let num = self.parse_literal();
            if num.is_none() {
                return None;
            }

            return Some(Packet::Literal {
                version,
                number: num.unwrap(),
            });
        }

        let len_type = self.bits(1).unwrap();

        let subpackets = if len_type == 0 {
            let len = self.bits(15).unwrap();
            let subvec = (0..len)
                .filter_map(|_| self.bits.pop_front())
                .collect::<Vec<Bit>>();
            Parser::new(&subvec).parse()
        } else {
            let len = self.bits(11).unwrap();
            (0..len)
                .filter_map(|_| self.parse_packet())
                .collect::<Vec<Packet>>()
        };

        Some(Packet::Operator {
            version,
            packet_type,
            subpackets,
        })
    }

    fn parse_literal(&mut self) -> Option<i64> {
        let mut out = 0;
        loop {
            let section = self.bits(5);
            if section.is_none() {
                return None;
            }
            let section = section.unwrap();

            out = (out << 4) + (section & 15);

            if (section >> 4) == 0 {
                return Some(out);
            }
        }
    }

    fn bits(&mut self, n: usize) -> Option<i64> {
        (0..n).fold(None, |acc, _| {
            let nb = self.bits.pop_front();
            match nb {
                None => None,
                Some(b) => Some((acc.unwrap_or(0) << 1) + if b == Bit::One { 1 } else { 0 }),
            }
        })
    }
}

fn sum_versions(packets: &Vec<Packet>) -> i64 {
    packets.iter().map(|p| match p {
        Packet::Operator { version, subpackets, .. } => {
            version + sum_versions(subpackets)
        },
        Packet::Literal { version, .. }=> {
            *version
        }
    }).sum()
}

fn eval(packet: &Packet) -> i64 {
    match packet {
        Packet::Literal { number, .. } => {
            *number
        },
        Packet::Operator { packet_type, subpackets, .. } => {
            let mut evaled = subpackets.iter().map(|p| eval(p));

            match packet_type {
                0 => evaled.sum(),
                1 => evaled.reduce(|a, b| a * b).unwrap(),
                2 => evaled.min().unwrap(),
                3 => evaled.max().unwrap(),
                5 => {
                    let a = evaled.next().unwrap();
                    let b = evaled.next().unwrap();
                    if a > b {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    let a = evaled.next().unwrap();
                    let b = evaled.next().unwrap();
                    if a < b {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    let a = evaled.next().unwrap();
                    let b = evaled.next().unwrap();
                    if a == b {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("unimplemented")
            }
        }
    }
}
