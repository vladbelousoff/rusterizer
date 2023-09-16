use crate::color::Color;
use crate::vec3::Vec3;
use std::cmp::{max, min};
use std::fmt;

pub struct Image(Vec<Vec<Color>>);

impl Image {
    pub fn new(w: usize, h: usize) -> Self {
        let v = vec![Color::new(0, 0, 0); w];
        Self(vec![v.clone(); h])
    }

    pub fn h(&self) -> i32 {
        self.0.len() as i32
    }

    pub fn w(&self) -> i32 {
        self.0[0].len() as i32
    }

    pub fn aspect(&self) -> f32 {
        self.w() as f32 / self.h() as f32
    }

    pub fn draw_point(&mut self, x: i32, y: i32, color: &Color) {
        if x < 0 || x >= self.w() {
            return;
        }

        if y < 0 || y >= self.h() {
            return;
        }

        self.0[y as usize][x as usize] = Color {
            r: color.r,
            g: color.g,
            b: color.b,
        };
    }

    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: &Color) {
        let mut dx = x1 as i32 - x0 as i32;
        let mut dy = y1 as i32 - y0 as i32;

        let sign_x = if dx > 0 {
            1
        } else {
            if dx < 0 {
                -1
            } else {
                0
            }
        };
        let sign_y = if dy > 0 {
            1
        } else {
            if dy < 0 {
                -1
            } else {
                0
            }
        };

        if dx < 0 {
            dx = -dx;
        }

        if dy < 0 {
            dy = -dy;
        }

        let pdx: i32;
        let pdy: i32;
        let es: i32;
        let el: i32;

        if dx > dy {
            pdx = sign_x;
            pdy = 0;
            es = dy;
            el = dx;
        } else {
            pdx = 0;
            pdy = sign_y;
            es = dx;
            el = dy;
        }

        let (mut x, mut y) = (x0 as i32, y0 as i32);
        let (mut e, mut t) = (el / 2, 0);

        self.draw_point(x, y, color);

        while t < el {
            e -= es;
            if e < 0 {
                e += el;
                x += sign_x;
                y += sign_y;
            } else {
                x += pdx;
                y += pdy;
            }
            t += 1;

            self.draw_point(x, y, color);
        }
    }

    fn x_coord(&self, x: f32) -> i32 {
        ((x + 1.0) * 0.5 * self.w() as f32) as i32
    }

    fn y_coord(&self, y: f32) -> i32 {
        ((1.0 - (y + 1.0) * 0.5) * self.h() as f32) as i32
    }

    fn is_inside_triangle(
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
    ) -> bool {
        let a = (x1 - x0) * (y2 - y1) - (x2 - x1) * (y1 - y0);
        let b = (x2 - x0) * (y3 - y2) - (x3 - x2) * (y2 - y0);
        let c = (x3 - x0) * (y1 - y3) - (x1 - x3) * (y3 - y0);

        return a.signum() == b.signum() && a.signum() == c.signum();
    }

    pub fn draw_triangle(&mut self, a: &Vec3, b: &Vec3, c: &Vec3, dir: &Vec3, color: &Color) {
        if a.x < -1.0 || a.x > 1.0 || a.y < -1.0 || a.y > 1.0 {
            return;
        }

        if b.x < -1.0 || b.x > 1.0 || b.y < -1.0 || b.y > 1.0 {
            return;
        }

        if c.x < -1.0 || c.x > 1.0 || c.y < -1.0 || c.y > 1.0 {
            return;
        }

        // Back-face culling

        let e0 = b - a;
        let e1 = c - a;
        let normal = e0.cross(&e1).norm();

        if normal.dot(&dir) < 0.0 {
            return;
        }

        let (ax, ay) = (self.x_coord(a.x), self.y_coord(a.y));
        let (bx, by) = (self.x_coord(b.x), self.y_coord(b.y));
        let (cx, cy) = (self.x_coord(c.x), self.y_coord(c.y));

        let min_x = min(min(ax, bx), cx);
        let max_x = max(max(ax, bx), cx);

        let min_y = min(min(ay, by), cy);
        let max_y = max(max(ay, by), cy);

        for x in min_x..max_x {
            for y in min_y..max_y {
                if Image::is_inside_triangle(x, y, ax, ay, bx, by, cx, cy) {
                    self.draw_point(x, y, color);
                }
            }
        }

        self.draw_line(ax, ay, bx, by, color);
        self.draw_line(bx, by, cx, cy, color);
        self.draw_line(cx, cy, ax, ay, color);
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "P3\n{} {}\n{}", self.w(), self.h(), u8::MAX)?;

        for column in &self.0 {
            for color in column {
                write!(f, "{} ", color)?;
            }
            writeln!(f)?;
        }

        writeln!(f)
    }
}
