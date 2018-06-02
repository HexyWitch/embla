use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, MulAssign, Sub};

use super::semi_ord::SemiOrd;
use super::vec2::Vec2;

#[derive(Clone)]
pub struct Rect<T> {
    pub min: Vec2<T>,
    pub max: Vec2<T>,
}

impl<T> Rect<T> {
    pub fn new(min: Vec2<T>, max: Vec2<T>) -> Rect<T> {
        Rect { min, max }
    }

    pub fn from_bounds(x_min: T, x_max: T, y_min: T, y_max: T) -> Rect<T> {
        Rect {
            min: Vec2::new(x_min, x_max),
            max: Vec2::new(y_min, y_max),
        }
    }
}

impl<T: Add<Output = T> + Copy> Rect<T> {
    pub fn with_size(min: Vec2<T>, size: Vec2<T>) -> Rect<T> {
        Rect {
            min: min,
            max: min + size,
        }
    }
}

impl<T: Sub<Output = T> + Copy> Rect<T> {
    pub fn size(&self) -> Vec2<T> {
        self.max - self.min
    }
}

impl<T> Rect<T>
where
    T: Add<Output = T> + Sub<Output = T> + Div<Output = T> + From<i32> + Copy,
{
    pub fn with_center(center: Vec2<T>, size: Vec2<T>) -> Rect<T> {
        let half_size = size / T::from(2);
        Rect {
            min: center - half_size,
            max: center + half_size,
        }
    }
}

impl<T: AddAssign + Copy> Rect<T> {
    pub fn translate(&mut self, v: Vec2<T>) -> &mut Self {
        self.min += v;
        self.max += v;
        self
    }
}

impl<T: MulAssign + Copy> Rect<T> {
    pub fn scale(&mut self, v: Vec2<T>) -> &mut Self {
        self.min *= v;
        self.max *= v;
        self
    }
}

impl<T: SemiOrd> Rect<T> {
    pub fn contains(&self, v: Vec2<T>) -> bool {
        !(v.x.semi_cmp(&self.min.x) == Ordering::Less
            || v.y.semi_cmp(&self.min.y) == Ordering::Less
            || v.x.semi_cmp(&self.max.x) == Ordering::Greater
            || v.y.semi_cmp(&self.max.y) == Ordering::Greater)
    }

    pub fn overlaps(&self, r: Rect<T>) -> bool {
        !(r.max.x.semi_cmp(&self.min.x) == Ordering::Less
            || r.max.y.semi_cmp(&self.min.y) == Ordering::Less
            || r.min.x.semi_cmp(&self.max.x) == Ordering::Greater
            || r.min.y.semi_cmp(&self.max.y) == Ordering::Greater)
    }
}

#[test]
fn rect() {
    let rects = vec![
        Rect::new(Vec2::new(2, 2), Vec2::new(8, 8)),
        Rect::from_bounds(2, 2, 8, 8),
        Rect::with_size(Vec2::new(2, 2), Vec2::new(6, 6)),
        Rect::with_center(Vec2::new(5, 5), Vec2::new(6, 6)),
    ];
    assert_eq!(rects[0].min, rects[1].min);
    assert_eq!(rects[0].max, rects[1].max);
    assert_eq!(rects[0].min, rects[2].min);
    assert_eq!(rects[0].max, rects[2].max);
    assert_eq!(rects[0].min, rects[3].min);
    assert_eq!(rects[0].max, rects[3].max);

    let mut r1 = rects[0].clone();
    r1.translate(Vec2::new(4, 2));
    assert_eq!(r1.min, Vec2::new(6, 4));
    assert_eq!(r1.max, Vec2::new(12, 10));

    r1.scale(Vec2::new(3, 2));
    assert_eq!(r1.min, Vec2::new(18, 8));
    assert_eq!(r1.max, Vec2::new(36, 20));

    assert_eq!(r1.contains(Vec2::new(20, 15)), true);
    assert_eq!(r1.contains(Vec2::new(20, 21)), false);
    assert_eq!(r1.contains(Vec2::new(20, 5)), false);
    assert_eq!(r1.contains(Vec2::new(17, 15)), false);
    assert_eq!(r1.contains(Vec2::new(37, 15)), false);

    let mut r2 = r1.clone();
    r2.translate(r1.size() / 2);
    assert_eq!(r1.overlaps(r2), true);

    let mut r3 = r1.clone();
    r3.translate(r1.size() + Vec2::new(1, 1));
    assert_eq!(r1.overlaps(r3), false);
}
