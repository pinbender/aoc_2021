use crate::bitmap::Bitmap;

use std::collections::HashSet;
use std::mem;

use itertools::Itertools;

pub fn solve_1(lines: &[&str]) -> usize {
    let mut board = parse(lines);
    (0..100).map(|_| step_1(&mut board)).sum()
}

pub fn solve_2(lines: &[&str]) -> usize {
    let mut board = parse(lines);
    let size = board.size();
    let total_flash = (size.0 * size.1) as usize;
    let synced = (1..)
        .filter_map(|n| {
            (step_1(&mut board) == total_flash).then(|| n)
        }).next().unwrap();
    synced
}

fn parse(lines: &[&str]) -> Bitmap::<u8> {
    let mut board = Bitmap::<u8>::new((10,10));
    let digits = lines.iter()
        .flat_map(|l| 
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
            );
    board.fill(digits);
    board
}

fn step_1(board: &mut Bitmap::<u8>) -> usize {
    let mut flashers = HashSet::new();
    board.mutate(|c, &n| {
        let new_result = n + 1;
        if new_result > 9 {
            flashers.insert(c);
        }
        new_result
    });
    let mut new_flashers = flashers.clone();
    let mut next_flashers = HashSet::new();
    while !new_flashers.is_empty() {
        for c in new_flashers.drain() {
            for neighbor_c in get_neighbors(c) {
                if let Some(neighbor) = board.get_mut(&neighbor_c) {
                    *neighbor += 1;
                    if *neighbor == 10 {
                        next_flashers.insert(neighbor_c);
                        flashers.insert(neighbor_c);
                    }
                }
            }
        }
        mem::swap(&mut new_flashers, &mut next_flashers);
    }
    flashers.iter().map(|c| {
        board.set(c, 0)
    }).for_each(drop);
    
    //print_board(&board);
    //println!("\n");

    flashers.len()
}

fn get_neighbors(coord: (isize, isize)) -> impl Iterator<Item=(isize, isize)> {
    [
        (coord.0 - 1, coord.1 - 1),
        (coord.0, coord.1 - 1),
        (coord.0 + 1, coord.1 - 1),
        (coord.0 - 1, coord.1),
        (coord.0 + 1, coord.1),
        (coord.0 - 1, coord.1 + 1),
        (coord.0, coord.1 + 1),
        (coord.0 + 1, coord.1 + 1)
    ].into_iter()
}

fn print_board(board: &Bitmap<u8>) {
    let numbers = board.visit()
        .map(|(_, &n)| char::from_digit(n as u32, 10).unwrap())
        .chunks(board.size().0 as usize)
        .into_iter()
        .map(|line| line.collect::<String>())
        .join("\n");
    println!("{}", numbers)
}