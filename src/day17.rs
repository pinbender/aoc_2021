pub fn solve(lines: &[&str]) -> (usize, usize) {
    let range = parse_range(lines[0]);

    let pt1 = sum_of_n(range.1.0);

    let search_y = range.1.0.abs();
    let pt2 = (1..=range.0.1)
        .flat_map(|x| (-search_y..=search_y).map(move |y| (x,y)))
        .filter(|&c| is_in_bounds(c, range))
        .count() as isize;
    (pt1 as usize, pt2 as usize)
}

type Range1d = (isize, isize);
type Range2d = ((isize, isize), (isize, isize));

fn sum_of_n(n: isize) -> isize {
    (n * (n + 1)) / 2
}

fn test_bounds1(c: isize, range: Range1d) -> bool {
    (range.0 <= c) && (c <= range.1)
}

fn test_bounds2(c: (isize, isize), range: Range2d) -> bool {
    test_bounds1(c.0, range.0)
        && test_bounds1(c.1, range.1)
}

fn is_in_bounds(mut vel: (isize, isize), bounds: Range2d) -> bool {
    let _vel_start = vel;
    let mut c = (0,0);
    while (c.0 <= bounds.0.1) && (c.1 >= bounds.1.0) {
        if vel.0 > 0 {
            c.0 += vel.0;
            vel.0 -= 1;
        }
        c.1 += vel.1;
        vel.1 -= 1;
        if test_bounds2(c, bounds) {
            //println!("Yes: {:?}", _vel_start);
            return true;
        }
    }
    false
}

fn parse_range(line: &str) -> Range2d {
    let parts: Vec<_> = line.split(",").collect();
    let x_part = parts[0].split("=").last().unwrap();
    let y_part = parts[1].split("=").last().unwrap();
    let x_range: Vec<_> = x_part.split("..").map(|n| n.parse().unwrap()).collect();
    let y_range: Vec<_> = y_part.split("..").map(|n| n.parse().unwrap()).collect();
    ((x_range[0], x_range[1]), (y_range[0], y_range[1]))
} 