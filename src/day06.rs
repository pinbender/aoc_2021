pub fn solve_1(lines: &[&str]) -> usize {
    let mut histogram = parse(lines);
    for _ in 0..80 {
        sim_day(&mut histogram);
    }
    histogram.into_iter().sum()
}

pub fn solve_2(lines: &[&str]) -> usize {
    let mut histogram = parse(lines);
    for _ in 0..256 {
        sim_day(&mut histogram);
    }
    histogram.into_iter().sum()
}

fn parse(lines: &[&str]) -> [usize; 9] {
    let mut histogram = [0; 9];
    let input = lines[0].split(',').map(|n| n.parse::<usize>().unwrap());
    input.map(|n| histogram[n] += 1).for_each(drop);
    histogram
}

fn sim_day(histogram: &mut [usize; 9]) {
    let zeroes = histogram[0];
    for i in 0..8 {
        histogram[i] = histogram[i + 1];
    }
    histogram[8] = zeroes;
    histogram[6] += zeroes;
}
