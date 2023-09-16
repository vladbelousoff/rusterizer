use std::ops::*;
use std::fmt;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn neg(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn len_sqd(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> f32 {
        self.len_sqd().sqrt()
    }

    pub fn norm(&self) -> Self {
        *self / self.len()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;

    #[test]
    fn test_add() {
        let v1 = Vec3::new(2.0, 3.0, 4.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(v1 + v2, Vec3::new(3.0, 5.0, 7.0));
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3::new(2.0, 3.0, 4.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(v1 - v2, Vec3::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_mul() {
        let v = Vec3::new(2.0, 3.0, 4.0);

        assert_eq!(v * 11.0, Vec3::new(22.0, 33.0, 44.0));
        assert_eq!(22.0 * v, Vec3::new(44.0, 66.0, 88.0));
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3::new(2.0, 3.0, 4.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(v1.dot(&v2), 20.0);
        assert_eq!(v2.dot(&v1), 20.0);
    }

    #[test]
    fn test_div() {
        let v = Vec3::new(2.0, 3.0, 4.0);
        assert_eq!(v / 2.0, Vec3::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3::new(2.0, 3.0, 4.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(v1.cross(&v2), Vec3::new(1.0, -2.0, 1.0));
        assert_eq!(v2.cross(&v1), Vec3::new(-1.0, 2.0, -1.0));
    }

    #[test]
    fn test_neg() {
        assert_eq!(Vec3::new(2.0, 3.0, 4.0).neg(), Vec3::new(-2.0, -3.0, -4.0));
    }

    #[test]
    fn test_len() {
        let v1 = Vec3::new(4.0, 8.0, 1.0);
        assert_eq!(v1.len(), 9.0);
    }
}