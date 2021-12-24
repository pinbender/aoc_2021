use crate::bitmap::Bitmap;
use crate::twod::*;
use std::cmp::Reverse;
use std::collections::{hash_map::Entry, BinaryHeap, HashMap};

pub fn solve(lines: &[&str]) -> (usize, usize) {
    let maze = Bitmap::<u8>::from_lines(&lines, |c| c.to_digit(10).unwrap() as u8).unwrap();

    let result_1 = find_best_path_cost(&maze);

    let maze_size = maze.size();
    let mut big_maze = Bitmap::<u8>::new((maze_size.0 * 5, maze_size.1 * 5));
    for y_tile in 0..5 {
        for x_tile in 0..5 {
            let bias = (x_tile + y_tile) as u8;
            let origin = (maze_size.0 * x_tile, maze_size.1 * y_tile);
            for (c, &from) in maze.visit() {
                big_maze.set(&add_vec(&origin, &c), (from + bias - 1) % 9 + 1);
            }
        }
    }

    //print_maze(&big_maze);

    let result_2 = find_best_path_cost(&big_maze);
    (result_1, result_2)
}

fn find_best_path_cost(maze: &Bitmap<u8>) -> usize {
    let start = (0 as isize, 0 as isize);
    let end = add_vec(&maze.size(), &(-1, -1));

    let mut from = HashMap::new();
    from.insert(start, start);

    let mut cost = HashMap::new();
    cost.insert(start, 0);

    let mut todo = BinaryHeap::new();
    todo.push((Reverse(0), start));

    let result = loop {
        let (_priority, tip) = todo.pop().unwrap();
        //print!("{:?}={}/", tip, _priority);
        if tip == end {
            break _priority.0;
        }

        let tip_cost = *cost.get(&tip).unwrap();

        for neighbor in tip.neighbors4().into_iter() {
            let add_cost = maze.get(&neighbor);
            if add_cost.is_none() {
                continue;
            }
            let neighbor_cost = *add_cost.unwrap() as usize;

            let new_cost = tip_cost + neighbor_cost;
            let neighbor_cost_entry = cost.entry(neighbor);
            if match &neighbor_cost_entry {
                Entry::Vacant(_) => true,
                Entry::Occupied(existing) => new_cost < *existing.get(),
            } {
                *neighbor_cost_entry.or_default() = new_cost;
                *from.entry(neighbor).or_default() = tip;
                let heuristic = manhattan(&neighbor, &end);
                todo.push((Reverse(new_cost + heuristic), neighbor));
            }
        }
    };

    /*
    let mut result = 0;
    let mut tip = end;
    while tip != start {
        println!("{:?} = {}", tip, maze.get(&tip).unwrap());
        result += *maze.get(&tip).unwrap() as usize;
        tip = *from.get(&tip).expect("Broken path");
    }
    */

    result
}

fn manhattan(coord: &(isize, isize), target: &(isize, isize)) -> usize {
    (coord.0 - target.0).abs() as usize + (coord.1 - target.1).abs() as usize
}

#[allow(unused)]
fn print_maze(maze: &Bitmap<u8>) {
    println!(
        "{:?}\n{}\n",
        maze.size(),
        maze.to_string(|&n| char::from_digit(n as u32, 10).unwrap())
    );
}
