use super::pipelines::PipelineKind;

pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_vertices: u32,
    pub num_indices: u32,

    pub pipeline: PipelineKind,
    pub bind_groups: Vec<wgpu::BindGroup>,
}