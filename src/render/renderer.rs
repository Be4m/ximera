use super::{
    Shaders,
    pipelines::{Pipelines, MeshDebugPipeline, BindGroupLayouts},
    Mesh,
};


pub struct Renderer {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,

    pub surface: wgpu::Surface,
    pub surface_format: wgpu::TextureFormat,
    pub resolution: winit::dpi::PhysicalSize<u32>,

    pub pipelines: Pipelines,
    pub bind_group_layouts: BindGroupLayouts,
}

impl Renderer {

    pub fn new(
        window: &winit::window::Window,
        runtime: &tokio::runtime::Runtime,
    ) -> Renderer {

        let instance = wgpu::Instance::new(
            wgpu::InstanceDescriptor {
                backends: wgpu::Backends::all(),
                ..Default::default()
            }
        );
    
        let surface = unsafe { instance.create_surface(window) }.unwrap();
    
        let adapter = runtime.block_on(instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }
        ))
        .unwrap();
    
        let (device, queue) = runtime.block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::POLYGON_MODE_LINE,
                limits: Default::default(),
            },
            None,
        )).unwrap();

        let size = window.inner_size();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &surface_config);

        let shaders = Shaders::create(&device);
        let bind_group_layouts = BindGroupLayouts::create(&device);

        let pipelines = Pipelines {
            mesh_debug: MeshDebugPipeline::new(
                &device,
                &shaders.dummy,
                &shaders.dummy,
                surface_format,
            ),
        };
        
        Self {
            device,
            queue,
            
            surface,
            surface_format,
            resolution: size,

            pipelines,
            bind_group_layouts,
        }
    }

    pub fn render_mesh(&self, mesh: &Mesh) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let pipeline = mesh.pipeline.get_pipeline(&self.pipelines);

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 1.0,
                                    b: 0.0,
                                    g: 0.0,
                                    a: 1.0,
                                }
                            ),
                            store: true,
                        }
                    })
                ],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(pipeline.render_pipeline());
            mesh.bind_groups
                .iter()
                .enumerate()
                .for_each(|(i, bgroup)| {
                    render_pass.set_bind_group(i.try_into().unwrap(), bgroup, &[]);
                });

            render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..mesh.num_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}