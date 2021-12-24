use std::mem;
use itertools::Itertools;

pub struct Bitmap<T> {
    data: Vec<T>,
    _size: (isize, isize),
}

impl<T> Bitmap<T> {
    pub fn new(size: (isize, isize)) -> Self
    where
        T: Default + Copy,
    {
        Bitmap {
            data: vec![Default::default(); (size.0 * size.1) as usize],
            _size: size,
        }
    }
    pub fn from_lines(lines: &[&str], translator: impl Fn(char) -> T) -> Option<Self>
    where
        T: Default,
    {
        let width = lines.get(0)?.len();
        let height = lines.len();
        let mut data = Vec::new();
        data.reserve_exact(width * height);
        lines
            .iter()
            .flat_map(|l| l.chars().map(&translator))
            .for_each(|n| data.push(n));
        Some(Bitmap {
            data,
            _size: (width as isize, height as isize),
        })
    }
    pub fn size(&self) -> (isize, isize) {
        self._size
    }
    pub fn fill(&mut self, mut data: impl Iterator<Item = T>)
    where
        T: Default,
    {
        self.data.fill_with(|| data.next().unwrap_or_default());
    }
    pub fn get(&self, coord: &(isize, isize)) -> Option<&T> {
        self.in_bounds(&coord)
            .then(|| &(*self.data)[self.index(coord)])
    }
    pub fn get_mut(&mut self, coord: &(isize, isize)) -> Option<&mut T> {
        self.in_bounds(&coord)
            .then(|| self.index(coord))
            .map(|i| &mut (*self.data)[i])
    }
    pub fn set(&mut self, coord: &(isize, isize), value: T) -> Option<T> {
        if self.in_bounds(coord) {
            let index = self.index(coord);
            Some(mem::replace(&mut self.data[index], value))
        } else {
            None
        }
    }
    pub fn visit(&self) -> impl Iterator<Item = ((isize, isize), &T)> {
        (0..self.data.len()).map(|i| (self.coord(i), &self.data[i]))
    }
    pub fn mutate(&mut self, mut f: impl FnMut((isize, isize), &T) -> T) {
        for i in 0..self.data.len() {
            let new_value = f(self.coord(i), &self.data[i]);
            self.data[i] = new_value;
        }
    }
    pub fn in_bounds(&self, coord: &(isize, isize)) -> bool {
        coord.0 >= 0 && coord.1 >= 0 && coord.0 < self._size.0 && coord.1 < self._size.1
    }

    pub fn index(&self, coord: &(isize, isize)) -> usize {
        coord.1 as usize * self._size.0 as usize + coord.0 as usize
    }
    pub fn coord(&self, index: usize) -> (isize, isize) {
        let width = self._size.0 as usize;
        ((index % width) as isize, (index / width) as isize)
    }
    pub fn to_string(&self, to_char: impl Fn(&T) -> char) -> String {
        let size = self.size();
        self.visit()
            .map(|(_, n)| to_char(n))
            .chunks(size.0 as usize).into_iter()
            .map(|line| line.collect::<String>())
            .join("\n")
    }
}
