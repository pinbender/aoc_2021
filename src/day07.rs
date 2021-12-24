pub fn solve_1(lines: &[&str]) -> usize {
    let input = parse(lines);
    solve_general(&input, |n| n)
}

pub fn solve_2(lines: &[&str]) -> usize {
    let input = parse(lines);
    solve_general(&input, |n| (n*n + n) / 2)
}

pub fn solve_general(input: &[isize], cost: impl Fn(isize) -> isize) -> usize {
    let max = input.iter().max().unwrap();
    (0..=*max).map(|test| 
        input.iter().map(|&n| cost((test - n).abs()))
            .sum::<isize>() as usize
    ).min().unwrap()
}

fn parse(lines: &[&str]) -> Vec<isize> {
    lines[0].split(',').map(|n| n.parse::<isize>().unwrap()).collect()
}