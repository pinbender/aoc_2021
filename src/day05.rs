use itertools::Itertools;
use std::collections::HashSet;
use super::bitmap::Bitmap;

pub fn solve_1(lines: &[&str]) -> usize {
    let part_lines = lines.iter()
        .map(|&l| Line::parse(l))
        .filter(|line| line.is_horizontal() || line.is_vertical());
    solve_generic(part_lines)
}

pub fn solve_2(lines: &[&str]) -> usize {
    let part_lines = lines.iter()
        .map(|&l| Line::parse(l));
    solve_generic(part_lines)
}

// Note: Input verified as <1024 for all lines
const BOARD_WIDTH: usize = 1 << 10;
const BOARD_HEIGHT: usize = 1 << 10;

fn solve_generic(lines: impl Iterator<Item=Line>) -> usize {
    // Note: This could arguably be a hashset instead.
    let mut board = Bitmap::<bool>::new((BOARD_WIDTH as isize, BOARD_HEIGHT as isize));
    let mut intersections = HashSet::new();
    lines.map(|line| {
            let inc = line.direction();
            let mut coord = line.start;
            loop {
                if !board.set(&coord, true).unwrap() {
                    intersections.insert(coord);
                }
                if coord == line.end {
                    break;
                }
                coord.0 += inc.0;
                coord.1 += inc.1;
            }
        }).for_each(drop);
    intersections.len()
}

struct Line {
    start: (isize,isize),
    end: (isize,isize)
}

impl Line {
    fn parse(input: &str) -> Line {
        let mut parts = input.split_whitespace();
        let start = parts.next().unwrap()
            .split(',').map(|n| n.parse::<isize>().unwrap()).collect_tuple().unwrap();
        parts.next();
        let end = parts.next().unwrap()
            .split(',').map(|n| n.parse::<isize>().unwrap()).collect_tuple().unwrap();
        Line { start, end }
    }
    fn is_horizontal(&self) -> bool { self.start.1 == self.end.1 }
    fn is_vertical(&self) -> bool { self.start.0 == self.end.0 }
    fn direction(&self) -> (isize, isize) { 
        ((self.end.0 - self.start.0).signum(),
         (self.end.1 - self.start.1).signum())
    }
}
