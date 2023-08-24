use cgmath::Vector3;

use super::Mesh;

/// This struct's made easy to generate meshes.
/// 
/// It takes away the worry of vertex and index buffers and allows the caller
/// to generate meshes by calling convenient methods, such as: add_triangles or add_quads.
/// 
/// MeshBuilder should not be used for "on the fly" contexts, meaning that all the MeshBuilder
/// instances should be built before the program starts running. That's because the build method
/// is not meant to be fast but rather safe and correct, preventing the caller from making wrong
/// geometry.
/// 
/// TOOD: For use cases where generating geometry on-the-fly is necessary, we should implement a new
/// struct solely for this purpose: DynamicMeshBuilder, or something similar.
pub struct MeshBuilder {
    triangles: Vec::<Triangle>,
    quads: Vec::<Quad>,
}

impl MeshBuilder {

    pub fn new() -> Self {

        Self {
            triangles: vec![],
            quads: vec![],
        }
    }

    pub fn add_triangles(&mut self, triangles: &[Triangle]) {
        self.triangles.extend(triangles);
    }

    pub fn add_quads(&mut self, quads: &[Quad]) {
        self.quads.extend(quads);
    }

    // TODO: While trying to make this method to generate an index buffer
    // I've faced a challange that at the present moment I am not sure how to correctly overcome.
    // The fact that you can't compare two floats with one another was causing me lots of frustration and time;
    // and thus I have decided to leave this matter for the future or for more clever minds.
    pub fn build(self) -> Mesh {

        let vertex_buffer = vec![
            self.triangles.into_iter()
                .flatten()
                .collect::<Vec<Vector3::<f32>>>(),
            self.quads.into_iter()
                .flat_map(|q| q.triangulate())
                .flatten()
                .collect::<Vec<Vector3::<f32>>>()
        ].concat();

        Mesh::new(vertex_buffer, None)
    }
}

#[derive(Clone, Copy)]
pub struct Triangle {
    a: Vector3::<f32>,
    b: Vector3::<f32>,
    c: Vector3::<f32>,
}

#[derive(Clone, Copy)]
pub struct Quad {
    a: Vector3::<f32>,
    b: Vector3::<f32>,
    c: Vector3::<f32>,
    d: Vector3::<f32>,
}

impl Quad {

    // TODO: This relies on the premise that the quad's vertices are gonna be
    // filled in correctly, that means in clock-wise order.
    // If we're trying to be pedantic here, checks should be when creating a new Quad.
    pub fn triangulate(self) -> [Triangle; 2] {

        let tri1 = Triangle {
            a: self.a,
            b: self.b,
            c: self.c,
        };

        let tri2 = Triangle {
            a: self.c,
            b: self.d,
            c: self.a,
        };

        return [tri1, tri2];
    }
}

pub struct TriangleIntoIter {
    triangle: Triangle,
    index: usize,
}

impl Iterator for TriangleIntoIter {
    type Item = Vector3::<f32>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => self.triangle.a,
            1 => self.triangle.b,
            2 => self.triangle.c,
            _ => return None,
        };

        self.index += 1;
        Some(result)
    }
}

impl IntoIterator for Triangle {
    type Item = Vector3::<f32>;
    type IntoIter = TriangleIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        TriangleIntoIter {
            triangle: self,
            index: 0,
        }
    }
}