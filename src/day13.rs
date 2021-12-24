use crate::bitmap::Bitmap;
use itertools::Itertools;
use std::collections::HashSet;

pub fn solve(lines: &[&str]) -> (usize, usize) {
    let (coords, folds) = parse(lines);
    let first_coords = bend(coords.into_iter(), &folds[0]);
    let first_result = first_coords.len();
    //print(first_coords.iter().cloned());
    let full_result = folds[1..]
        .iter()
        .fold(first_coords, |coords, f| bend(coords.into_iter(), f));
    
    print(full_result.iter().cloned());
    (first_result, full_result.len())
}

fn parse(lines: &[&str]) -> (HashSet<(isize, isize)>, Vec<(bool, isize)>) {
    let blank = lines.iter().position(|l| l.is_empty()).unwrap();

    let coords = lines[0..blank]
        .iter()
        .flat_map(|l| l.split(','))
        .map(|n| n.parse::<isize>().unwrap())
        .tuples()
        .collect();

    let folds = lines[blank + 1..]
        .iter()
        .map(|&l| (&l[11..=11] == "y", l[13..].parse().unwrap()))
        .collect();

    (coords, folds)
}

fn bend(
    coords: impl Iterator<Item = (isize, isize)>,
    &(y_axis, line): &(bool, isize),
) -> HashSet<(isize, isize)> {
    coords
        .map(|c| {
            if !y_axis {
                if c.0 < line {
                    (c.0, c.1)
                } else {
                    (2 * line - c.0, c.1)
                }
            } else {
                if c.1 < line {
                    (c.0, c.1)
                } else {
                    (c.0, 2 * line - c.1)
                }
            }
        })
        .collect()
}

fn print(coords: impl Iterator<Item = (isize, isize)>) {
    let coords = coords.collect::<Vec<_>>();
    let min = coords.iter().fold((isize::max_value(), isize::max_value()), |m, c| {
        (m.0.min(c.0), m.1.min(c.1))
    });
    let max = coords.iter().fold((isize::min_value(), isize::min_value()), |m, c| {
        (m.0.max(c.0), m.1.max(c.1))
    });
    let size = (max.0 - min.0 + 1, max.1 - min.1 + 1);
    let mut bmp = Bitmap::<bool>::new(size);
    for c in coords {
        bmp.set(&(c.0 - min.0, c.1 - min.1), true);
    }
    let board = bmp.to_string(|&b| if b { "#" } else { "." });
    println!("{:?}\n{}\n", board.size(), board);
}