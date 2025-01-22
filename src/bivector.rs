use std::ops::{Div, Mul};

use crate::rotor::Rotor3;

/// Defines a 3D bivector, generic float type
#[derive(Debug, Clone, Copy, Default)]
pub struct Bivector {
    pub xy: f32,
    pub yz: f32,
    pub zx: f32,
}

impl Bivector {
    pub fn new(xy: f32, yz: f32, zx: f32) -> Self {
        Self { xy, yz, zx }
    }

    // Calculates the magnitude of the current bivector
    pub fn magnitude(&self) -> f32 {
        f32::sqrt(self.xy * self.xy + self.yz * self.yz + self.zx * self.zx)
    }

    // Creates a normalized bivector with the same orientation as the current one
    pub fn to_normalized(&self) -> Self {
        if self.magnitude() == 0.0 {
            return Bivector {
                xy: 0.0,
                yz: 0.0,
                zx: 0.0,
            };
        }

        Bivector {
            xy: self.xy / self.magnitude(),
            yz: self.yz / self.magnitude(),
            zx: self.zx / self.magnitude(),
        }
    }

    // Normalizes the current bivector, preserves orientation
    pub fn normalize_self(&mut self) {
        if self.magnitude() == 0.0 {
            return;
        }
        self.xy /= self.magnitude();
        self.yz /= self.magnitude();
        self.zx /= self.magnitude();
    }

    /// Creates a rotor out of the bivector using exponentiation.
    ///
    /// ```e^ix = sin(x)+i*sin(x)```
    pub fn exponentiate(&self) -> Rotor3 {
        Rotor3 {
            scalar: self.magnitude().cos(),
            bivector: self.to_normalized() * self.magnitude().sin(),
        }
    }
}

impl Mul<f32> for Bivector {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Bivector {
            xy: self.xy * rhs,
            yz: self.yz * rhs,
            zx: self.zx * rhs,
        }
    }
}

impl Div<f32> for Bivector {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        assert_ne!(rhs, 0.0);
        Bivector {
            xy: self.xy / rhs,
            yz: self.yz / rhs,
            zx: self.zx / rhs,
        }
    }
}
