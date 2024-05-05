use std::ops::{Mul, Div};

use crate::{rotor::Rotor3, vector::Vector};

/// Defines a 3D bivector, generic float type
#[derive(Debug, Clone, Copy)]
pub struct Bivector<const DIM: usize> {
    vec_1: Vector<DIM>,
    vec_2: Vector<DIM>,
}

impl<const DIM: usize> Bivector<DIM> {
    // Calculates the magnitude of the current bivector
    pub fn magnitude(&self) -> f32 {
        let angle = self.vec_1.dot(self.vec_2).acos();
        self.vec_1.magnitude() * self.vec_2.magnitude() * angle.sin()
    }

    // Creates a normalized bivector with the same orientation as the current one
    pub fn to_normalized(&self) -> Self {
        if self.magnitude() == 0.0 {
            return Bivector {
                vec_1: Vector::default(),
                vec_2: Vector::default(),
            };
        }

        Bivector {
            vec_1: self.vec_1 / self.magnitude(),
            vec_2: self.vec_2 / self.magnitude(),
        }
    }

    /* // Normalizes the current bivector, preserves orientation
    pub fn normalize_self(&mut self) {
        if self.magnitude() != zero() {
            self.xy = self.xy / self.magnitude();
            self.yz = self.yz / self.magnitude();
            self.zx = self.zx / self.magnitude();
        }
    }

    /// Creates a rotor out of the bivector using exponentiation.
    ///
    /// ```e^ix = sin(x)+i*sin(x)```
    pub fn exponentiate(&self) -> Rotor3<T> {
        Rotor3 {
            scalar: self.magnitude().cos(),
            bivector: self.to_normalized() * self.magnitude().sin(),
        }
    } */
}

impl<const DIM: usize> Mul<f32> for Bivector<DIM> {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Bivector {
            vec_1: self.vec_1 * rhs,
            vec_2: self.vec_2 * rhs,
        }
    }
}

impl<const DIM: usize> Div<f32> for Bivector<DIM> {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        assert_ne!(rhs, 0.0);
        Bivector {
            vec_1: self.vec_1 / rhs,
            vec_2: self.vec_2 / rhs,
        }
    }
}
