use crate::render::Vertex;

use super::{
    BindGroupLayouts, BindGroupLayoutMap,
    BindGroupKind,
    bind_group_map,
};

pub struct ModelPipeline {
    pub render_pipeline: wgpu::RenderPipeline,
}

impl ModelPipeline {
    pub const BIND_GROUP_MAP: BindGroupLayoutMap = bind_group_map!(
        BindGroupLayouts::NUM_LAYOUTS,
        BindGroupKind::Model
    );

    pub fn new(
        device: &wgpu::Device,
        shader_module: &wgpu::ShaderModule,
        target_format: wgpu::TextureFormat,
        bind_group_layouts: &BindGroupLayouts,
    ) -> Self {
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Mesh Debug pipeline layout"),
            bind_group_layouts: &[
                &bind_group_layouts.model,
            ],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Mesh Debug pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader_module,
                entry_point: "vertex_main",
                buffers: &[Vertex::buffer_layout()],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: shader_module,
                entry_point: "fragment_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        Self {
            render_pipeline,
        }
    }
}