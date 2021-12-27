use lazy_static;
use regex::Regex;
use itertools::Itertools;

pub fn solve(lines: &[&str]) -> (usize, usize) {
    let result_1 = lines[1..].iter().copied()
        .map(str::to_owned)
        .fold(lines[0].to_owned(), add);

    let answer_1 = magnitude(&result_1);

    let answer_2 = lines.iter().copied()
        .map(str::to_owned)
        .permutations(2)
        .map(|p| magnitude(&add(p[0].clone(),p[1].clone())))
        .max().unwrap();
        
    (answer_1, answer_2)
}

fn add(left: String, right: String) -> String {
    let result = reduce(format!("[{},{}]", left, right));
    println!("  {}\n+ {}\n= {}\n", left, right, result);
    result
}

fn reduce(num: String) -> String {
    let mut work = num;
    loop {
        let to_explode = find_explode(&work);
        let to_split = find_split(&work);
        if to_explode.0 != to_explode.1 {
            work = explode_at(work, to_explode);
        }
        else if to_split.0 != to_split.1 {
            work = split_at(work, to_split);
        }
        else {
            break;
        }
    }
    work
}

fn find_explode(num: &String) -> (usize, usize) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\[\d+,\d+\]").unwrap();
    }
    
    let mut depth = 0;
    for (i,c) in num.chars().enumerate() {
        match c {
            '[' => { 
                depth += 1; 
                if depth > 4 {
                    if let Some(m) = RE.find(&num[i..]) {
                        return (i + m.start(), i + m.end());
                    }
                }
            },
            ']' => { depth -= 1; },
            _ => ()
        }
    }
    (num.len(), num.len())
}

fn find_split(num: &String) -> (usize, usize) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d\d+").unwrap();
    }
    RE.find(num)
        .map(|m| (m.start(), m.end()))
        .unwrap_or((num.len(), num.len()))
}

fn explode_at(mut num: String, index: (usize, usize)) -> String {
    let mut split = num[index.0+1..index.1-1].split(',').map(|n| n.parse::<i32>().unwrap());
    let expl_a = split.next().unwrap();
    let expl_b = split.next().unwrap();
    let mut left_piece = num[0..index.0].to_owned();
    let mut right_piece = num[index.1..].to_owned();

    //println!("explode: {}/[{},{}]/{}", left_piece, expl_a, expl_b, right_piece);

    lazy_static! {
        static ref FIND_NUMBER: Regex = Regex::new(r"\d+").unwrap();
    }

    let left_number = FIND_NUMBER.find_iter(&left_piece).last();
    if let Some(left_match) = left_number {
        let left_num = left_match.as_str().parse::<i32>().unwrap();
        let new_left = left_num + expl_a;
        left_piece = format!("{}{}{}",
            &left_piece[0..left_match.start()],
            new_left,
            &left_piece[left_match.end()..]
        );
    }

    let right_number = FIND_NUMBER.find_iter(&right_piece).next();
    if let Some(right_match) = right_number {
        let right_num = right_match.as_str().parse::<i32>().unwrap();
        let new_right = right_num + expl_b;
        right_piece = format!("{}{}{}",
            &right_piece[0..right_match.start()],
            new_right,
            &right_piece[right_match.end()..]
        );
    }

    num = format!("{}0{}", left_piece, right_piece);
    //println!("explode_result: {}", num);
    num
}

fn split_at(num: String, index: (usize, usize)) -> String {
    let left_side = &num[0..index.0];
    let split_num = num[index.0..index.1].parse::<i32>().unwrap();
    let right_side = &num[index.1..];
    //println!("split: {}/{}/{}", left_side, split_num, right_side);
    let left = split_num / 2;
    let right = (split_num + 1) / 2;
    let result = format!("{}[{},{}]{}", left_side, left, right, right_side);
    //println!("split_result: {}", result);
    result
}

fn magnitude(num: &str) -> usize {
    let mut stack = Vec::new();
    for c in num.chars() {
        match c {
            ',' => {
                let left = stack.pop().unwrap();
                stack.push(left * 3);
            },
            ']' => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left + right * 2);
            },
            '[' => {},
            _ => {
                stack.push(c.to_digit(10).unwrap() as usize);
            }
        }
    }
    stack.pop().unwrap()
}