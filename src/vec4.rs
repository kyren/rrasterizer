use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

use num::Float;

use vec3::Vector3;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vector4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Vector4<T> {
        Vector4 { x, y, z, w }
    }

    pub fn vec3(self) -> Vector3<T> {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl<T> Vector4<T>
where
    T: Float,
{
    pub fn magnitude_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn magnitude(&self) -> T {
        self.magnitude_squared().sqrt()
    }

    pub fn normalize(&self) -> Vector4<T> {
        *self / self.magnitude()
    }
}

impl<T> Add<Vector4<T>> for Vector4<T>
where
    T: Add<T, Output = T>,
{
    type Output = Vector4<T>;

    fn add(self, rhs: Vector4<T>) -> Vector4<T> {
        Vector4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl<T> AddAssign<Vector4<T>> for Vector4<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Vector4<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl<T> Sub<Vector4<T>> for Vector4<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Vector4<T>;

    fn sub(self, rhs: Vector4<T>) -> Vector4<T> {
        Vector4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl<T> SubAssign<Vector4<T>> for Vector4<T>
where
    T: SubAssign<T>,
{
    fn sub_assign(&mut self, rhs: Vector4<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

impl<T> Mul<T> for Vector4<T>
where
    T: Copy + Mul<T, Output = T>,
{
    type Output = Vector4<T>;

    fn mul(self, rhs: T) -> Vector4<T> {
        Vector4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl<T> Mul<Vector4<T>> for Vector4<T>
where
    T: Mul<T, Output = T>,
{
    type Output = Vector4<T>;

    fn mul(self, rhs: Vector4<T>) -> Vector4<T> {
        Vector4 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}

impl<T> MulAssign<Vector4<T>> for Vector4<T>
where
    T: MulAssign<T>,
{
    fn mul_assign(&mut self, rhs: Vector4<T>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}

impl<T> Div<T> for Vector4<T>
where
    T: Copy + Div<T, Output = T>,
{
    type Output = Vector4<T>;

    fn div(self, rhs: T) -> Vector4<T> {
        Vector4 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl<T> Div<Vector4<T>> for Vector4<T>
where
    T: Div<T, Output = T>,
{
    type Output = Vector4<T>;

    fn div(self, rhs: Vector4<T>) -> Vector4<T> {
        Vector4 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w,
        }
    }
}

impl<T> DivAssign<Vector4<T>> for Vector4<T>
where
    T: DivAssign<T>,
{
    fn div_assign(&mut self, rhs: Vector4<T>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self.w /= rhs.w;
    }
}

impl<T> Neg for Vector4<T>
where
    T: Neg<Output = T>,
{
    type Output = Vector4<T>;

    fn neg(self) -> Self::Output {
        Vector4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}
