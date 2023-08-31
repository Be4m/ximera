use crate::render::{Vertex, pipelines::BindGroupKind};
use super::{
    BindGroupLayouts, BindGroupLayoutMap,
    bind_group_map,
};

pub struct MeshDebugPipeline {
    pub render_pipeline: wgpu::RenderPipeline,
}

impl MeshDebugPipeline {
    pub const BIND_GROUP_MAP: BindGroupLayoutMap = bind_group_map!(
        BindGroupLayouts::NUM_LAYOUTS,
    );

    pub fn new(
        device: &wgpu::Device,
        vs_module: &wgpu::ShaderModule,
        fs_module: &wgpu::ShaderModule,
        target_format: wgpu::TextureFormat,
    ) -> Self {

        // Make sure that "Wireframe" rendering is enabled for debug pipeline
        assert!(device.features().contains(wgpu::Features::POLYGON_MODE_LINE));

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Mesh Debug pipeline layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Mesh Debug pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: vs_module,
                entry_point: "vertex_main",
                buffers: &[Vertex::buffer_layout()],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Line,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: fs_module,
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