use crate::color::Color;
use crate::image::Image;
use crate::mat44::Mat44;
use crate::vec3::Vec3;
use crate::obj::ObjModel;
use rand::Rng;

mod color;
mod image;
mod mat44;
mod vec3;
mod obj;

fn main() {
    // Try to load an OBJ file, fall back to sphere if not found
    let mut model = if let Ok(obj_model) = ObjModel::load("models/deer.obj") {
        obj_model
    } else {
        generate_sphere(20, 20)
    };

    // Calculate normals if not present
    if model.normals.is_empty() {
        model.calculate_normals();
    }

    // Center and scale the model
    model.center_and_scale(10.0);

    let sun_dir = Vec3::new(0.1, 0.1, -1.0);

    let mut image = Image::new(800, 600);
    let mut rng = rand::thread_rng();
    let persp = Mat44::persp((45.0_f32).to_radians(), image.aspect(), 0.1, 100.0);

    for x in -1..=1 {
        for y in -1..=1 {
            let trans = Mat44::trans(&Vec3::new(x as f32 * 8.0, y as f32 * 8.0, -30.0));
            let angle = rng.gen_range(-360.0_f32..360.0_f32);
            let rotat = Mat44::rotat(&Vec3::new(1.0, 1.0, 1.0), angle.to_radians());

            let t_vertices: Vec<Vec3> = model.vertices.iter().map(|v| persp * trans * rotat * v).collect();

            for face in &model.faces {
                if let Some((v0, v1, v2)) = model.get_triangle_vertices(face) {
                    let e0 = v1 - v0;
                    let e1 = v2 - v0;
                    let n = e0.cross(&e1).norm();

                    let lum = n.dot(&sun_dir.neg().norm()).clamp(0.0, 1.0);
                    let color = Vec3::new(1.0, 1.0, 1.0) * (0.3 + 0.7 * lum);
                    let color = Color::from(color);

                    let t_v0 = &t_vertices[face.vertices[0]];
                    let t_v1 = &t_vertices[face.vertices[1]];
                    let t_v2 = &t_vertices[face.vertices[2]];

                    image.draw_triangle(
                        t_v0,
                        t_v1,
                        t_v2,
                        &Vec3::new(0.0, 0.0, 1.0),
                        &color,
                    );
                }
            }
        }
    }

    println!("{}", image);
}

fn generate_sphere(lat_segments: usize, lon_segments: usize) -> ObjModel {
    let mut model = ObjModel::new();
    
    // Generate vertices
    for i in 0..=lat_segments {
        let theta = std::f32::consts::PI * (i as f32) / (lat_segments as f32);
        let y = theta.cos();
        let r = theta.sin();
        for j in 0..=lon_segments {
            let phi = 2.0 * std::f32::consts::PI * (j as f32) / (lon_segments as f32);
            let x = r * phi.cos();
            let z = r * phi.sin();
            model.vertices.push(Vec3::new(x * 1.5, y * 1.5, z * 1.5));
        }
    }

    // Generate faces
    for i in 0..lat_segments {
        for j in 0..lon_segments {
            let a = i * (lon_segments + 1) + j;
            let b = a + lon_segments + 1;
            let c = a + 1;
            let d = b + 1;
            // Each quad is split into two triangles
            model.faces.push(crate::obj::Face {
                vertices: vec![a, b, c],
            });
            model.faces.push(crate::obj::Face {
                vertices: vec![c, b, d],
            });
        }
    }

    model
}
