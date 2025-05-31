use std::{
    mem,
    ops::{Index, IndexMut, Mul},
};

use crate::math::vector3d::Vector3D;

#[derive(Debug, PartialEq)]
pub struct Matrix3D {
    // Column-major order,
    // so n[0] is the first column,
    // n[2][1] is the third column, second row etc.
    pub n: [[f32; 3]; 3],
}

impl Matrix3D {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        n00: f32,
        n01: f32,
        n02: f32,
        n10: f32,
        n11: f32,
        n12: f32,
        n20: f32,
        n21: f32,
        n22: f32,
    ) -> Self {
        Self {
            n: [[n00, n10, n20], [n01, n11, n21], [n02, n12, n22]],
        }
    }

    pub fn from_vectors(v1: &Vector3D, v2: &Vector3D, v3: &Vector3D) -> Self {
        Self {
            n: [
                [v1[0], v1[1], v1[2]],
                [v2[0], v2[1], v2[2]],
                [v3[0], v3[1], v3[2]],
            ],
        }
    }
}

// Converts (x,y) row-column order notation to column-major order
impl Index<(usize, usize)> for Matrix3D {
    type Output = f32;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.n[j][i]
    }
}

impl IndexMut<(usize, usize)> for Matrix3D {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.n[j][i]
    }
}

impl Index<usize> for Matrix3D {
    type Output = Vector3D;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { mem::transmute(&self.n[index]) }
    }
}

impl IndexMut<usize> for Matrix3D {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { mem::transmute(&mut self.n[index]) }
    }
}

impl Mul<Matrix3D> for Matrix3D {
    type Output = Matrix3D;

    fn mul(self, rhs: Matrix3D) -> Self::Output {
        Matrix3D::new(
            self.n[0][0] * rhs.n[0][0] + self.n[1][0] * rhs.n[0][1] + self.n[2][0] * rhs.n[0][2],
            self.n[0][0] * rhs.n[1][0] + self.n[1][0] * rhs.n[1][1] + self.n[2][0] * rhs.n[1][2],
            self.n[0][0] * rhs.n[2][0] + self.n[1][0] * rhs.n[2][1] + self.n[2][0] * rhs.n[2][2],
            self.n[0][1] * rhs.n[0][0] + self.n[1][1] * rhs.n[0][1] + self.n[2][1] * rhs.n[0][2],
            self.n[0][1] * rhs.n[1][0] + self.n[1][1] * rhs.n[1][1] + self.n[2][1] * rhs.n[1][2],
            self.n[0][1] * rhs.n[2][0] + self.n[1][1] * rhs.n[2][1] + self.n[2][1] * rhs.n[2][2],
            self.n[0][2] * rhs.n[0][0] + self.n[1][2] * rhs.n[0][1] + self.n[2][2] * rhs.n[0][2],
            self.n[0][2] * rhs.n[1][0] + self.n[1][2] * rhs.n[1][1] + self.n[2][2] * rhs.n[1][2],
            self.n[0][2] * rhs.n[2][0] + self.n[1][2] * rhs.n[2][1] + self.n[2][2] * rhs.n[2][2],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        let m = Matrix3D::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(m[0], Vector3D::new(1.0, 4.0, 7.0));
        assert_eq!(m[1], Vector3D::new(2.0, 5.0, 8.0));
        assert_eq!(m[2], Vector3D::new(3.0, 6.0, 9.0));

        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(0, 1)], 2.0);
        assert_eq!(m[(0, 2)], 3.0);
        assert_eq!(m[(1, 0)], 4.0);
        assert_eq!(m[(1, 1)], 5.0);
        assert_eq!(m[(1, 2)], 6.0);
        assert_eq!(m[(2, 0)], 7.0);
        assert_eq!(m[(2, 1)], 8.0);
        assert_eq!(m[(2, 2)], 9.0);
    }

    #[test]
    fn test_index_mut() {
        let mut m = Matrix3D::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        m[0] = Vector3D::new(10.0, 11.0, 12.0);
        assert_eq!(m[0], Vector3D::new(10.0, 11.0, 12.0));

        let v = &mut m[0];
        v[0] = 10.0;
        assert_eq!(m[0], Vector3D::new(10.0, 11.0, 12.0));

        m[(0, 0)] = 10.0;
        assert_eq!(m[(0, 0)], 10.0);

        let v00 = &mut m[(0, 0)];
        *v00 = 5.0;
        assert_eq!(m[(0, 0)], 5.0);

        let v = &mut m[(0, 0)];
        *v = 10.0;
        assert_eq!(m[(0, 0)], 10.0);
    }

    #[test]
    fn test_mul() {
        let m1 = Matrix3D::new(1.0, 3.0, -2.0, 0.0, -1.0, 4.0, 4.0, -3.0, 2.0);
        let m2 = Matrix3D::new(2.0, -2.0, 3.0, 1.0, 5.0, 3.0, -3.0, 4.0, 1.0);
        let m3 = m1 * m2;
        assert_eq!(
            m3,
            Matrix3D::new(11.0, 5.0, 10.0, -13.0, 11.0, 1.0, -1.0, -15.0, 5.0)
        );
        assert_eq!(m3[0], Vector3D::new(11.0, -13.0, -1.0));
        assert_eq!(m3[1], Vector3D::new(5.0, 11.0, -15.0));
        assert_eq!(m3[2], Vector3D::new(10.0, 1.0, 5.0));
    }
}
