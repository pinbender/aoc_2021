use std::collections::{HashMap,HashSet};
use itertools::Itertools;

pub fn solve_1(lines: &[&str]) -> usize {
    let graph = parse(lines);
    let mut visited = HashMap::new();
    let mut path = Vec::new();
    let mut results = HashSet::new();
    path.push("start");
    //println!("part 1:");
    recursive_solve(&graph, &mut visited, &mut path, 1, &mut results);
    
    results.len()
}

pub fn solve_2(lines: &[&str]) -> usize {
    let graph = parse(lines);
    let mut visited = HashMap::new();
    let mut path = Vec::new();
    path.push("start");
    let mut results = HashSet::new();
    //println!("part 2:");
    recursive_solve(&graph, &mut visited, &mut path, 2, &mut results);
    results.len()
}

fn parse<'a>(lines: &'a [&'a str]) -> HashMap<&str, Vec<&str>> {
    let mut graph = HashMap::new();
    for (a,b) in lines.iter().flat_map(|l| l.split("-")).tuples() {
        let list: &mut Vec<&str> = graph.entry(a).or_default();
        if b != "start" {
            list.push(b);
        }
        let list = graph.entry(b).or_default();
        if a != "start" {
            list.push(a);
        }
    }
    graph
}

fn recursive_solve<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>, 
    visited: &mut HashMap<&'a str, usize>, 
    path: &mut Vec<&'a str>,
    limit: usize,
    results: &mut HashSet<String>) -> usize {
    let tip = path.last().unwrap();
    if tip == &"end" {
        let result = path.join(",");
        let inserted = results.insert(result/*.clone()*/);
        if inserted { 
            //println!("{}", result);
            1 
        } else { 0 }
    }
    else {
        let neighbors = graph.get(tip).unwrap();
        let mut result = 0;
        for node in neighbors {
            let lower = node.chars().next().unwrap().is_lowercase();
            let mut local_limit = limit;
            if lower {
                let entry = visited.entry(node.clone()).or_insert(0);
                if *entry >= limit {
                    continue;
                }
                else {
                    *entry += 1;
                    if *entry >= limit {
                        local_limit = 1;
                    }
                }
            }

            path.push(node.clone());
            result += recursive_solve(graph, visited, path, local_limit, results);
            path.pop();
            
            if lower {
                let entry = visited.get_mut(node).expect("Unable to undo visit");
                *entry -= 1;
            }
        }
        result
    }
}