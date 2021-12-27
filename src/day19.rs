use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{HashSet, VecDeque};
use tuple::Map;

pub fn solve(lines: &[&str]) -> (usize, usize) {
    let mut scanners = parse(lines);

    // 12 correlated beacons = 11^2 distances, bi-directional = half
    let overlap_threshold = (11 * 11) / 2;

    let distances: Vec<_> = scanners.iter().map(Scanner::compute_distances).collect();

    let mut work: VecDeque<_> = (0..scanners.len())
        .tuple_combinations()
        .map(|(s0, s1)| {
            let overlaps = distances[s0].intersection(&distances[s1]).count();
            ((s0, s1), overlaps)
        })
        .sorted_by_key(|(_pair, overlaps)| Reverse(*overlaps))
        .filter_map(|(pair, overlaps)| (overlaps >= overlap_threshold).then(|| pair))
        .collect();

    let mut offsets = vec![Point3::default(); scanners.len()];
    let mut done: HashSet<usize> = HashSet::new();
    done.insert(0);
    while let Some((mut o, mut s)) = work.pop_front() {
        if !done.contains(&o) {
            if !done.contains(&s) {
                // Do it later
                work.push_back((o,s));
                continue;
            }
            else {
                std::mem::swap(&mut o, &mut s);
            }
        }
        if !done.contains(&s) {
            println!("{} -> {}", o, s);
            let (orientation, offset) = scanners[o].orient(&scanners[s]);
            offsets[s] = offset;
            scanners[s].adjust(&orientation, &offset);
            done.insert(s);
        }
    }

    for (s, scanner) in scanners.iter().enumerate() {
        println!("Scanner {} @ {:?}:", s, offsets[s]);
        for beacon in scanner.beacons.iter().sorted() {
            println!("{:?}", beacon);
        }
        println!("");
    }

    let pt_1 = scanners
        .iter()
        .flat_map(|scanner| scanner.beacons.iter().map(|beacon| *beacon))
        .unique()
        .count();

    let pt_2 = offsets.into_iter().tuple_combinations()
        .map(|(a, b)| distance(&a, &b))
        .max()
        .unwrap() as usize;

    (pt_1, pt_2)
}

fn parse(lines: &[&str]) -> Vec<Scanner> {
    let mut result: Vec<Scanner> = Vec::new();
    for line in lines {
        if line.starts_with("--") {
            result.push(Default::default());
        } else if !line.is_empty() {
            let top = result.last_mut().unwrap();
            let pos = line
                .split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap();
            top.beacons.push(pos);
        }
    }
    for scanner in result.iter_mut() {
        scanner.compute_distances();
    }
    result
}

type Point3 = (i32, i32, i32);

fn vec_sub(p0: &Point3, p1: &Point3) -> Point3 {
    (p0.0 - p1.0, p0.1 - p1.1, p0.2 - p1.2)
}

fn vec_add(p0: &Point3, p1: &Point3) -> Point3 {
    (p0.0 + p1.0, p0.1 + p1.1, p0.2 + p1.2)
}

fn vec_abs(p: &Point3) -> Point3 {
    (p.0.abs(), p.1.abs(), p.2.abs())
}

fn vec_sum(p: &Point3) -> i32 {
    p.0 + p.1 + p.2
}

fn vec_has_zero(p: &Point3) -> bool {
    p.0 == 0 || p.1 == 0 || p.2 == 0
}

fn distance(p0: &Point3, p1: &Point3) -> i32 {
    // Manhattan should be fine, right?
    vec_sum(&vec_abs(&vec_sub(p0, p1)))
}

fn beacon_pair_fingerprint(b0: &Point3, b1: &Point3) -> Point3 {
    let p = vec_abs(&vec_sub(b0, b1));
    let p0 = p.0.max(p.1).max(p.2);
    let p2 = p.0.min(p.1).min(p.2);
    let p1 = (p.0 + p.1 + p.2) - (p0 + p2);
    (p0, p1, p2)
}

fn get_shuffle(origin: &Point3, other: &Point3) -> Option<Point3> {
    let options = [1,-1, 2,-2, 3,-3];
    let mut candidates = 
        iproduct!(options.iter().cloned(), options.iter().cloned(), options.iter().cloned())
        .filter(|candidate| apply_shuffle(candidate, other) == *origin);

    let first = candidates.next();
    if first.is_some() && !candidates.next().is_some() {
        first
    }
    else {
        None
    }
}

fn apply_shuffle_element(s: i32, p: &Point3) -> i32 {
    match s {
        1 => p.0,
        -1 => -p.0,
        2 => p.1,
        -2 => -p.1,
        3 => p.2,
        -3 => -p.2,
        _ => unreachable!("Invalid shuffle element")
    }
}

fn apply_shuffle(shuffle: &Point3, p: &Point3) -> Point3 {
    shuffle.map(|n| apply_shuffle_element(n, p))
}

fn orient_points(
    self_p0: &Point3, self_p1: &Point3,
    other_p0: &Point3, other_p1: &Point3) -> Option<(Point3, Point3)> 
    {
    let sample_offset = vec_sub(&self_p1, &self_p0);
    // A zero offset in an axis can't differentiate positive from negative
    if vec_has_zero(&sample_offset) {
        return None;
    }
    let other_offset = vec_sub(&other_p1, &other_p0);

    let shuffle = get_shuffle(&sample_offset, &other_offset);
    shuffle.map(|shuffle| {
        let adjusted = apply_shuffle(&shuffle, &other_p0);
        let offset = vec_sub(&self_p0, &adjusted);
        (shuffle, offset)
    })
}

#[derive(Default)]
struct Scanner {
    beacons: Vec<Point3>,
}

impl Scanner {
    fn compute_distances(&self) -> HashSet<i32> {
        let num_beacons = self.beacons.len();
        (0..num_beacons)
            .tuple_combinations()
            .map(|(p0, p1)| {
                let pos0 = self.beacons[p0];
                let pos1 = self.beacons[p1];
                distance(&pos0, &pos1)
            })
            .collect()
    }
    fn compute_fingerprints(&self) -> Vec<HashSet<Point3>> {
        let num_beacons = self.beacons.len();
        (0..num_beacons)
            .map(|a| {
                (0..num_beacons)
                    .map(|b| beacon_pair_fingerprint(&self.beacons[a], &self.beacons[b]))
                    .collect()
            })
            .collect()
    }
    fn orient(&self, other: &Self) -> (Point3, Point3) {
        let self_fp = self.compute_fingerprints();
        let other_fp = other.compute_fingerprints();

        // Includes "self" in the fingerprint sets
        let min_neighbors = 12;

        self_fp.iter().enumerate()
            // Find beacon indices with enough matching neighbor fingerprints
            .filter_map(|(i, self_fp_set)| {
                let best = other_fp.iter()
                    .map(|other_fp_set| self_fp_set.intersection(other_fp_set).count())
                    .enumerate()
                    .max_by_key(|(_j, score)| *score)
                    .unwrap();
                (best.1 >= min_neighbors).then(|| (i,best.0))
            })
            // Find the first available unambiguous shuffle & offset
            .tuple_windows()
            .filter_map(|(pair0, pair1)| {
                let self_p0 = &self.beacons[pair0.0];
                let self_p1 = &self.beacons[pair1.0];
                let other_p0 = &other.beacons[pair0.1];
                let other_p1 = &other.beacons[pair1.1];
                orient_points(self_p0, self_p1, other_p0, other_p1)
            })
            .next()
            .unwrap()
    }
    fn adjust(&mut self, shuffle: &Point3, offset: &Point3) {
        for beacon in self.beacons.iter_mut() {
            *beacon = vec_add(&apply_shuffle(shuffle, beacon), offset);
        }
    }
}
