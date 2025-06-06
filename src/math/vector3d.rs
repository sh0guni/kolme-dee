use std::ops::{Add, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

#[derive(Debug, PartialEq)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        self / self.magnitude()
    }
}

impl Add for &Vector3D {
    type Output = Vector3D;

    fn add(self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3D {
    type Output = Vector3D;

    fn sub(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Vector3D {
    type Output = Vector3D;

    fn neg(self) -> Self::Output {
        Vector3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f32> for &Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f32> for Vector3D {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f32> for &Vector3D {
    type Output = Vector3D;

    fn div(self, rhs: f32) -> Self::Output {
        Vector3D {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f32> for Vector3D {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Index<usize> for Vector3D {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vector3D {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);
        let v3 = &v1 + &v2;
        assert_eq!(v3.x, 5.0);
        assert_eq!(v3.y, 7.0);
        assert_eq!(v3.z, 9.0);
    }

    #[test]
    fn test_sub() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);
        let v3 = v1 - v2;
        assert_eq!(v3.x, -3.0);
        assert_eq!(v3.y, -3.0);
        assert_eq!(v3.z, -3.0);
    }

    #[test]
    fn test_neg() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = -v1;
        assert_eq!(v2.x, -1.0);
        assert_eq!(v2.y, -2.0);
        assert_eq!(v2.z, -3.0);
    }

    #[test]
    fn test_mul() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = &v1 * 2.0;
        assert_eq!(v2.x, 2.0);
        assert_eq!(v2.y, 4.0);
        assert_eq!(v2.z, 6.0);
    }

    #[test]
    fn test_mul_assign() {
        let mut v1 = Vector3D::new(1.0, 2.0, 3.0);
        v1 *= 2.0;
        assert_eq!(v1.x, 2.0);
        assert_eq!(v1.y, 4.0);
        assert_eq!(v1.z, 6.0);
    }

    #[test]
    fn test_div() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = &v1 / 2.0;
        assert_eq!(v2.x, 0.5);
        assert_eq!(v2.y, 1.0);
        assert_eq!(v2.z, 1.5);
    }

    #[test]
    fn test_div_by_zero() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = &v1 / 0.0;

        // Going by IEEE 754, dividing by zero results in infinity
        assert_eq!(v2.x, f32::INFINITY);
        assert_eq!(v2.y, f32::INFINITY);
        assert_eq!(v2.z, f32::INFINITY);
    }

    #[test]
    fn test_div_assign() {
        let mut v1 = Vector3D::new(1.0, 2.0, 3.0);
        v1 /= 2.0;
        assert_eq!(v1.x, 0.5);
        assert_eq!(v1.y, 1.0);
        assert_eq!(v1.z, 1.5);
    }

    #[test]
    fn test_index() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
    }

    #[test]
    #[should_panic]
    fn test_index_out_of_bounds() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        let _ = v[3];
    }

    #[test]
    fn test_index_mut() {
        let mut v = Vector3D::new(1.0, 2.0, 3.0);
        v[0] = 4.0;
        assert_eq!(v[0], 4.0);
    }

    #[test]
    #[should_panic]
    fn test_index_mut_out_of_bounds() {
        let mut v = Vector3D::new(1.0, 2.0, 3.0);
        v[3] = 4.0;
    }

    #[test]
    fn test_operator_chaining() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);
        let v3 = &v1 + &v2;
        let v4 = &v3 * 2.0;
        assert_eq!(v4.x, 10.0);
        assert_eq!(v4.y, 14.0);
        assert_eq!(v4.z, 18.0);

        let v5 = &(&v1 + &v2) * 2.0;
        assert_eq!(v5, v4);
    }

    #[test]
    fn test_magnitude() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        assert_eq!(v1.magnitude(), 3.7416575);

        let v2 = Vector3D::new(0.0, 0.0, 0.0);
        assert_eq!(v2.magnitude(), 0.0);

        let v3 = Vector3D::new(1.0, 1.0, 1.0);
        assert_eq!(v3.magnitude(), 1.7320508);
    }

    #[test]
    fn test_normalize() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(0.26726124, 0.5345225, 0.8017837);
        assert_eq!(v1.normalize(), v2);
    }
}
