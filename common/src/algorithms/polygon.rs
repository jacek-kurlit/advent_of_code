use std::ops::{Add, Mul, Sub};

use crate::coordinates::Vec2;

pub struct Polygon2<T> {
    coords: Vec<Vec2<T>>,
}

impl<T> Polygon2<T>
where
    T: Sub<Output = T> + Add<Output = T> + PartialOrd + Mul<Output = T> + Default + Copy,
{
    pub fn new(coords: Vec<Vec2<T>>) -> Self {
        Self { coords }
    }

    pub fn from_tuple(coords: Vec<(T, T)>) -> Self {
        Self {
            coords: coords.into_iter().map(|(x, y)| Vec2::new(x, y)).collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.coords.len()
    }

    pub fn is_empty(&self) -> bool {
        self.coords.is_empty()
    }

    pub fn get_coord(&self, index: usize) -> &Vec2<T> {
        &self.coords[index]
    }

    pub fn contains_point(&self, p: &Vec2<T>) -> bool {
        let between = |p, a, b| p >= a && p <= b || p <= a && p >= b;
        let mut inside = false;
        let mut i = self.coords.len() - 1;
        let mut j = 0;
        let zero = T::default();
        while j < self.coords.len() {
            let a = &self.coords[i];
            let b = &self.coords[j];
            i = j;
            j += 1;
            // corner cases
            if p.x == a.x && p.y == a.y || p.x == b.x && p.y == b.y {
                return true;
            }
            if a.y == b.y && p.y == a.y && between(&p.x, &a.x, &b.x) {
                return true;
            }

            if between(&p.y, &a.y, &b.y) {
                // if P inside the vertical range
                // filter out "ray pass vertex" problem by treating the line a little lower
                if p.y == a.y && b.y >= a.y || p.y == b.y && a.y >= b.y {
                    continue;
                }
                // calc cross product `PA X PB`, P lays on left side of AB if c > 0
                let c = (a.x - p.x) * (b.y - p.y) - (b.x - p.x) * (a.y - p.y);
                if c == zero {
                    return true;
                }
                if (a.y < b.y) == (c > zero) {
                    inside = !inside;
                }
            }
        }

        inside
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn polygon_contains_points() {
        let polygon = Polygon2::new(vec![
            Vec2::new(1, 7),
            Vec2::new(1, 11),
            Vec2::new(7, 11),
            Vec2::new(7, 9),
            Vec2::new(5, 9),
            Vec2::new(5, 2),
            Vec2::new(3, 2),
            Vec2::new(3, 7),
        ]);
        //true
        assert!(polygon.contains_point(&Vec2::new(1, 8)));
        assert!(polygon.contains_point(&Vec2::new(2, 8)));
        assert!(polygon.contains_point(&Vec2::new(5, 2)));
        assert!(polygon.contains_point(&Vec2::new(5, 3)));
        assert!(polygon.contains_point(&Vec2::new(6, 9)));
        //false
        assert!(!polygon.contains_point(&Vec2::new(0, 0)));
        assert!(!polygon.contains_point(&Vec2::new(6, 8)));
        assert!(!polygon.contains_point(&Vec2::new(1000, 1000)));
    }
}
