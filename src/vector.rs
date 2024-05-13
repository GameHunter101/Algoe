use nalgebra::Vector3;

use crate::{bivector::Bivector, rotor::Rotor3};

pub trait GeometricOperations {
    fn wedge(&self, rhs: &Self) -> Bivector;
    fn dot(&self, rhs: &Self) -> f32;
    fn geometric(&self, rhs: &Self) -> Rotor3;
}

impl GeometricOperations for Vector3<f32> {
    fn wedge(&self, rhs: &Self) -> Bivector {
        let xy = self.x * rhs.y - self.y * rhs.x;
        let yz = self.y * rhs.z - self.z * rhs.y;
        let zx = self.z * rhs.x - self.x * rhs.z;

        Bivector { xy, yz, zx }
    }

    fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn geometric(&self, rhs: &Self) -> Rotor3 {
        Rotor3 {
            scalar: self.dot(rhs),
            bivector: self.wedge(rhs),
        }
    }
}
