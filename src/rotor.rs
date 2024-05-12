use std::ops::Mul;

use super::bivector::Bivector;

#[derive(Debug, Clone, Copy, Default)]
pub struct Rotor3 {
    pub scalar: f32,
    pub bivector: Bivector,
}

impl Rotor3 {
    pub fn new(scalar: f32, bivector: Bivector) -> Self {
        Self { scalar, bivector }
    }
}

impl Mul<&[f32; 3]> for Rotor3 {
    type Output = [f32; 3];
    fn mul(self, rhs: &[f32; 3]) -> Self::Output {
        let s_x = rhs[0] * self.scalar + rhs[1] * self.bivector.xy - rhs[2] * self.bivector.zx;
        let s_y = rhs[1] * self.scalar - rhs[0] * self.bivector.xy + rhs[2] * self.bivector.yz;
        let s_z = rhs[2] * self.scalar - rhs[1] * self.bivector.yz + rhs[0] * self.bivector.zx;
        let s_xyz =
            rhs[0] * self.bivector.yz + rhs[1] * self.bivector.zx - rhs[2] * self.bivector.xy;

        let v_x = s_x * self.scalar + s_y * self.bivector.xy - s_z * self.bivector.zx
            + s_xyz * self.bivector.yz;

        let v_y = s_y * self.scalar - s_x * self.bivector.xy
            + s_z * self.bivector.yz
            + s_xyz * self.bivector.zx;

        let v_z = s_z * self.scalar + s_x * self.bivector.zx - s_y * self.bivector.yz
            + s_xyz * self.bivector.xy;

        [v_x, v_y, v_z]
    }
}

impl Mul<Rotor3> for Rotor3 {
    type Output = Rotor3;
    fn mul(self, rhs: Rotor3) -> Self::Output {
        let s_0 = self.scalar * rhs.scalar
            - self.bivector.xy * rhs.bivector.xy
            - self.bivector.yz * rhs.bivector.yz
            - self.bivector.zx * rhs.bivector.zx;

        let s_xy = self.scalar * rhs.bivector.xy + self.bivector.xy * rhs.scalar
            - self.bivector.yz * rhs.bivector.zx
            + self.bivector.zx * rhs.bivector.yz;

        let s_yz = self.scalar * rhs.bivector.yz
            + self.bivector.xy * rhs.bivector.zx
            + self.bivector.yz * rhs.scalar
            - self.bivector.zx * rhs.bivector.xy;

        let s_zx = self.scalar * rhs.bivector.zx - self.bivector.xy * rhs.bivector.yz
            + self.bivector.yz * rhs.bivector.xy
            + self.bivector.zx * rhs.scalar;

        Rotor3 {
            scalar: s_0,
            bivector: Bivector {
                xy: s_xy,
                yz: s_yz,
                zx: s_zx,
            },
        }
    }
}

mod test {
    #![allow(unused)]
    use crate::bivector::Bivector;
    use std::f32::consts;

    fn comp_float(mut lhs: f32, mut rhs: f32) {
        let diff = (lhs - rhs).abs();
        lhs = lhs.abs();
        rhs = rhs.abs();

        let largest = if rhs > lhs { rhs } else { lhs };

        dbg!(diff, largest * std::f32::EPSILON);
        assert!(diff <= largest * std::f32::EPSILON);
    }

    fn comp_vec(lhs: &[f32; 3], rhs: &[f32; 3]) {
        for i in 0..3 {
            comp_float(lhs[i], rhs[i]);
        }
    }

    #[test]
    fn test_vector_rotation() {
        let vec = [1.0, 0.0, 0.0];
        let rotor = (Bivector::new(1.0, 0.0, 0.0) * -consts::FRAC_PI_8).exponentiate();

        let theoretical_vec = [consts::SQRT_2 / 2.0, consts::SQRT_2 / 2.0, 0.0];

        let rotated_vec = rotor * &vec;

        comp_vec(&rotated_vec, &theoretical_vec);
    }

    #[test]
    fn test_rotor_rotation() {
        let vec = [1.0, 0.0, 0.0];
        let rotor = (Bivector::new(1.0, 0.0, 0.0) * -consts::FRAC_PI_8).exponentiate();

        let rotated = rotor * rotor;

        let theoretical_vec = [0.0, 1.0, 0.0];

        let rotated_vec = rotated * &vec;

        dbg!(rotated_vec, theoretical_vec);

        comp_vec(&rotated_vec, &theoretical_vec);
    }
}
