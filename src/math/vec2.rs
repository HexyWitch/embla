use num_traits::{Float, Zero};
use std::cmp::PartialEq;
use std::convert::Into;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 { x, y }
    }
}

impl<T: Zero> Vec2<T> {
    pub fn zero() -> Vec2<T> {
        Vec2 {
            x: T::zero(),
            y: T::zero(),
        }
    }
}

impl<T: Float + From<f32>> Vec2<T> {
    pub fn with_angle(angle: T) -> Vec2<T> {
        Vec2 {
            x: angle.cos(),
            y: angle.sin(),
        }
    }
    pub fn angle(&self) -> T {
        self.y.atan2(self.x)
    }
    pub fn mag(&self) -> T {
        (self.x.powf(2.0.into()) + self.y.powf(2.0.into())).sqrt()
    }
    pub fn mag_squared(&self) -> T {
        self.x.powf(2.0.into()) + self.y.powf(2.0.into())
    }
    pub fn rotated(&self, a: T) -> Vec2<T> {
        let (a_cos, a_sin) = (a.cos(), a.sin());
        Vec2::new(
            self.x * a_cos + self.y * a_sin,
            self.y * a_cos + self.x * a_sin,
        )
    }
    pub fn normalized(&self) -> Vec2<T> {
        self.clone() / self.mag()
    }
}

impl<T> Into<(T, T)> for Vec2<T> {
    fn into(self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Vec2<T>;
    fn add(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: AddAssign> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub<Output = T>> Sub for Vec2<T> {
    type Output = Vec2<T>;
    fn sub(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: SubAssign> SubAssign for Vec2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Mul<Output = T>> Mul for Vec2<T> {
    type Output = Vec2<T>;
    fn mul(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T: MulAssign> MulAssign for Vec2<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vec2<T> {
    type Output = Vec2<T>;
    fn mul(self, rhs: T) -> Self {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Div<Output = T>> Div for Vec2<T> {
    type Output = Vec2<T>;
    fn div(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T: DivAssign> DivAssign for Vec2<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vec2<T> {
    type Output = Vec2<T>;
    fn div(self, rhs: T) -> Self {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: DivAssign + Copy> DivAssign<T> for Vec2<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T: PartialEq> PartialEq for Vec2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y)
    }
}

#[test]
fn vec2() {
    use std::f32;

    assert_eq!(Vec2::new(14, 46), Vec2::new(14, 46));

    let mut v1 = Vec2::new(6, 2);
    let v2 = Vec2::new(2, 2);

    assert_eq!(v1 + v2, Vec2::new(8, 4));
    assert_eq!(v1 - v2, Vec2::new(4, 0));
    assert_eq!(v1 * v2, Vec2::new(12, 4));
    assert_eq!(v1 / v2, Vec2::new(3, 1));

    v1 += v2;
    assert_eq!(v1, Vec2::new(8, 4));
    v1 -= v2;
    assert_eq!(v1, Vec2::new(6, 2));
    v1 *= v2;
    assert_eq!(v1, Vec2::new(12, 4));
    v1 /= v2;
    assert_eq!(v1, Vec2::new(6, 2));

    let v3 = Vec2::with_angle(f32::consts::PI * 0.5) * 10.0;
    assert!(v3.y >= 9.99);

    let v3 = v3.rotated(f32::consts::PI * 0.78);
    assert!(v3.mag_squared() > 99.9 && v3.mag_squared() < 100.1);
    assert!(v3.mag() > 9.99 && v3.mag() < 10.01);
}
