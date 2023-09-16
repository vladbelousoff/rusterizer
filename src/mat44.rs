use crate::vec3::Vec3;
use std::ops::Mul;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Mat44([[f32; 4]; 4]);

impl Mul<&Vec3> for Mat44 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        let x = self.0[0][0] * rhs.x + self.0[0][1] * rhs.y + self.0[0][2] * rhs.z + self.0[0][3];
        let y = self.0[1][0] * rhs.x + self.0[1][1] * rhs.y + self.0[1][2] * rhs.z + self.0[1][3];
        let z = self.0[2][0] * rhs.x + self.0[2][1] * rhs.y + self.0[2][2] * rhs.z + self.0[2][3];
        let w = self.0[3][0] * rhs.x + self.0[3][1] * rhs.y + self.0[3][2] * rhs.z + self.0[3][3];
        Vec3::new(x / w, y / w, z / w)
    }
}

impl Mul for Mat44 {
    type Output = Mat44;

    fn mul(self, rhs: Mat44) -> Self::Output {
        let a = self;
        let b = rhs;
        Mat44([
            [
                a.0[0][0] * b.0[0][0]
                    + a.0[0][1] * b.0[1][0]
                    + a.0[0][2] * b.0[2][0]
                    + a.0[0][3] * b.0[3][0],
                a.0[0][0] * b.0[0][1]
                    + a.0[0][1] * b.0[1][1]
                    + a.0[0][2] * b.0[2][1]
                    + a.0[0][3] * b.0[3][1],
                a.0[0][0] * b.0[0][2]
                    + a.0[0][1] * b.0[1][2]
                    + a.0[0][2] * b.0[2][2]
                    + a.0[0][3] * b.0[3][2],
                a.0[0][0] * b.0[0][3]
                    + a.0[0][1] * b.0[1][3]
                    + a.0[0][2] * b.0[2][3]
                    + a.0[0][3] * b.0[3][3],
            ],
            [
                a.0[1][0] * b.0[0][0]
                    + a.0[1][1] * b.0[1][0]
                    + a.0[1][2] * b.0[2][0]
                    + a.0[1][3] * b.0[3][0],
                a.0[1][0] * b.0[0][1]
                    + a.0[1][1] * b.0[1][1]
                    + a.0[1][2] * b.0[2][1]
                    + a.0[1][3] * b.0[3][1],
                a.0[1][0] * b.0[0][2]
                    + a.0[1][1] * b.0[1][2]
                    + a.0[1][2] * b.0[2][2]
                    + a.0[1][3] * b.0[3][2],
                a.0[1][0] * b.0[0][3]
                    + a.0[1][1] * b.0[1][3]
                    + a.0[1][2] * b.0[2][3]
                    + a.0[1][3] * b.0[3][3],
            ],
            [
                a.0[2][0] * b.0[0][0]
                    + a.0[2][1] * b.0[1][0]
                    + a.0[2][2] * b.0[2][0]
                    + a.0[2][3] * b.0[3][0],
                a.0[2][0] * b.0[0][1]
                    + a.0[2][1] * b.0[1][1]
                    + a.0[2][2] * b.0[2][1]
                    + a.0[2][3] * b.0[3][1],
                a.0[2][0] * b.0[0][2]
                    + a.0[2][1] * b.0[1][2]
                    + a.0[2][2] * b.0[2][2]
                    + a.0[2][3] * b.0[3][2],
                a.0[2][0] * b.0[0][3]
                    + a.0[2][1] * b.0[1][3]
                    + a.0[2][2] * b.0[2][3]
                    + a.0[2][3] * b.0[3][3],
            ],
            [
                a.0[3][0] * b.0[0][0]
                    + a.0[3][1] * b.0[1][0]
                    + a.0[3][2] * b.0[2][0]
                    + a.0[3][3] * b.0[3][0],
                a.0[3][0] * b.0[0][1]
                    + a.0[3][1] * b.0[1][1]
                    + a.0[3][2] * b.0[2][1]
                    + a.0[3][3] * b.0[3][1],
                a.0[3][0] * b.0[0][2]
                    + a.0[3][1] * b.0[1][2]
                    + a.0[3][2] * b.0[2][2]
                    + a.0[3][3] * b.0[3][2],
                a.0[3][0] * b.0[0][3]
                    + a.0[3][1] * b.0[1][3]
                    + a.0[3][2] * b.0[2][3]
                    + a.0[3][3] * b.0[3][3],
            ],
        ])
    }
}

impl Mat44 {
    pub fn ident() -> Self {
        Self([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn trans(v: &Vec3) -> Self {
        Self([
            [1.0, 0.0, 0.0, v.x],
            [0.0, 1.0, 0.0, v.y],
            [0.0, 0.0, 1.0, v.z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn scale(v: &Vec3) -> Self {
        Self([
            [v.x, 0.0, 0.0, 0.0],
            [0.0, v.y, 0.0, 0.0],
            [0.0, 0.0, v.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotat(r: &Vec3, alpha: f32) -> Self {
        let (sin, cos) = alpha.sin_cos();
        let r = r.norm();

        Self([
            [
                cos + r.x * r.x * (1.0 - cos),
                r.x * r.y * (1.0 - cos) - r.z * sin,
                r.x * r.z * (1.0 - cos) + r.y * sin,
                0.0,
            ],
            [
                r.y * r.x * (1.0 - cos) + r.z * sin,
                cos + r.y * r.y * (1.0 - cos),
                r.y * r.z * (1.0 - cos) - r.x * sin,
                0.0,
            ],
            [
                r.z * r.x * (1.0 - cos) - r.y * sin,
                r.z * r.y * (1.0 - cos) + r.x * sin,
                cos + r.z * r.z * (1.0 - cos),
                0.0,
            ],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn persp(fov: f32, aspect: f32, n: f32, f: f32) -> Self {
        let a = aspect;
        let s = (fov * 0.5).tan();
        Self([
            [1.0 / (a * s), 0.0, 0.0, 0.0],
            [0.0, 1.0 / s, 0.0, 0.0],
            [0.0, 0.0, -f / (f - n), -f * n / (f - n)],
            [0.0, 0.0, -1.0, 0.0],
        ])
    }
}

#[cfg(test)]
mod tests {
    use crate::mat44::Mat44;
    use crate::vec3::Vec3;

    #[test]
    fn test_mul_ident() {
        let a = Mat44::ident();
        let b = Mat44::ident();
        let c = Mat44::ident();

        assert_eq!(c, a * b);
    }

    #[test]
    fn test_mul_scale_identity() {
        let a = Mat44::ident();
        let b = Mat44::scale(&Vec3::new(2.0, 2.0, 2.0));

        assert_eq!(b, a * b);
    }
}
