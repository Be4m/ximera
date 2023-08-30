// I don't think we're going to need more than one vertex for this program as of now.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
}

impl Vertex {
    pub fn buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                }
            ],
        }
    }
}

pub struct Model {
    pub transform: nalgebra::Transform3<f32>,
    
    pub vertex_data: Vec<Vertex>,
    pub index_data: Vec<i16>,
    
    pub num_vertices: u32,
    pub num_indices: u32,
}

pub struct MeshData {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_vertices: u32,
    pub num_indices: u32,

    pub model_mat: [[f32; 4]; 4],
}

pub enum Mesh {
    Created(MeshData),
    Loaded(MeshData)
}