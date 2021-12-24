use std::collections::HashMap;
use itertools::Itertools;

pub fn solve(lines: &[&str]) -> (usize, usize) {
    let (puzzle, key) = parse(lines);
    let mut buffer = puzzle.into_iter()
        .chain([0].into_iter())
        .tuple_windows()
        .map(|(a, b)| [a,b])
        .fold(HashMap::new(), |mut h, symbol| {
            *h.entry(symbol).or_insert(0) += 1;
            h
        });
    for _ in 0..10 {
        buffer = step(buffer, &key);
    }
    let counter = count_buffer(&buffer);
    let (min1, max1) = counter.values().cloned().minmax().into_option().unwrap();

    for _ in 10..40 {
        buffer = step(buffer, &key);
    }
    let counter = count_buffer(&buffer);
    let (min2, max2) = counter.values().cloned().minmax().into_option().unwrap();

    (max1 - min1, max2 - min2)
}

fn parse(lines: &[&str]) -> (Vec<u8>, HashMap<[u8; 2], u8>) {
    let puzzle = lines[0].as_bytes();
    let key = lines[2..]
        .iter()
        .map(|&l| {
            (
                l[0..2].bytes().collect::<Vec<_>>().try_into().unwrap(),
                l.bytes().nth(6).unwrap(),
            )
        })
        .collect();
    (puzzle.into(), key)
}

fn step(buffer: HashMap<[u8; 2], usize>, key: &HashMap<[u8; 2], u8>) -> HashMap<[u8; 2], usize> {
    buffer.iter()
        .fold(HashMap::new(), |mut h, (symbol, count)| {
            if let Some(&lookup) = key.get(symbol) {
                *h.entry([symbol[0], lookup]).or_insert(0) += count;
                *h.entry([lookup, symbol[1]]).or_insert(0) += count;
            }
            else {
                *h.entry(symbol.clone()).or_insert(0) += count;
            }
            h
        })
}

fn count_buffer(buffer: &HashMap<[u8; 2], usize>) -> HashMap<u8, usize> {
    buffer.iter()
        .fold(HashMap::new(), |mut h, (symbol, count)| {
            *h.entry(symbol[0]).or_insert(0) += count;
            h
        })
}