use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

use num::Float;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2 { x, y }
    }
}

impl<T> Vector2<T>
where
    T: Float,
{
    pub fn magnitude_squared(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    pub fn magnitude(&self) -> T {
        self.magnitude_squared().sqrt()
    }

    pub fn normalize(&self) -> Vector2<T> {
        *self / self.magnitude()
    }

    pub fn cross(&self, rhs: Vector2<T>) -> T {
        self.x * rhs.y - self.y * rhs.x
    }
}

impl<T> Add<Vector2<T>> for Vector2<T>
where
    T: Add<T, Output = T>,
{
    type Output = Vector2<T>;

    fn add(self, rhs: Vector2<T>) -> Vector2<T> {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> AddAssign<Vector2<T>> for Vector2<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Vector2<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Sub<Vector2<T>> for Vector2<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Vector2<T>;

    fn sub(self, rhs: Vector2<T>) -> Vector2<T> {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> SubAssign<Vector2<T>> for Vector2<T>
where
    T: SubAssign<T>,
{
    fn sub_assign(&mut self, rhs: Vector2<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> Mul<T> for Vector2<T>
where
    T: Copy + Mul<T, Output = T>,
{
    type Output = Vector2<T>;

    fn mul(self, rhs: T) -> Vector2<T> {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Mul<Vector2<T>> for Vector2<T>
where
    T: Mul<T, Output = T>,
{
    type Output = Vector2<T>;

    fn mul(self, rhs: Vector2<T>) -> Vector2<T> {
        Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T> MulAssign<Vector2<T>> for Vector2<T>
where
    T: MulAssign<T>,
{
    fn mul_assign(&mut self, rhs: Vector2<T>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T> Div<T> for Vector2<T>
where
    T: Copy + Div<T, Output = T>,
{
    type Output = Vector2<T>;

    fn div(self, rhs: T) -> Vector2<T> {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T> Div<Vector2<T>> for Vector2<T>
where
    T: Div<T, Output = T>,
{
    type Output = Vector2<T>;

    fn div(self, rhs: Vector2<T>) -> Vector2<T> {
        Vector2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T> DivAssign<Vector2<T>> for Vector2<T>
where
    T: DivAssign<T>,
{
    fn div_assign(&mut self, rhs: Vector2<T>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<T> Neg for Vector2<T>
where
    T: Neg<Output = T>,
{
    type Output = Vector2<T>;

    fn neg(self) -> Self::Output {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}
