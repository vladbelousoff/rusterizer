use std::fmt;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

impl From<Vec3> for Color {
    fn from(vec: Vec3) -> Self {
        let max = f32::from(u8::MAX);

        let r = (max * vec.x) as u8;
        let g = (max * vec.y) as u8;
        let b = (max * vec.z) as u8;

        Self::new(r, g, b)
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;
    use crate::color::Color;

    #[test]
    fn test_from_vec3() {
        let v = Vec3::new(0.4, 0.6, 1.0);
        assert_eq!(Color::from(v), Color::new(102, 153, 255));
    }
}
