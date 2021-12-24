pub fn solve_1(lines: &[&str]) -> usize {
    let (size, parsed) = parse(lines);
    let (gamma, epsilon) = compute_gamma_epsilon(size, &parsed);
    gamma * epsilon
}

pub fn solve_2(lines: &[&str]) -> usize {
    let (size, parsed) = parse(lines);
    let oxygen = compute_rating(true, size.0, &parsed);
    let scrubber = compute_rating(false, size.0, &parsed);
    oxygen * scrubber
}

fn compute_gamma_epsilon(size: (usize, usize), data: &Vec<usize>) -> (usize, usize) {
    let (width, height) = size;
    let mut tally = vec![0; width];
    for p in data {
        for (i, t) in tally.iter_mut().enumerate() {
            if (p & (1 << (width - i - 1))) != 0 {
                *t += 1
            };
        }
    }
    let mut gamma = 0;
    for bit in tally {
        gamma = gamma << 1;
        if bit > height / 2 {
            gamma |= 1;
        }
    }

    let mask = (1 << width) - 1;
    (gamma, !gamma & mask)
}

fn get_column_gamma(mask: usize, data: &Vec<usize>) -> usize {
    let population = data.iter().filter(|d| (*d & mask) != 0).count();
    if population >= ((data.len() + 1)/ 2) {
        mask
    } else {
        0
    }
}

fn compute_rating(oxy: bool, width: usize, data: &Vec<usize>) -> usize {
    let mut working_set = data.clone();
    for i in 0..width {
        let col = width - i - 1;
        let mask = 1 << col;
        let mut flag = get_column_gamma(mask, &working_set);
        if !oxy {
            flag = !flag & mask;
        }
        working_set.retain(|n| n & mask == flag);
        if working_set.len() == 1 {
            return working_set[0];
        }
    }
    unreachable!("Rating computation failed!");
}

fn parse(lines: &[&str]) -> ((usize, usize), Vec<usize>) {
    let size = (lines[0].len(), lines.len());
    let data = lines
        .iter()
        .map(|l| usize::from_str_radix(l, 2).unwrap())
        .collect();
    (size, data)
}
