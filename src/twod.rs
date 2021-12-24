use std::ops::Add;

pub trait TwoD {
    fn neighbors4(&self) -> [Self; 4]
    where
        Self: std::marker::Sized;
}

pub fn add_vec<T: Add + Copy>(
    a: &(T, T),
    b: &(T, T),
) -> (<T as std::ops::Add>::Output, <T as std::ops::Add>::Output) {
    (a.0 + b.0, a.1 + b.1)
}

impl TwoD for (isize, isize) {
    fn neighbors4(&self) -> [Self; 4] {
        [
            add_vec(&self, &(-1, 0)),
            add_vec(&self, &(1, 0)),
            add_vec(&self, &(0, -1)),
            add_vec(&self, &(0, 1)),
        ]
    }
}
