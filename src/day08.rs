use std::collections::HashMap;
use itertools::Itertools;

pub fn solve_1(lines: &[&str]) -> usize {
    let notes = parse(lines);
    notes.iter().map(|n| 
        n.output.iter().filter_map(|o| 
            match o.count_ones() {
                2 | 3 | 4 | 7 => Some(()),
                _ => None
            }
        ).count()
    ).sum()
}

pub fn solve_2(lines: &[&str]) -> usize {
    let notes = parse(lines);
    notes.iter().map(Note::solve).sum()
}

fn parse(lines: &[&str]) -> Vec<Note> {
    lines.iter().map(|l| Note::new(l).unwrap()).collect()
}

fn parse_char(c: char) -> u8 {
    match c {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        _ => unreachable!("Invalid signal")
    }
}

fn parse_to_bits(s: &str) -> u8 {
    s.chars().map(|c| 1 << parse_char(c)).fold(0, |n, c| {n | c})
}

struct Note {
    input: Vec<u8>,
    output: Vec<u8>
}

impl Note {
    fn new(line: &str) -> Option<Note> {
        let mut parts = line.split(" | ").map(|part|
            part.split_whitespace().map(parse_to_bits).collect::<Vec<_>>()
        );
        let input = parts.next()?;
        let output = parts.next()?;
        Some(Self { input, output })
    }
    fn solve(&self) -> usize {
        let all = self.input.iter()
            .chain(self.output.iter())
            .copied()
            .unique()
            .collect::<Vec<u8>>();
        let one = all.iter().copied().find(|&s| s.count_ones() == 2).unwrap();
        let seven = all.iter().copied().find(|&s| s.count_ones() == 3).unwrap();
        let four = all.iter().copied().find(|&s| s.count_ones() == 4).unwrap();
        let eight = all.iter().copied().find(|&s| s == 0x7f).unwrap();
        let corner = four ^ one;
        let mut six_bits = all.clone();
        six_bits.retain(|&s| s.count_ones() == 6);
        assert_eq!(six_bits.len(), 3);
        let zero = six_bits.iter().copied().find(|&s| (s & corner) != corner).unwrap();
        let nine = six_bits.iter().copied().find(|&s| (s != zero) && ((s & one) == one)).unwrap();
        let six = six_bits.iter().copied().find(|&s| (s != zero) && (s != nine)).unwrap();
        let bottom_right = six & one;
        let top_right = bottom_right ^ one;
        let mut five_bits = all.clone();
        five_bits.retain(|&s| s.count_ones() == 6);
        assert_eq!(five_bits.len(), 3);
        let two = five_bits.iter().copied().find(|&s| (s & one) == top_right).unwrap();
        let three = five_bits.iter().copied().find(|&s| (s & one) == one).unwrap();
        let five = five_bits.iter().copied().find(|&s| (s & one) == bottom_right).unwrap();

        let digits = [zero, one, two, three, four, five, six, seven, eight, nine].into_iter()
            .enumerate()
            .map(|(i, n)| (n,i as usize))
            .collect::<HashMap<u8, usize>>();

        self.output.iter()
            .fold(0, |total, digit| total * 10 + digits.get(digit).unwrap())
    }
}