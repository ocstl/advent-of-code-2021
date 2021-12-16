const FILE: &str = "inputs/day16.txt";

type Version = u32;
type Value = u64;

const fn to_bits(b: u8) -> [u32; 4] {
    match b {
        b'0' => [0, 0, 0, 0],
        b'1' => [0, 0, 0, 1],
        b'2' => [0, 0, 1, 0],
        b'3' => [0, 0, 1, 1],
        b'4' => [0, 1, 0, 0],
        b'5' => [0, 1, 0, 1],
        b'6' => [0, 1, 1, 0],
        b'7' => [0, 1, 1, 1],
        b'8' => [1, 0, 0, 0],
        b'9' => [1, 0, 0, 1],
        b'A' => [1, 0, 1, 0],
        b'B' => [1, 0, 1, 1],
        b'C' => [1, 1, 0, 0],
        b'D' => [1, 1, 0, 1],
        b'E' => [1, 1, 1, 0],
        b'F' => [1, 1, 1, 1],
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OpType {
    Sum,
    Product,
    Minimum,
    Maximum,
    Value,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl From<u32> for OpType {
    fn from(type_id: u32) -> Self {
        match type_id {
            0 => OpType::Sum,
            1 => OpType::Product,
            2 => OpType::Minimum,
            3 => OpType::Maximum,
            4 => OpType::Value,
            5 => OpType::GreaterThan,
            6 => OpType::LessThan,
            7 => OpType::EqualTo,
            _ => unimplemented!(),
        }
    }
}

trait BitIterator: Iterator<Item = u32> {
    fn group(&mut self, n: usize) -> u32 {
        self.take(n).fold(0, |acc, bit| (acc << 1) + bit)
    }
}

impl<T: Iterator<Item = u32>> BitIterator for T {}

#[derive(Debug, Clone)]
enum Packet {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

impl Packet {
    fn new<I: BitIterator>(iter: &mut I) -> Self {
        let version = iter.group(3);
        let type_id = OpType::from(iter.group(3));
        match type_id {
            OpType::Value => Packet::Literal(LiteralPacket::new(version, type_id, iter)),
            _ => Packet::Operator(OperatorPacket::new(version, type_id, iter)),
        }
    }

    fn versions_iter(&self) -> Box<dyn Iterator<Item = Version> + '_> {
        match self {
            Packet::Literal(p) => Box::new(p.versions_iter()),
            Packet::Operator(p) => Box::new(p.versions_iter()),
        }
    }

    fn value(&self) -> Value {
        match self {
            Packet::Literal(p) => p.value(),
            Packet::Operator(p) => p.value(),
        }
    }
}

impl<T: AsRef<str>> From<T> for Packet {
    fn from(input: T) -> Self {
        let mut bit_iterator = input.as_ref().bytes().flat_map(to_bits);
        Packet::new(&mut bit_iterator)
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct LiteralPacket {
    version: Version,
    type_id: OpType,
    value: u64,
}

impl LiteralPacket {
    fn new<I: BitIterator>(version: u32, type_id: OpType, iter: &mut I) -> Self {
        let mut value: Value = 0;
        let mut not_last = 1;

        while not_last == 1 {
            not_last = iter.group(1);
            value <<= 4;
            value += Value::from(iter.group(4));
        }

        LiteralPacket {
            version,
            type_id,
            value,
        }
    }

    fn versions_iter(&self) -> impl Iterator<Item = Version> {
        std::iter::once(self.version)
    }

    fn value(&self) -> Value {
        self.value
    }
}

#[derive(Debug, Clone)]
struct OperatorPacket {
    version: Version,
    type_id: OpType,
    packets: Vec<Packet>,
}

impl OperatorPacket {
    fn new<I: BitIterator>(version: u32, type_id: OpType, iter: &mut I) -> Self {
        let packets = if iter.group(1) == 0 {
            Self::by_length(iter)
        } else {
            Self::by_nbr(iter)
        };

        OperatorPacket {
            version,
            type_id,
            packets,
        }
    }

    fn versions_iter(&self) -> impl Iterator<Item = Version> + '_ {
        std::iter::once(self.version).chain(self.packets.iter().flat_map(Packet::versions_iter))
    }

    fn value(&self) -> Value {
        let mut values = self.packets.iter().map(Packet::value);
        match self.type_id {
            OpType::Sum => values.sum(),
            OpType::Product => values.product(),
            OpType::Minimum => values.min().unwrap(),
            OpType::Maximum => values.max().unwrap(),
            OpType::Value => unreachable!(),
            OpType::GreaterThan => {
                if values.next().unwrap() > values.next().unwrap() {
                    1
                } else {
                    0
                }
            }
            OpType::LessThan => {
                if values.next().unwrap() < values.next().unwrap() {
                    1
                } else {
                    0
                }
            }
            OpType::EqualTo => {
                if values.next().unwrap() == values.next().unwrap() {
                    1
                } else {
                    0
                }
            }
        }
    }

    #[allow(clippy::needless_collect)]
    fn by_length<I: BitIterator>(iter: &mut I) -> Vec<Packet> {
        let l = iter.group(15);

        // Have to collect, sadly, given the recursive nature of the
        // construction. Otherwise, we get a E0275 compiler error. So it is not
        // "needless".
        let bits: Vec<u32> = iter.take(l as usize).collect();
        let mut iter = bits.into_iter().peekable();

        let mut packets = Vec::new();
        while iter.peek().is_some() {
            packets.push(Packet::new(&mut iter));
        }

        packets
    }

    fn by_nbr<I: BitIterator>(iter: &mut I) -> Vec<Packet> {
        let l = iter.group(11);
        let mut packets = Vec::new();

        for _ in 0..l {
            packets.push(Packet::new(iter));
        }

        packets
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;

    // Decode the structure of your hexadecimal-encoded BITS transmission; what
    // do you get if you add up the version numbers in all packets?
    let outermost = Packet::from(input.trim());
    let part1: Version = outermost.versions_iter().sum();
    println!("Part 1: {}", part1);

    // What do you get if you evaluate the expression represented by your
    // hexadecimal-encoded BITS transmission?
    let part2 = outermost.value();
    println!("Part 2: {}", part2);

    Ok(())
}
