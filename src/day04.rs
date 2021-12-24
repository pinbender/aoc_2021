use itertools::Itertools;
use std::collections::HashMap;

pub fn solve_1(lines: &[&str]) -> usize {
    solve_general(lines, true)
}

pub fn solve_2(lines: &[&str]) -> usize {
    solve_general(lines, false)
}

pub fn solve_general(lines: &[&str], first: bool) -> usize {
    let (seq, mut boards) = parse(lines);
    let mut evaluator = seq.into_iter().flat_map(|n| {
        boards
            .iter_mut()
            .filter_map(|board| {
                let won = !board.is_winner() && board.mark(n);
                won.then(|| board.score(n))
            })
            // Force evaluation to avoid escape
            .collect::<Vec<_>>()
    });
    if first {
        evaluator.next()
    } else {
        evaluator.last()
    }
    .expect("Nobody won")
}

lazy_static! {
    static ref WINNERS: Vec<u32> = vec![
        0b10000_10000_10000_10000_10000,
        0b01000_01000_01000_01000_01000,
        0b00100_00100_00100_00100_00100,
        0b00010_00010_00010_00010_00010,
        0b00001_00001_00001_00001_00001,
        0b11111_00000_00000_00000_00000,
        0b00000_11111_00000_00000_00000,
        0b00000_00000_11111_00000_00000,
        0b00000_00000_00000_11111_00000,
        0b00000_00000_00000_00000_11111
    ];
}

struct Board {
    numbers: Vec<u8>,
    index: HashMap<u8, u8>,
    marked: u32,
    won: bool,
}

impl Board {
    fn from<'a>(input: impl Iterator<Item = &'a str>) -> Board {
        let numbers = input
            .skip(1)
            .map(|l| l.split_whitespace().map(|n| n.parse::<u8>().unwrap()))
            .take(5)
            .flatten()
            .collect::<Vec<u8>>();
        let index = numbers
            .iter()
            .enumerate()
            .map(|(i, n)| (*n, i as u8))
            .collect::<HashMap<u8, u8>>();
        Board {
            numbers,
            index,
            marked: 0,
            won: false,
        }
    }
    fn is_winner(&self) -> bool {
        self.won
    }
    fn mark(&mut self, number: u8) -> bool {
        if let Some(lookup) = self.index.get(&number) {
            self.marked |= 1 << lookup;
            self.won = WINNERS.iter().any(|w| (self.marked & *w) == *w);
            self.won
        } else {
            false
        }
    }
    fn score(&self, last_number: u8) -> usize {
        self.numbers
            .iter()
            .enumerate()
            .filter_map(|(i, n)| {
                if (self.marked & (1 << i)) == 0 {
                    Some(*n as usize)
                } else {
                    None
                }
            })
            .sum::<usize>()
            * (last_number as usize)
    }
}

fn parse(lines: &[&str]) -> (Vec<u8>, Vec<Board>) {
    let mut reader = lines.into_iter().map(|s| *s);
    let sequence = reader
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect();

    let boards = reader
        .chunks(6)
        .into_iter()
        .map(|b| Board::from(b))
        .collect();
    (sequence, boards)
}
