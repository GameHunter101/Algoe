use std::ops::Mul;


use super::bivector::Bivector;

#[derive(Debug)]
pub struct Rotor3<T> {
    pub scalar: T,
    // pub bivector: Bivector3<T>,
}

/* impl<T: Float> Mul<&[T; 3]> for Rotor3<T> {
    type Output = [T; 3];
    fn mul(self, rhs: &[T; 3]) -> Self::Output {
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
} */
