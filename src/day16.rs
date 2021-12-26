pub fn solve(lines: &[&str]) -> (usize, usize) {
    let solution = lines.iter().map(|l| {
        let mut stream = Bitstream::new(l);
        let solutions = parse_packets(&mut stream);
        println!("solutions: {:?}", solutions);
        solutions
    }).last().unwrap();

    solution
}

#[derive(Default)]
pub struct Bitstream {
    payload: Vec<u8>,
    index: usize,
    available: u8,
    buffer: u16
}

impl Bitstream {
    pub fn new(payload: &str) -> Bitstream {
        let payload: Vec<u8> = payload.as_bytes()
            .chunks(2)
            .map(|c| u8::from_str_radix(std::str::from_utf8(c).unwrap(), 16).unwrap())
            .collect();
        Self {
            payload: payload,
            .. Default::default()
        }
    }

    fn _load(&mut self) {
        if self.index < self.payload.len() {
            let next = self.payload.get(self.index).copied().unwrap_or_default();
            //println!("load: {:08b}", next);
            self.buffer = (self.buffer << 8) | (next as u16);
            self.index += 1;
            self.available += 8;
        }
    }

    pub fn len(&self) -> usize { self.payload.len() * 8 }
    pub fn remaining(&self) -> usize { self.len() - self.index * 8 + self.available as usize }

    pub fn read(&mut self, mut bit_count: u8) -> u64 {
        let mut result = 0;

        while bit_count > 8 {
            result = (result << 8) | self.read(8);
            bit_count -= 8;
        }

        if self.available < bit_count {
            self._load();
        }
        let bits = if self.available > 0 {
            let from_buffer = bit_count.min(self.available);
            let offset = self.available - from_buffer;
            let mask = (1 << from_buffer) - 1;
            let bits = self.buffer & (mask << offset);
            self.buffer ^= bits;
            self.available -= from_buffer;
            (bits >> offset) << (bit_count - from_buffer)
        }
        else { 0 };
        let result = (result << bit_count) | (bits as u64);
        //println!("read({0}) = {2:01$b}", bit_count, bit_count as usize, result);
        result
    }
    pub fn read_value(&mut self) -> u64 {
        let mut result = 0;
        loop {
            let flag = self.read(1);
            let next = self.read(4);
            result = (result << 4) | next;
            if flag == 0 {
                return result;
            }
        }
    }
}

fn parse_packets(stream: &mut Bitstream) -> (usize, usize) 
{
    let version = stream.read(3) as usize;
    let packet_type = parse_packet_type(stream.read(3) as u8);
    if let PacketType::Constant = packet_type {
        let constant = stream.read_value();
        //println!("constant: {}/{}", version, constant);
        (version, constant as usize)
    }
    else {
        let mut version_sum = version;
        let mut values = Vec::new();
        // Operator
        let length_type = stream.read(1);
        //println!("operator: {}/{}", version, length_type);

        if length_type == 0 {
            let sub_size = stream.read(15) as usize;
            //println!("sub size: {}", sub_size);
            let until = stream.remaining() - sub_size;
            while stream.remaining() > until {
                let (sub_version, sub_value) = parse_packets(stream);
                version_sum += sub_version;
                values.push(sub_value);
            }
        } else {
            let sub_count = stream.read(11);
            //println!("sub count: {}", sub_count);
            for _ in 0..sub_count {
                let (sub_version, sub_value) = parse_packets(stream);
                version_sum += sub_version;
                values.push(sub_value);
            }
        }
        let value = match packet_type {
            PacketType::Sum => values.into_iter().sum(),
            PacketType::Product => values.into_iter().product(),
            PacketType::Minimum => values.into_iter().min().unwrap(),
            PacketType::Maximum => values.into_iter().max().unwrap(),
            PacketType::Constant => unreachable!("Constant encountered during operator"),
            PacketType::Greater => if values[0] > values[1] { 1 } else { 0 },
            PacketType::Less => if values[0] < values[1] { 1 } else { 0 },
            PacketType::Equal => if values[0] == values[1] { 1 } else { 0 }
        };
        (version_sum, value)
    }
}

#[repr(u8)]
#[allow(dead_code)]
enum PacketType {
    Sum = 0,
    Product,
    Minimum,
    Maximum,
    Constant,
    Greater,
    Less,
    Equal
}

fn parse_packet_type(packet_type: u8) -> PacketType {
    if packet_type > PacketType::Equal as u8 {
        panic!("Invalid packet type encountered");
    }
    unsafe { std::mem::transmute(packet_type) }
}
