pub fn solve(lines: &[&str]) -> (usize, usize) {
    let result = lines.iter().map(|l| {
        let stream = Bitstream::new(l);
        let mut result = 0;
        parse_packets(&mut stream, |version, _| result += version);
        println!("pt1: {}", result);
        result
    }).last().unwrap();

    (result, 0)
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
        let payload = payload[0..].into();
        let length = payload.size();
        Self {
            payload,
            .. Default::default()
        }
    }

    fn _load(&mut self) {
        if (index < self.payload.size()) {
            let next = self.payload.get(self.index).copied().unwrap_or_default();
            self.buffer = (self.buffer << 8) | (next as u16);
            self.index += 1;
            self.available += 8;
        }
    }

    pub fn size(&self) -> { self.payload.size() * 8 }
    pub fn remaining(&self) -> { self.size() - self.index * 8 + available }

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
            let bits = self.buffer >> (self.available - from_buffer);
            self.buffer >>= self.available - from_buffer;
            self.available -= from_buffer;
            bits
        }
        else { 0 }
        (result << bit_count) | (bits as u64)
    }
    pub fn read_value(&mut self) -> u64 {
        let mut result = 0;
        loop {
            let flag = self.read(1);
            let next = self.read(4);
            result = (result << 4) | next;
            if (flag == 0) {
                return result;
            }
        }
    }
}

fn parse_packets(stream: &mut Bitstream, 
    processor: &FnMut(version, Option<usize>)) 
{
    let version = stream.read(3);
    let packet_type = stream.read(3);
    if (packet_type == 4) {
        let constant = stream.read_value();
        processor(version, Some(constant));
    }
    else {
        // Operator
        processor(version, None);
        let length_type = stream.read(1);
        if length_type == 0 {
            let sub_size = stream.read(15);
            let until = stream.remaining() - sub_size;
            while stream.remaining() > until {
                parse_packets(stream, processor);
            }
        } else {
            let sub_count = stream.read(11);
            for _ in 0..sub_count {
                parse_packets(stream, processor);
            }
        }
    }
}
