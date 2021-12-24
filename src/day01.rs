pub fn solve_1(lines: &[&str]) -> usize {
    solve(1, &parse(lines))
}

pub fn solve_2(lines: &[&str]) -> usize {
    solve(3, &parse(lines))
}

fn parse(lines: &[&str]) -> Vec<i32> {
    lines.iter().map(|l| l.parse().unwrap()).collect()
}

fn solve(width: usize, entries: &[i32]) -> usize {
    entries.windows(width).zip(entries[1..].windows(width))
        .filter(|(a,b)| a.iter().sum::<i32>() < b.iter().sum::<i32>()).count()
}
