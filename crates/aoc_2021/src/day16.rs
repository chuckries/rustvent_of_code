use aoc_common::file_string;

struct PacketReader {
    bytes: Vec<u8>,
    bit_idx: usize,
    data_idx: usize,
    position: usize,
}

impl PacketReader {
    fn new(input: &str) -> PacketReader {
        let mut bytes: Vec<u8> = Vec::with_capacity(input.len() / 2);

        for chunk in input.as_bytes().chunks(2) {
            let byte = (Self::hex_to_byte(chunk[0]) << 4) |
                (Self::hex_to_byte(chunk[1]));
            bytes.push(byte);
        }

        PacketReader {
            bytes,
            bit_idx: 7,
            data_idx: 0,
            position: 0,
        }
    }

    fn read_packet(&mut self) -> Packet {
        let version = self.read_bits(3);
        let type_id = self.read_bits(3);

        let mut packet = Packet {
            version,
            type_id,
            literal: 0,
            subs: Vec::new()
        };

        if type_id == 4 {
            let literal = self.read_literal();
            packet.literal = literal;
        } else {
            self.read_subpackets(&mut packet);
        }

        packet
    }

    fn read_subpackets(&mut self, packet: &mut Packet) {
        let length_type_id = self.read_bits(1);
        if length_type_id == 0 {
            let bit_count = self.read_bits(15);
            let offset = self.position + bit_count;
            while self.position < offset {
                packet.subs.push(self.read_packet());
            }
        } else {
            let count = self.read_bits(11);
            for _ in 0..count {
                packet.subs.push(self.read_packet());
            }
        }
    }

    fn read_literal(&mut self) -> usize {
        let mut literal = 0;
        
        loop {
            let chunk = self.read_bits(5);
            literal = (literal << 4) | (chunk & 0x0F);
            if (chunk & 0x10) == 0 {
                break;
            }
        }

        literal
    }

    fn read_bits(&mut self, count: usize) -> usize {
        let mut result: usize = 0;

        for _ in 0..count {
            result = (result << 1) | ((self.bytes[self.data_idx] as usize & (1 << self.bit_idx)) >> self.bit_idx);
            if self.bit_idx == 0 {
                self.data_idx += 1;
                self.bit_idx = 7;
            } else {
                self.bit_idx -= 1;
            }
        }

        self.position += count;
        result
    }

    fn hex_to_byte(hex: u8) -> u8 {
        if hex <= b'9' {
            hex - b'0'
        } else {
            hex - b'A' + 10
        }
    }
}

struct Packet {
    version: usize,
    type_id: usize,
    literal: usize,
    subs: Vec<Packet>
}

impl Packet {
    fn sum_versions(&self) -> usize {
        self.version + self.subs.iter().map(|s| s.sum_versions()).sum::<usize>()
    }

    fn evaluate(&self) -> usize {
        match self.type_id {
            0 => self.subs.iter().map(|s| s.evaluate()).sum::<usize>(),
            1 => self.subs.iter().map(|s| s.evaluate()).reduce(|accum, item| accum * item).unwrap(),
            2 => self.subs.iter().map(|s| s.evaluate()).min().unwrap(),
            3 => self.subs.iter().map(|s| s.evaluate()).max().unwrap(),
            4 => self.literal,
            5 => if self.subs[0].evaluate() > self.subs[1].evaluate() { 1 } else { 0 },
            6 => if self.subs[0].evaluate() < self.subs[1].evaluate() { 1 } else { 0 },
            7 => if self.subs[0].evaluate() == self.subs[1].evaluate() { 1 } else { 0 },
            _ => panic!()
        }
    }
}

fn input() -> Packet {
    let mut reader = PacketReader::new(&file_string("inputs/day16.txt"));
    reader.read_packet()
}

#[test]
fn part1() {
    let answer = input().sum_versions();
    assert_eq!(answer, 893);
}

#[test]
fn part2() {
    let answer = input().evaluate();
    assert_eq!(answer, 4358595186090);
}