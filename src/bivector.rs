use std::ops::Mul;

use num_traits::{zero, Float};

use crate::rotor::Rotor3;

/// Defines a 3D bivector, generic float type
#[derive(Debug, Clone, Copy)]
pub struct Bivector3<T> {
    pub xy: T,
    pub yz: T,
    pub zx: T,
}

impl<T: Float> Bivector3<T> {
    // Calculates the magnitude of the current bivector
    pub fn magnitude(&self) -> T {
        (self.xy * self.xy + self.yz * self.yz + self.zx * self.zx).sqrt()
    }

    // Creates a normalized bivector with the same orientation as the current one
    pub fn to_normalized(&self) -> Self {
        if self.magnitude() == zero() {
            return Bivector3::<T> {
                xy: zero(),
                yz: zero(),
                zx: zero(),
            };
        }

        Bivector3 {
            xy: self.xy / self.magnitude(),
            yz: self.yz / self.magnitude(),
            zx: self.zx / self.magnitude(),
        }
    }

    // Normalizes the current bivector, preserves orientation
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
    }
}

impl<T: Float> Mul<T> for Bivector3<T> {
    type Output = Bivector3<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Bivector3 {
            xy: rhs * self.xy,
            yz: rhs * self.yz,
            zx: rhs * self.zx,
        }
    }
}
