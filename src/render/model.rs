use nalgebra::Transform3;
use wgpu::util::DeviceExt;

use crate::render::pipelines::BindGroupKind;

use super::{
    pipelines::{PipelineKind, BindGroupLayouts},
    mesh::Mesh,
};

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

impl From<nalgebra::Point3<f32>> for Vertex {
    fn from(value: nalgebra::Point3<f32>) -> Self {
        Self {
            position: [
                value.x,
                value.y,
                value.z,
            ],
        }   
    }
}

#[derive(Clone)]
pub struct Model {
    pub transform: Transform3<f32>,
    
    pub vertex_data: Vec<Vertex>,
    pub index_data: Vec<i16>,
    
    pub num_vertices: u32,
    pub num_indices: u32,

    pub pipeline_kind: PipelineKind,
}

impl Model {
    pub fn create_mesh(
        &self,
        device: &wgpu::Device,
        bind_group_layouts: &BindGroupLayouts,
    ) -> Mesh {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(self.vertex_data.as_slice()),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(self.index_data.as_slice()),
            usage: wgpu::BufferUsages::INDEX,
        });

        let model_mat: [[f32; 4]; 4] = bytemuck::cast(self.transform);
        let model_mat_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&model_mat),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_groups = self.pipeline_kind.used_bind_groups()
            .into_iter()
            .map(|kind| {
                match kind {
                    BindGroupKind::Model => {
                        device.create_bind_group(&wgpu::BindGroupDescriptor {
                            label: None,
                            layout: &bind_group_layouts.model,
                            entries: &[
                                wgpu::BindGroupEntry {
                                    binding: 0,
                                    resource: model_mat_buffer.as_entire_binding(),
                                }
                            ]
                        })
                    }
                }
            })
            .collect::<Vec<_>>();

        Mesh {
            vertex_buffer,
            index_buffer,
            num_vertices: self.num_vertices.clone(),
            num_indices: self.num_indices.clone(),

            pipeline: self.pipeline_kind,
            bind_groups,
        }
    }
}