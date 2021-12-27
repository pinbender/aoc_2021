#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate itertools;

#[allow(dead_code)]
mod bitmap;
#[allow(dead_code)]
mod twod;

mod day19;

fn main() {
    let lines: Vec<_> = include_str!("../data/day19.txt").lines().collect();
    let solution = day19::solve(&lines);
    println!("Day 19: {:?}", solution);
}
