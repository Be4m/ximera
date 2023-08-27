use cgmath::Point3;

use super::{Mesh, Vertex};

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
    origin: Point3<f32>,

    pinned_vertices: Vec<Vertex>,
    triangles: Vec<Triangle>,
    quads: Vec<Quad>,
}

impl MeshBuilder {

    pub fn new(origin: Point3<f32>) -> Self {

        Self {
            origin,

            pinned_vertices: vec![],
            triangles: vec![],
            quads: vec![],
        }
    }

    pub fn pin_vertex(&mut self, vertex: Vertex) {
        // TODO: Refactoring
        let vertex = Vertex {
            position: [
                self.origin.x + vertex.position[0],
                self.origin.y + vertex.position[1],
                self.origin.z + vertex.position[2],
            ]
        };

        self.pinned_vertices.push(vertex);
    }

    pub fn pin_vertices(&mut self, vertices: &[Vertex]) {
        self.pinned_vertices.extend(vertices);
    }

    pub fn add_triangles(&mut self, triangles: &[Triangle]) {
        self.triangles.extend(triangles);
    }

    pub fn add_triangle(&mut self, triangle: Triangle) {
        self.triangles.push(triangle);
    }

    pub fn add_quad(&mut self, quad: Quad) {
        self.quads.push(quad);
    }

    pub fn add_quads(&mut self, quads: &[Quad]) {
        self.quads.extend(quads);
    }

    pub fn get_pinned_vertex(
        &self,
        offset: Option<usize>,
        index: usize,
    ) -> Option<Vertex> {
        self.pinned_vertices.get(match offset {
            Some(v) => index + v,
            None => index,
        }).copied()
    }

    // TODO: While trying to make this method to generate an index buffer
    // I've faced a challange that at the present moment I am not sure how to correctly overcome.
    // The fact that you can't compare two floats with one another was causing me lots of frustration and time;
    // and thus I have decided to leave this matter for the future or for more clever minds.
    pub fn build(
        self,
        device: &wgpu::Device,
    ) -> Mesh {

        let vertex_data = vec![
            self.triangles.into_iter()
                .flatten()
                .collect::<Vec<Vertex>>(),
            self.quads.into_iter()
                .flat_map(|q| q.triangulate())
                .flatten()
                .collect::<Vec<Vertex>>()
        ].concat();

        let num_vertices = vertex_data.len() as u32;

        use wgpu::util::DeviceExt;
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(vertex_data.as_slice()),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Mesh::new(vertex_buffer, num_vertices, None)
    }

    pub fn gen_uv_sphere(
        radius: f32,
        num_slices: u32,
        num_stacks: u32,
        origin: Point3<f32>,
    ) -> MeshBuilder {
        let mut mesh_builder = MeshBuilder::new(origin);

        // Top & bottom vertices
        let vtop = Vertex { position: [0.0, radius, 0.0] };
        let vbottom = Vertex { position: [0.0, -radius, 0.0] };

        // Add top vertex
        mesh_builder.pin_vertex(vtop);

        for stack in 0..(num_stacks - 1) {
            let phi = std::f32::consts::PI / (num_stacks as f32) * (stack as f32 + 1.0);

            for slice in 0..num_slices {
                let theta = std::f32::consts::TAU / (num_slices as f32) * (slice as f32);

                let x = radius * theta.sin() * phi.cos();
                let y = radius * theta.sin() * phi.sin();
                let z = radius * theta.cos();

                mesh_builder.pin_vertex(Vertex { position: [x, y, z] });
            }
        }

        // Add bottom vertex
        mesh_builder.pin_vertex(vbottom);

        // Add top/bottom triangles
        for slice in 0..num_slices {
            let i0 = slice as usize;
            let i1 = ((slice + 1) % num_slices) as usize;

            // Offset by 1 for the top vertex.
            let v0 = mesh_builder.get_pinned_vertex(Some(1), i0).unwrap();
            let v1 = mesh_builder.get_pinned_vertex(Some(1), i1).unwrap();

            mesh_builder.add_triangle(Triangle {
                a: v0,
                b: v1,
                c: vtop,
            });

            let offset = ((num_stacks - 2) * num_slices + 1) as usize;
            let v0 = mesh_builder.get_pinned_vertex(Some(offset), i0).unwrap();
            let v1 = mesh_builder.get_pinned_vertex(Some(offset), i1).unwrap();

            mesh_builder.add_triangle(Triangle {
                a: v0,
                b: v1,
                c: vbottom,
            });
        }

        // Add quads
        for stack in 0..(num_stacks - 2) {
            for slice in 0..num_slices {
                let i0 = slice as usize;
                let i1 = (slice + 1) as usize;

                let top_offset = (stack * num_slices + 1) as usize;
                let v0 = mesh_builder.get_pinned_vertex(Some(top_offset), i0).unwrap();
                let v1 = mesh_builder.get_pinned_vertex(Some(top_offset), i1).unwrap();

                let bottom_offset = top_offset + num_slices as usize;
                let v2 = mesh_builder.get_pinned_vertex(Some(bottom_offset), i1).unwrap();
                let v3 = mesh_builder.get_pinned_vertex(Some(bottom_offset), i0).unwrap();

                mesh_builder.add_quad(Quad {
                    a: v0,
                    b: v1,
                    c: v2,
                    d: v3,
                });
            }
        }

        return mesh_builder;
    }
}

#[derive(Clone, Copy)]
pub struct Triangle {
    a: Vertex,
    b: Vertex,
    c: Vertex,
}

#[derive(Clone, Copy)]
pub struct Quad {
    a: Vertex,
    b: Vertex,
    c: Vertex,
    d: Vertex,
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
    type Item = Vertex;

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
    type Item = Vertex;
    type IntoIter = TriangleIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        TriangleIntoIter {
            triangle: self,
            index: 0,
        }
    }
}