use crate::vec3::Vec3;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Vec3,
    pub tex_coord: Option<Vec3>, // Using Vec3 for simplicity, z is usually 0
    pub normal: Option<Vec3>,
}

#[derive(Debug, Clone)]
pub struct Face {
    pub vertices: Vec<usize>, // Indices into the vertex list
}

#[derive(Debug, Clone)]
pub struct ObjModel {
    pub vertices: Vec<Vec3>,
    pub tex_coords: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub faces: Vec<Face>,
}

impl ObjModel {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            tex_coords: Vec::new(),
            normals: Vec::new(),
            faces: Vec::new(),
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut model = ObjModel::new();

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "v" => {
                    // Vertex position: v x y z [w]
                    if parts.len() >= 4 {
                        let x: f32 = parts[1].parse().unwrap_or(0.0);
                        let y: f32 = parts[2].parse().unwrap_or(0.0);
                        let z: f32 = parts[3].parse().unwrap_or(0.0);
                        model.vertices.push(Vec3::new(x, y, z));
                    }
                }
                "vt" => {
                    // Texture coordinate: vt u v [w]
                    if parts.len() >= 3 {
                        let u: f32 = parts[1].parse().unwrap_or(0.0);
                        let v: f32 = parts[2].parse().unwrap_or(0.0);
                        let w: f32 = if parts.len() >= 4 {
                            parts[3].parse().unwrap_or(0.0)
                        } else {
                            0.0
                        };
                        model.tex_coords.push(Vec3::new(u, v, w));
                    }
                }
                "vn" => {
                    // Normal: vn x y z
                    if parts.len() >= 4 {
                        let x: f32 = parts[1].parse().unwrap_or(0.0);
                        let y: f32 = parts[2].parse().unwrap_or(0.0);
                        let z: f32 = parts[3].parse().unwrap_or(0.0);
                        model.normals.push(Vec3::new(x, y, z));
                    }
                }
                "f" => {
                    // Face: f v1/vt1/vn1 v2/vt2/vn2 v3/vt3/vn3 ...
                    if parts.len() >= 4 {
                        let mut face_vertices = Vec::new();
                        
                        for i in 1..parts.len() {
                            let vertex_data: Vec<&str> = parts[i].split('/').collect();
                            
                            // Parse vertex index (required)
                            let vertex_idx: usize = if !vertex_data.is_empty() {
                                vertex_data[0].parse().unwrap_or(1)
                            } else {
                                1
                            };
                            
                            // OBJ indices are 1-based, convert to 0-based
                            let vertex_idx = if vertex_idx > 0 {
                                vertex_idx - 1
                            } else {
                                model.vertices.len() + vertex_idx
                            };
                            
                            face_vertices.push(vertex_idx);
                        }
                        
                        // Triangulate the face if it has more than 3 vertices
                        if face_vertices.len() == 3 {
                            model.faces.push(Face {
                                vertices: face_vertices,
                            });
                        } else if face_vertices.len() > 3 {
                            // Simple triangulation: fan triangulation
                            for i in 1..face_vertices.len() - 1 {
                                model.faces.push(Face {
                                    vertices: vec![
                                        face_vertices[0],
                                        face_vertices[i],
                                        face_vertices[i + 1],
                                    ],
                                });
                            }
                        }
                    }
                }
                _ => {
                    // Skip other commands (mtl, g, s, etc.)
                }
            }
        }

        Ok(model)
    }

    pub fn get_triangle_vertices(&self, face: &Face) -> Option<(Vec3, Vec3, Vec3)> {
        if face.vertices.len() != 3 {
            return None;
        }

        let v0 = self.vertices.get(face.vertices[0])?;
        let v1 = self.vertices.get(face.vertices[1])?;
        let v2 = self.vertices.get(face.vertices[2])?;

        Some((*v0, *v1, *v2))
    }

    pub fn get_triangle_normals(&self, face: &Face) -> Option<(Vec3, Vec3, Vec3)> {
        if face.vertices.len() != 3 || self.normals.is_empty() {
            return None;
        }

        let default_normal = Vec3::new(0.0, 0.0, 1.0);
        let v0 = self.normals.get(face.vertices[0]).unwrap_or(&default_normal);
        let v1 = self.normals.get(face.vertices[1]).unwrap_or(&default_normal);
        let v2 = self.normals.get(face.vertices[2]).unwrap_or(&default_normal);

        Some((*v0, *v1, *v2))
    }

    pub fn calculate_normals(&mut self) {
        // Calculate normals for each face and accumulate them per vertex
        let mut vertex_normals: Vec<Vec3> = vec![Vec3::new(0.0, 0.0, 0.0); self.vertices.len()];
        let mut vertex_counts: Vec<usize> = vec![0; self.vertices.len()];

        for face in &self.faces {
            if let Some((v0, v1, v2)) = self.get_triangle_vertices(face) {
                let e0 = v1 - v0;
                let e1 = v2 - v0;
                let normal = e0.cross(&e1).norm();

                for &vertex_idx in &face.vertices {
                    vertex_normals[vertex_idx] = vertex_normals[vertex_idx] + normal;
                    vertex_counts[vertex_idx] += 1;
                }
            }
        }

        // Average the normals for each vertex
        self.normals.clear();
        for i in 0..self.vertices.len() {
            if vertex_counts[i] > 0 {
                self.normals.push(vertex_normals[i] / vertex_counts[i] as f32);
            } else {
                self.normals.push(Vec3::new(0.0, 0.0, 1.0));
            }
        }
    }

    pub fn get_bounding_box(&self) -> (Vec3, Vec3) {
        if self.vertices.is_empty() {
            return (Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        }

        let mut min = self.vertices[0];
        let mut max = self.vertices[0];

        for vertex in &self.vertices {
            min.x = min.x.min(vertex.x);
            min.y = min.y.min(vertex.y);
            min.z = min.z.min(vertex.z);
            
            max.x = max.x.max(vertex.x);
            max.y = max.y.max(vertex.y);
            max.z = max.z.max(vertex.z);
        }

        (min, max)
    }

    pub fn center_and_scale(&mut self, scale: f32) {
        let (min, max) = self.get_bounding_box();
        let center = (min + max) / 2.0;
        let size = (max - min).len();
        let scale_factor = scale / size;

        for vertex in &mut self.vertices {
            *vertex = (*vertex - center) * scale_factor;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_vertex() {
        let mut model = ObjModel::new();
        let line = "v 1.0 2.0 3.0";
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts[0] == "v" && parts.len() >= 4 {
            let x: f32 = parts[1].parse().unwrap();
            let y: f32 = parts[2].parse().unwrap();
            let z: f32 = parts[3].parse().unwrap();
            model.vertices.push(Vec3::new(x, y, z));
        }
        
        assert_eq!(model.vertices.len(), 1);
        assert_eq!(model.vertices[0], Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_parse_face() {
        let mut model = ObjModel::new();
        // Add some vertices first
        model.vertices.push(Vec3::new(0.0, 0.0, 0.0));
        model.vertices.push(Vec3::new(1.0, 0.0, 0.0));
        model.vertices.push(Vec3::new(0.0, 1.0, 0.0));
        
        let line = "f 1 2 3";
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts[0] == "f" && parts.len() >= 4 {
            let mut face_vertices = Vec::new();
            for i in 1..parts.len() {
                let vertex_idx: usize = parts[i].parse().unwrap();
                face_vertices.push(vertex_idx - 1); // Convert to 0-based
            }
            model.faces.push(Face { vertices: face_vertices });
        }
        
        assert_eq!(model.faces.len(), 1);
        assert_eq!(model.faces[0].vertices, vec![0, 1, 2]);
    }
} 