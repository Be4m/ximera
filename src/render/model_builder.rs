use nalgebra::Point3;

use super::mesh::Model;

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
pub struct ModelBuilder {
    points: Vec<Point3<f32>>,

    triangles: Vec<IndexTriangle>,
    quads: Vec<IndexQuad>,
}

impl ModelBuilder {

    pub fn new() -> Self {

        Self {
            points: vec![],

            triangles: vec![],
            quads: vec![],
        }
    }

    pub fn add_point(&mut self, position: Point3<f32>) {
        self.points.push(position);
    }

    pub fn index_triangle(&mut self, triangle: IndexTriangle) {
        self.triangles.push(triangle);
    }

    pub fn index_quad(&mut self, quad: IndexQuad) {
        self.quads.push(quad);
    }

    pub fn build(self) -> Model {
        let index_data = vec![
            self.triangles.into_iter()
                .flatten()
                .collect::<Vec<_>>(),
            self.quads.into_iter()
                .flat_map(|qd| qd.triangulate())
                .flatten()
                .collect::<Vec<_>>()
        ].concat();

        todo!();
    }

    pub fn build_uv_sphere(
        radius: f32,
        num_slices: u32,
        num_stacks: u32,
    ) -> Self { todo!() }
}

#[derive(Clone, Copy)]
pub struct IndexTriangle {
    a: i16,
    b: i16,
    c: i16,
}

#[derive(Clone, Copy)]
pub struct IndexQuad {
    a: i16,
    b: i16,
    c: i16,
    d: i16,
}

impl IndexQuad {

    // TODO: This relies on the premise that the quad's vertices are gonna be
    // filled in correctly, that means in clock-wise order.
    // If we're trying to be pedantic here, checks should be when creating a new Quad.
    pub fn triangulate(self) -> [IndexTriangle; 2] {

        let tri1 = IndexTriangle {
            a: self.a,
            b: self.b,
            c: self.c,
        };

        let tri2 = IndexTriangle {
            a: self.c,
            b: self.d,
            c: self.a,
        };

        return [tri1, tri2];
    }
}

pub struct IndexTriangleIntoIter {
    triangle: IndexTriangle,
    index: usize,
}

impl Iterator for IndexTriangleIntoIter {
    type Item = i16;

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

impl IntoIterator for IndexTriangle {
    type Item = i16;
    type IntoIter = IndexTriangleIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IndexTriangleIntoIter {
            triangle: self,
            index: 0,
        }
    }
}