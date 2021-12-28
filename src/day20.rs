use crate::bitmap::Bitmap;
use std::collections::HashSet;

pub fn solve(lines: &[&str]) -> (usize, usize) {
    let (key, board) = Board::parse(lines);

    let pt_1 = (0..2)
        .fold(board.clone(), |board, _| {
            let enhanced = board.enhance(&key);
            //enhanced.print();
            enhanced
        })
        .pixels
        .len();

    let pt_2 = (0..50)
        .fold(board, |board, _| {
            let enhanced = board.enhance(&key);
            //enhanced.print();
            enhanced
        })
        .pixels
        .len();

    (pt_1, pt_2)
}

#[derive(Clone)]
struct Board {
    pixels: HashSet<(i32, i32)>,
    bounds: ((i32, i32), (i32, i32)),
    background: bool,
}

impl Board {
    fn parse(lines: &[&str]) -> (Vec<bool>, Board) {
        let key = lines[0].chars().map(|c| c == '#').collect();

        let size = (lines[2].len() as i32, (lines.len() - 2) as i32);
        let bounds = ((0, 0), size);
        let pixels: HashSet<_> = lines[2..]
            .iter()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .filter_map(move |(x, c)| (c == '#').then(|| (x as i32, y as i32)))
            })
            .collect();
        (
            key,
            Board {
                pixels,
                bounds,
                background: false,
            },
        )
    }

    fn enhance(&self, key: &Vec<bool>) -> Self {
        let new_bounds = (
            (self.bounds.0 .0 - 2, self.bounds.0 .1 - 2),
            (self.bounds.1 .0 + 2, self.bounds.1 .1 + 2),
        );

        let new_pixels: HashSet<_> = (new_bounds.0 .1..new_bounds.1 .1)
            .flat_map(|y| {
                (new_bounds.0 .0..new_bounds.1 .0).filter_map(move |x| self.enhance_at(key, (x, y)))
            })
            .collect();

        let background = if key[0] {
            !self.background
        } else {
            self.background
        };

        Self {
            pixels: new_pixels,
            bounds: new_bounds,
            background,
        }
    }

    fn enhance_at(&self, key: &Vec<bool>, coord: (i32, i32)) -> Option<(i32, i32)> {
        let index = [
            (coord.0 + 1, coord.1 + 1),
            (coord.0, coord.1 + 1),
            (coord.0 - 1, coord.1 + 1),
            (coord.0 + 1, coord.1),
            (coord.0, coord.1),
            (coord.0 - 1, coord.1),
            (coord.0 + 1, coord.1 - 1),
            (coord.0, coord.1 - 1),
            (coord.0 - 1, coord.1 - 1),
        ]
        .into_iter()
        .enumerate()
        .fold(0, |index, (i, c)| {
            index | ((self.get_pixel(&c) as usize) << i)
        });
        key[index].then(|| coord)
    }

    fn get_pixel(&self, c: &(i32, i32)) -> bool {
        if c.0 < self.bounds.0 .0
            || c.1 < self.bounds.0 .1
            || c.0 >= self.bounds.1 .0
            || c.1 >= self.bounds.1 .1
        {
            self.background
        } else {
            self.pixels.contains(c)
        }
    }

    fn print(&self) {
        let size = (
            (self.bounds.1 .0 - self.bounds.0 .0) as isize,
            (self.bounds.1 .1 - self.bounds.0 .1) as isize,
        );
        let mut debug = Bitmap::<bool>::new(size);
        for p in self.pixels.iter() {
            let bitmap_coord = (
                (p.0 - self.bounds.0 .0) as isize,
                (p.1 - self.bounds.0 .1) as isize,
            );
            debug.set(&bitmap_coord, true);
        }
        println!("{}", debug.to_string(|p| if *p { '#' } else { '.' }));
        println!("Background: {}\n", self.background);
    }
}
