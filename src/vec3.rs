use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

use num::Float;

use vec2::Vector2;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { x, y, z }
    }

    pub fn vec2(self) -> Vector2<T> {
        Vector2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl<T> Vector3<T>
where
    T: Float,
{
    pub fn magnitude_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn magnitude(&self) -> T {
        self.magnitude_squared().sqrt()
    }

    pub fn normalize(&self) -> Vector3<T> {
        *self / self.magnitude()
    }

    pub fn cross(&self, rhs: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn dot(&self, rhs: Vector3<T>) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl<T> Add<Vector3<T>> for Vector3<T>
where
    T: Add<T, Output = T>,
{
    type Output = Vector3<T>;

    fn add(self, rhs: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> AddAssign<Vector3<T>> for Vector3<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Vector3<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T> Sub<Vector3<T>> for Vector3<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Vector3<T>;

    fn sub(self, rhs: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> SubAssign<Vector3<T>> for Vector3<T>
where
    T: SubAssign<T>,
{
    fn sub_assign(&mut self, rhs: Vector3<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T> Mul<T> for Vector3<T>
where
    T: Copy + Mul<T, Output = T>,
{
    type Output = Vector3<T>;

    fn mul(self, rhs: T) -> Vector3<T> {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> Mul<Vector3<T>> for Vector3<T>
where
    T: Mul<T, Output = T>,
{
    type Output = Vector3<T>;

    fn mul(self, rhs: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T> MulAssign<Vector3<T>> for Vector3<T>
where
    T: MulAssign<T>,
{
    fn mul_assign(&mut self, rhs: Vector3<T>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl<T> Div<T> for Vector3<T>
where
    T: Copy + Div<T, Output = T>,
{
    type Output = Vector3<T>;

    fn div(self, rhs: T) -> Vector3<T> {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T> Div<Vector3<T>> for Vector3<T>
where
    T: Div<T, Output = T>,
{
    type Output = Vector3<T>;

    fn div(self, rhs: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<T> DivAssign<Vector3<T>> for Vector3<T>
where
    T: DivAssign<T>,
{
    fn div_assign(&mut self, rhs: Vector3<T>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl<T> Neg for Vector3<T>
where
    T: Neg<Output = T>,
{
    type Output = Vector3<T>;

    fn neg(self) -> Self::Output {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
