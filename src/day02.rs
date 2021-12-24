pub fn solve_1(lines: &[&str]) -> usize {
    let position = parse(lines).fold((0,0),
        |(advance, depth), (direction, distance)| {
            match direction {
                Dir::Forward => (advance + distance, depth),
                Dir::Up => (advance, depth - distance),
                Dir::Down => (advance, depth + distance)
            }
        });
    finish(position)
}

pub fn solve_2(lines: &[&str]) -> usize {
    let position = parse(lines).fold((0,0,0),
        |(advance, depth, aim), (direction, distance)| {
            match direction {
                Dir::Forward => (advance + distance, depth + aim * distance, aim),
                Dir::Up => (advance, depth, aim - distance),
                Dir::Down => (advance, depth, aim + distance)
            }
        });
    finish((position.0, position.1))
}

enum Dir {
    Forward,
    Up,
    Down
}

fn parse_dir(direction: &str) -> Dir {
    match direction {
        "forward" => Dir::Forward,
        "up" => Dir::Up,
        "down" => Dir::Down,
        _ => unreachable!("invalid direction")
    }
}

fn parse<'a>(lines: &'a[&str]) -> impl Iterator<Item=(Dir, i32)> + 'a {
    lines.iter().map(|line| {
        let mut split = line.split(" ");
        let direction = parse_dir(split.next().unwrap());
        let distance = split.next().unwrap().parse::<i32>().unwrap();
        (direction, distance)
    })
}

fn finish(position: (i32, i32)) -> usize {
    (position.0 * position.1) as usize
}
