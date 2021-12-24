use crate::bitmap::Bitmap;

pub fn solve_1(lines: &[&str]) -> usize {
    let cave = parse(lines);
    low_points(&cave)
        .map(|(_, depth)| depth as usize + 1).sum()
}

pub fn solve_2(lines: &[&str]) -> usize {
    let cave = parse(lines);
    let lows = low_points(&cave).map(|(c, _)| c).collect::<Vec<_>>();
    let mut tally = vec![1; lows.len()];
    let mut basins = Bitmap::<Option<u8>>::new(cave.size());
    for (i, c) in lows.into_iter().enumerate() {
        basins.set(&c, Some(i as u8));
    }

    for level in 1..=8 {
        for (coord, _) in cave.visit().filter(|&(_, &depth)| depth == level) {
            if !basins.get(&coord).unwrap().is_some() {
                let basin = 
                    *neighbors(&coord).filter_map(|neighbor| {
                        basins.get(&neighbor)
                    }).find(|b| b.is_some()).unwrap();
                basins.set(&coord, basin);
                tally[basin.unwrap() as usize] += 1;
            }
        }
    }

    tally.sort();
    tally.iter().skip(tally.len() - 3).fold(1, |result, basin_size| result * basin_size)
}

fn parse(lines: &[&str]) -> Bitmap<u8> {
    let size = (lines[0].len() as isize, lines.len() as isize);
    let mut result = Bitmap::new(size);
    result.fill(lines.iter().flat_map(
        |l| l.chars().map(|c| c.to_digit(10).unwrap() as u8)
    ));
    result
}

fn is_lower(d: u8, other: Option<&u8>) -> bool {
    other.map(|o| *o > d).unwrap_or(true) 
}

fn low_points(cave: &Bitmap<u8>) -> impl Iterator<Item=((isize, isize), u8)> + '_ {
    cave.visit().filter(|&(c, &depth)| {
        neighbors(&c).all(|other| is_lower(depth, cave.get(&other)))
    }).map(|(c, &depth)| (c, depth))
}

fn neighbors(c: &(isize, isize)) -> impl Iterator<Item=(isize, isize)> {
    [
        (c.0 - 1, c.1),
        (c.0 + 1, c.1),
        (c.0, c.1 - 1),
        (c.0, c.1 + 1)
    ].into_iter()
}