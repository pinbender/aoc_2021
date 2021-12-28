#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;

#[allow(unused_imports)]
#[macro_use]
extern crate itertools;

#[allow(dead_code)]
mod bitmap;
#[allow(dead_code)]
mod twod;

mod day21;

fn main() {
    let lines: Vec<_> = include_str!("../data/day21.txt").lines().collect();
    let solution = day21::solve(&lines);
    println!("Day 21: {:?}", solution);
}
