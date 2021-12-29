use itertools::Itertools;
use tuple::*;

pub fn solve(lines: &[&str]) -> (usize, usize) {
    let records = parse(lines);

    let relevant_box = AVolume::new((-50,-50,-50), (51,51,51));
    let pt_1 = solve_csg(&relevant_box, records.iter().cloned());

    let full_box = records.iter().fold(AVolume::default(),
        |result, item| item.1.op_union(&result));
    let pt_2 = solve_csg(&full_box, records.iter().cloned());

    (pt_1, pt_2)
}

fn solve_csg(relevant_box: &AVolume, records: impl Iterator<Item=(bool, AVolume)>) -> usize {
    let total = relevant_box.volume();

    let work = records.filter_map(|(on, b)| {
        let b = b.intersect(&relevant_box);
        (!b.is_empty()).then(|| (on, b))
    });

    let mut voxels = Vec::new();
    let mut negative = Vec::new();
    voxels.push(relevant_box.clone());
    let mut buffer = Vec::new();

    let mut background = false;

    for (on, operator) in work {
        let remaining = voxels.iter().map(|v| v.volume()).sum::<i64>();
        //dbg!(&voxels);
        let on_count = if background { remaining } else { total - remaining };

        println!("{} -> {:?} of {}, on={}", on, operator, voxels.len(), on_count);
        for v in voxels.drain(..) {
            if v.overlaps(&operator) {
                buffer.extend(v.subtract(&operator).filter(|b| !b.is_empty()));
            } else {
                buffer.push(v);
            }
        }
        std::mem::swap(&mut voxels, &mut buffer);
        if on == background {
            negative.push(operator);
            for v in voxels.iter() {
                for n in negative.drain(..) {
                    if n.overlaps(v) {
                        buffer.extend(n.subtract(&v).filter(|b| !b.is_empty()));
                    }
                    else {
                        buffer.push(n);
                    }
                }
                std::mem::swap(&mut negative, &mut buffer);
            }
            voxels.extend(negative.drain(..));
        }
        if voxels.is_empty() {
            voxels.push(relevant_box.clone());
            background = !background;
        }
    }

    let remaining = voxels.iter().map(|v| v.volume()).sum::<i64>();
    let result = if background { remaining } else { total - remaining };
    result as usize
}

fn parse(lines: &[&str]) -> Vec<(bool, AVolume)> {
    lines.iter().map(|l| {
        let mut main = l.split(' ');
        let on = main.next().unwrap() == "on";
        let pairs = main.next().unwrap().split(',').map(|coord| {
            coord[2..].split("..")
                .map(|n| n.parse::<i32>().unwrap())
                .collect_tuple::<(_,_)>().unwrap()
        }).collect_tuple::<(_,_,_)>().unwrap();
        let min = pairs.map(|c| c.0);
        let max = pairs.map(|c| c.1 + 1);
        (on, AVolume::new(min, max))
    }).collect()
}

type i32_3 = (i32, i32, i32);

fn max3(a: i32_3, b: i32_3) -> i32_3 {
    zip3(a, b).map(|(ae,be)| ae.max(be))
}

fn min3(a: i32_3, b: i32_3) -> i32_3 {
    zip3(a, b).map(|(ae,be)| ae.min(be))
}

fn zip3<T>(a: (T,T,T), b: (T,T,T)) -> ((T, T), (T, T), (T, T)) {
    ((a.0, b.0), (a.1, b.1), (a.2, b.2))
}

fn fold3<T, O>(v: (T,T,T), init: O, mut op: impl FnMut(O, T) -> O) -> O {
    let init = op(init, v.0);
    let init = op(init, v.1);
    op(init, v.2)
}

fn sub3<T: std::ops::Sub>(a: (T,T,T), b: (T,T,T)) -> (T::Output,T::Output,T::Output) {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

fn add3<T: std::ops::Add>(a: (T,T,T), b: (T,T,T)) -> (T::Output,T::Output,T::Output) {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn replace_at3<T>(mut v: (T,T,T), index: usize, op: impl FnOnce(&T) -> T) -> (T,T,T) {
    *v.get_mut(index).unwrap() = op(v.get(index).unwrap());
    v
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct AVolume {
    min: i32_3,
    max: i32_3
}

impl AVolume {
    fn new(min: i32_3, max: i32_3) -> Self {
        AVolume { min, max }
    }
    fn intersect(&self, other: &Self) -> Self {
        AVolume {
            min: max3(self.min, other.min),
            max: min3(self.max, other.max)
        }
    }
    // Union is a keyword :-/
    fn op_union(&self, other: &Self) -> Self {
        AVolume {
            min: min3(self.min, other.min),
            max: max3(self.max, other.max)
        }
    }
    fn overlaps(&self, other: &Self) -> bool {
        self.min.0 < other.max.0 &&
        self.min.1 < other.max.1 &&
        self.min.2 < other.max.2 &&
        self.max.0 > other.min.0 &&
        self.max.1 > other.min.1 &&
        self.max.2 > other.min.2
    }

    fn is_empty(&self) -> bool {
        self.volume() <= 0
    }
    fn volume(&self) -> i64 {
        let result = fold3(sub3(self.max, self.min), 1, |result, n| result * (n.abs() as i64));
        if (self.max.0 <= self.min.0) ||
            (self.max.1 <= self.min.1) ||
            (self.max.2 <= self.min.2) {
            -result
        }
        else {
            result
        }
    }
    fn inverse(&self) -> Self {
        Self::new(self.max, self.min)
    }
    fn split_plane(&self, plane: APlane) -> [Self; 2] {
        let axis = plane.axis.abs() as usize - 1;
        let distance = plane.distance;
        let new_max = replace_at3(self.max, axis, |&n| n.min(distance));
        let new_min = replace_at3(self.min, axis, |&n| n.max(distance));
        let mut positive = Self::new(new_min, self.max);
        let mut negative = Self::new(self.min, new_max);
        if plane.axis < 0 {
            std::mem::swap(&mut positive, &mut negative);
        }
        [positive, negative]
    }
    // Positive side facing out
    fn planes(&self) -> [APlane; 6] {
        [
            APlane::new(-1, self.min.0), APlane::new(1, self.max.0),
            APlane::new(-2, self.min.1), APlane::new(2, self.max.1),
            APlane::new(-3, self.min.2), APlane::new(3, self.max.2)
        ]
    }

    fn subtract(&self, other: &Self) -> impl Iterator<Item=Self> + '_ {
        let mut work = self.clone();
        other.planes().into_iter()
            .map(move |p| {
                let [output, remainder] = work.split_plane(p);
                work = remainder;
                output
            })
    }
}

impl Default for AVolume {
    fn default() -> Self {
        let min = i32::max_value();
        let max = i32::min_value();
        Self::new((min, min, min), (max, max, max))
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct APlane {
    axis: i8,
    distance: i32
}

impl APlane {
    fn new(axis: i8, distance: i32) -> Self {
        Self { axis, distance }
    }
}
