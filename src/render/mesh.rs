/// The struct Mesh is all what Renderer needs to render an 'object' or mesh.
/// The Mesh struct, not like Geometry struct, is low-level representation of a geometry,
/// containing only a vertex and index buffers.
/// 
/// The Geometry struct is based on triangles and has convinient functions, like: add_quad.
/// Before rendering, geometry has to be compiled down to a Mesh struct. To do so you can use
/// a method of Geometry's called `gen_mesh`.
pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub num_vertices: u32,
    pub index_buffer: Option<wgpu::Buffer>,
}

impl Mesh {

    pub fn new(
        vertex_buffer: wgpu::Buffer,
        num_vertices: u32,
        index_buffer: Option<wgpu::Buffer>,
    ) -> Mesh {

        Self {
            vertex_buffer,
            num_vertices,
            index_buffer,
        }
    }
}

// TODO: This should be renamed to MeshVertex or PositionVertex, etc.
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
}