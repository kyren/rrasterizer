use std::ops::{Add, Mul, MulAssign};

use num::Float;

use vec3::Vector3;
use vec4::Vector4;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Matrix4<T> {
    e11: T,
    e12: T,
    e13: T,
    e14: T,

    e21: T,
    e22: T,
    e23: T,
    e24: T,

    e31: T,
    e32: T,
    e33: T,
    e34: T,

    e41: T,
    e42: T,
    e43: T,
    e44: T,
}

impl<T> Matrix4<T> {
    pub fn new(
        e11: T,
        e12: T,
        e13: T,
        e14: T,
        e21: T,
        e22: T,
        e23: T,
        e24: T,
        e31: T,
        e32: T,
        e33: T,
        e34: T,
        e41: T,
        e42: T,
        e43: T,
        e44: T,
    ) -> Matrix4<T> {
        Matrix4 {
            e11,
            e12,
            e13,
            e14,
            e21,
            e22,
            e23,
            e24,
            e31,
            e32,
            e33,
            e34,
            e41,
            e42,
            e43,
            e44,
        }
    }

    pub fn transpose(self) -> Matrix4<T> {
        Matrix4 {
            e11: self.e11,
            e12: self.e21,
            e13: self.e31,
            e14: self.e41,
            e21: self.e12,
            e22: self.e22,
            e23: self.e32,
            e24: self.e42,
            e31: self.e13,
            e32: self.e23,
            e33: self.e33,
            e34: self.e43,
            e41: self.e14,
            e42: self.e24,
            e43: self.e34,
            e44: self.e44,
        }
    }
}

impl<T> Matrix4<T>
where
    T: Float,
{
    pub fn identity() -> Matrix4<T> {
        Matrix4 {
            e11: T::one(),
            e12: T::zero(),
            e13: T::zero(),
            e14: T::zero(),
            e21: T::zero(),
            e22: T::one(),
            e23: T::zero(),
            e24: T::zero(),
            e31: T::zero(),
            e32: T::zero(),
            e33: T::one(),
            e34: T::zero(),
            e41: T::zero(),
            e42: T::zero(),
            e43: T::zero(),
            e44: T::one(),
        }
    }

    pub fn rotation(rotation: Vector3<T>) -> Matrix4<T> {
        let rotx = Matrix4 {
            e11: T::one(),
            e12: T::zero(),
            e13: T::zero(),
            e14: T::zero(),
            e21: T::zero(),
            e22: rotation.x.cos(),
            e23: -rotation.x.sin(),
            e24: T::zero(),
            e31: T::zero(),
            e32: rotation.x.sin(),
            e33: rotation.x.cos(),
            e34: T::zero(),
            e41: T::zero(),
            e42: T::zero(),
            e43: T::zero(),
            e44: T::one(),
        };

        let roty = Matrix4 {
            e11: rotation.y.cos(),
            e12: T::zero(),
            e13: rotation.y.sin(),
            e14: T::zero(),
            e21: T::zero(),
            e22: T::one(),
            e23: T::zero(),
            e24: T::zero(),
            e31: -rotation.y.sin(),
            e32: T::zero(),
            e33: rotation.y.cos(),
            e34: T::zero(),
            e41: T::zero(),
            e42: T::zero(),
            e43: T::zero(),
            e44: T::one(),
        };

        let rotz = Matrix4 {
            e11: rotation.z.cos(),
            e12: -rotation.z.sin(),
            e13: T::zero(),
            e14: T::zero(),
            e21: rotation.z.sin(),
            e22: rotation.z.cos(),
            e23: T::zero(),
            e24: T::zero(),
            e31: T::zero(),
            e32: T::zero(),
            e33: T::one(),
            e34: T::zero(),
            e41: T::zero(),
            e42: T::zero(),
            e43: T::zero(),
            e44: T::one(),
        };

        rotz * roty * rotx
    }

    pub fn rotation_around(rotation: Vector3<T>, pos: Vector3<T>) -> Matrix4<T> {
        Self::translation(pos) * Self::rotation(rotation) * Self::translation(-pos)
    }

    pub fn scaling(scale: Vector3<T>) -> Matrix4<T> {
        Matrix4 {
            e11: scale.x,
            e12: T::zero(),
            e13: T::zero(),
            e14: T::zero(),
            e21: T::zero(),
            e22: scale.y,
            e23: T::zero(),
            e24: T::zero(),
            e31: T::zero(),
            e32: T::zero(),
            e33: scale.z,
            e34: T::zero(),
            e41: T::zero(),
            e42: T::zero(),
            e43: T::zero(),
            e44: T::one(),
        }
    }

    pub fn scaling_around(scale: Vector3<T>, pos: Vector3<T>) -> Matrix4<T> {
        Self::translation(pos) * Self::scaling(scale) * Self::translation(-pos)
    }

    pub fn translation(pos: Vector3<T>) -> Matrix4<T> {
        Matrix4 {
            e11: T::one(),
            e12: T::zero(),
            e13: T::zero(),
            e14: pos.x,
            e21: T::zero(),
            e22: T::one(),
            e23: T::zero(),
            e24: pos.y,
            e31: T::zero(),
            e32: T::zero(),
            e33: T::one(),
            e34: pos.z,
            e41: T::zero(),
            e42: T::zero(),
            e43: T::zero(),
            e44: T::one(),
        }
    }

    pub fn perspective(width: T, height: T, far: T, near: T) -> Matrix4<T> {
        let two = T::from(2).unwrap();
        Matrix4 {
            e11: (two * near) / width,
            e12: T::zero(),
            e13: T::zero(),
            e14: T::zero(),
            e21: T::zero(),
            e22: (two * near) / height,
            e23: T::zero(),
            e24: T::zero(),
            e31: T::zero(),
            e32: T::zero(),
            e33: -(far + near) / (far - near),
            e34: -(two * far * near) / (far - near),
            e41: T::zero(),
            e42: T::zero(),
            e43: -T::one(),
            e44: T::zero(),
        }
    }

    pub fn transform(&self, pos: Vector3<T>) -> Vector3<T> {
        let v = *self * Vector4::new(pos.x, pos.y, pos.z, T::one());
        v.vec3() / v.w
    }
}

impl<T> Mul<Matrix4<T>> for Matrix4<T>
where
    T: Copy + Add<T, Output = T> + Mul<T, Output = T>,
{
    type Output = Matrix4<T>;

    fn mul(self, rhs: Matrix4<T>) -> Matrix4<T> {
        Matrix4 {
            e11: self.e11 * rhs.e11 + self.e12 * rhs.e21 + self.e13 * rhs.e31 + self.e14 * rhs.e41,
            e12: self.e11 * rhs.e12 + self.e12 * rhs.e22 + self.e13 * rhs.e32 + self.e14 * rhs.e42,
            e13: self.e11 * rhs.e13 + self.e12 * rhs.e23 + self.e13 * rhs.e33 + self.e14 * rhs.e43,
            e14: self.e11 * rhs.e14 + self.e12 * rhs.e24 + self.e13 * rhs.e34 + self.e14 * rhs.e44,

            e21: self.e21 * rhs.e11 + self.e22 * rhs.e21 + self.e23 * rhs.e31 + self.e24 * rhs.e41,
            e22: self.e21 * rhs.e12 + self.e22 * rhs.e22 + self.e23 * rhs.e32 + self.e24 * rhs.e42,
            e23: self.e21 * rhs.e13 + self.e22 * rhs.e23 + self.e23 * rhs.e33 + self.e24 * rhs.e43,
            e24: self.e21 * rhs.e14 + self.e22 * rhs.e24 + self.e23 * rhs.e34 + self.e24 * rhs.e44,

            e31: self.e31 * rhs.e11 + self.e32 * rhs.e21 + self.e33 * rhs.e31 + self.e34 * rhs.e41,
            e32: self.e31 * rhs.e12 + self.e32 * rhs.e22 + self.e33 * rhs.e32 + self.e34 * rhs.e42,
            e33: self.e31 * rhs.e13 + self.e32 * rhs.e23 + self.e33 * rhs.e33 + self.e34 * rhs.e43,
            e34: self.e31 * rhs.e14 + self.e32 * rhs.e24 + self.e33 * rhs.e34 + self.e34 * rhs.e44,

            e41: self.e41 * rhs.e11 + self.e42 * rhs.e21 + self.e43 * rhs.e31 + self.e44 * rhs.e41,
            e42: self.e41 * rhs.e12 + self.e42 * rhs.e22 + self.e43 * rhs.e32 + self.e44 * rhs.e42,
            e43: self.e41 * rhs.e13 + self.e42 * rhs.e23 + self.e43 * rhs.e33 + self.e44 * rhs.e43,
            e44: self.e41 * rhs.e14 + self.e42 * rhs.e24 + self.e43 * rhs.e34 + self.e44 * rhs.e44,
        }
    }
}

impl<T> MulAssign<Matrix4<T>> for Matrix4<T>
where
    T: Copy + Add<T, Output = T> + Mul<T, Output = T>,
{
    fn mul_assign(&mut self, rhs: Matrix4<T>) {
        *self = *self * rhs;
    }
}

impl<T> Mul<Vector4<T>> for Matrix4<T>
where
    T: Copy + Add<T, Output = T> + Mul<T, Output = T>,
{
    type Output = Vector4<T>;

    fn mul(self, rhs: Vector4<T>) -> Vector4<T> {
        Vector4 {
            x: self.e11 * rhs.x + self.e12 * rhs.y + self.e13 * rhs.z + self.e14 * rhs.w,
            y: self.e21 * rhs.x + self.e22 * rhs.y + self.e23 * rhs.z + self.e24 * rhs.w,
            z: self.e31 * rhs.x + self.e32 * rhs.y + self.e33 * rhs.z + self.e34 * rhs.w,
            w: self.e41 * rhs.x + self.e42 * rhs.y + self.e43 * rhs.z + self.e44 * rhs.w,
        }
    }
}
