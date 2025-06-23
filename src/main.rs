use crate::color::Color;
use crate::image::Image;
use crate::mat44::Mat44;
use crate::vec3::Vec3;
use rand::Rng;

mod color;
mod image;
mod mat44;
mod vec3;

fn main() {
    // Generate a UV sphere
    let lat_segments = 20;
    let lon_segments = 20;
    let mut vertices = Vec::new();
    for i in 0..=lat_segments {
        let theta = std::f32::consts::PI * (i as f32) / (lat_segments as f32);
        let y = theta.cos();
        let r = theta.sin();
        for j in 0..=lon_segments {
            let phi = 2.0 * std::f32::consts::PI * (j as f32) / (lon_segments as f32);
            let x = r * phi.cos();
            let z = r * phi.sin();
            vertices.push(Vec3::new(x * 1.5, y * 1.5, z * 1.5));
        }
    }

    // Generate indices for triangles
    let mut indices = Vec::new();
    for i in 0..lat_segments {
        for j in 0..lon_segments {
            let a = i * (lon_segments + 1) + j;
            let b = a + lon_segments + 1;
            let c = a + 1;
            let d = b + 1;
            // Each quad is split into two triangles
            indices.push((a, b, c));
            indices.push((c, b, d));
        }
    }

    let sun_dir = Vec3::new(2.0, -0.25, -1.0);

    let mut image = Image::new(800, 600);
    let mut rng = rand::thread_rng();
    let persp = Mat44::persp((45.0_f32).to_radians(), image.aspect(), 0.1, 100.0);

    for x in -2..=2 {
        for y in -2..=2 {
            let trans = Mat44::trans(&Vec3::new(x as f32 * 4.0, y as f32 * 4.0, -30.0));
            let angle = rng.gen_range(-45.0_f32..45.0_f32);
            let rotat = Mat44::rotat(&Vec3::new(1.0, 1.0, 1.0), angle.to_radians());

            let t_vertices: Vec<Vec3> = vertices.iter().map(|v| persp * trans * rotat * v).collect();

            for (a, b, c) in &indices {
                let v0 = &vertices[a.clone()];
                let v1 = &vertices[b.clone()];
                let v2 = &vertices[c.clone()];

                let e0 = v1 - v0;
                let e1 = v2 - v0;
                let n = e0.cross(&e1).norm();

                let lum = n.dot(&sun_dir.neg().norm()).clamp(0.0, 1.0);
                let color = Vec3::new(1.0, 1.0, 1.0) * (0.3 + 0.7 * lum);
                let color = Color::from(color);

                image.draw_triangle(
                    &t_vertices[a.clone()],
                    &t_vertices[b.clone()],
                    &t_vertices[c.clone()],
                    &Vec3::new(0.0, 0.0, 1.0),
                    &color,
                );
            }
        }
    }

    println!("{}", image);
}
