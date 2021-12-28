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

mod day22;

fn main() {
    let lines: Vec<_> = include_str!("../data/day22.txt").lines().collect();
    let solution = day22::solve(&lines);
    println!("Day 22: {:?}", solution);
}
