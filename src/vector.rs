use std::ops::{Div, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Vector<const DIM: usize> {
    pub data: [f32; DIM],
}

impl<const DIM: usize> Default for Vector<DIM> {
    fn default() -> Self {
        Vector { data: [0.0; DIM] }
    }
}

impl<const DIM: usize> Vector<DIM> {
    pub fn new(data: [f32; DIM]) -> Self {
        Self { data }
    }

    pub fn magnitude(&self) -> f32 {
        let mut sum: f32 = 0.0;

        for elem in &self.data {
            sum += *elem * *elem;
        }

        sum.sqrt()
    }

    pub fn normalize(&self) -> Self {
        Vector {
            data: self.data.map(|elem| elem / self.magnitude()),
        }
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        let mut sum: f32 = 0.0;
        for i in 0..DIM {
            sum += self.data[i] * rhs.data[i];
        }
        sum
    }
}

impl<const DIM: usize> Mul<f32> for Vector<DIM> {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Vector {
            data: self.data.map(|e| e * rhs),
        }
    }
}

impl<const DIM: usize> Div<f32> for Vector<DIM> {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        assert_ne!(rhs, 0.0);

        Vector {
            data: self.data.map(|e| e / rhs),
        }
    }
}

impl<const DIM: usize> Mul<f32> for &Vector<DIM> {
    type Output = Vector<DIM>;
    fn mul(self, rhs: f32) -> Self::Output {
        Vector {
            data: self.data.map(|e| e * rhs),
        }
    }
}

impl<const DIM: usize> Div<f32> for &Vector<DIM> {
    type Output = Vector<DIM>;
    fn div(self, rhs: f32) -> Self::Output {
        assert_ne!(rhs, 0.0);

        Vector {
            data: self.data.map(|e| e / rhs),
        }
    }
}
