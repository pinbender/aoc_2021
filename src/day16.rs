pub fn solve(lines: &[&str]) -> (usize, usize) {
    let result = lines.iter().map(|l| {
        let stream = Bitstream::new(l);
        0
    }).last().unwrap();

    (result, 0)
}

pub struct Bitstream {
    payload: Vec<u8>,
    index: usize,
    available: u8,
    buffer: u16
}

impl Bitstream {
    pub fn new(payload: &str) -> Bitstream {
        Self {
            payload: payload[0..].into(),
            index: 0,
            available: 0,
            buffer: 0
        }
    }

    fn _load(&mut self) {
        let next = self.payload.get(self.index).copied().unwrap_or_default();
        self.buffer = (self.buffer << 8) | (next as u16);
        self.index += 1;
        self.available += 8;
    }

    fn read(&mut self, mut bit_count: u8) -> u64 {
        let mut result = 0;
        while bit_count > 8 {
            result = (result << 8) | self.read(8);
            bit_count -= 8;
        }

        if self.available < bit_count {
            self._load();
        }
        let bits = self.buffer >> (self.available - bit_count);
        self.buffer >>= self.available - bit_count;
        self.available -= bit_count;
        (result << bit_count) | (bits as u64)
    }
}