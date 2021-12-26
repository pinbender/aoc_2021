#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;

#[allow(dead_code)]
mod bitmap;
#[allow(dead_code)]
mod twod;

mod day16;

fn main() {
    {
        let lines: Vec<_> = include_str!("../data/day16.txt").lines().collect();
        let solution = day16::solve(&lines);
        println!("Day 16: {:?}", solution);
    }
}
