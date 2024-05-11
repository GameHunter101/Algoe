use std::ops::Mul;

use super::bivector::Bivector;

#[derive(Debug)]
pub struct Rotor3 {
    pub scalar: f32,
    pub bivector: Bivector,
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
        let s_0 = rhs.scalar * self.scalar
            - rhs.bivector.xy * self.bivector.xy
            - rhs.bivector.yz * self.bivector.yz
            - rhs.bivector.zx * self.bivector.zx;

        let s_xy = -(rhs.scalar * self.bivector.xy
            + rhs.bivector.xy * self.scalar
            + rhs.bivector.yz * self.bivector.zx
            - rhs.bivector.zx * self.bivector.yz);

        let s_yz = -(rhs.scalar * self.bivector.yz - rhs.bivector.xy * self.bivector.zx
            + rhs.bivector.yz * self.scalar
            + rhs.bivector.zx * self.bivector.xy);

        let s_zx = -(rhs.scalar * self.bivector.zx + rhs.bivector.xy * self.bivector.yz
            - rhs.bivector.yz * self.bivector.xy
            + rhs.bivector.zx * self.scalar);

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
