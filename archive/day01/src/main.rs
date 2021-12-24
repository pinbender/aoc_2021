
fn main() {
    let input = include_str!("../day01.txt");
    let entries = input.lines().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let result = solve(3, &entries);
    println!("Result: {}", result);
}

fn solve(width: usize, entries: &[i32]) -> usize {
    entries.windows(width).zip(entries[1..].windows(width))
        .filter(|(a,b)| a.iter().sum::<i32>() < b.iter().sum::<i32>()).count()
}
