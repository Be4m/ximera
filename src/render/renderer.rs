use super::{mesh::Mesh, pipelines::MeshDebugPipeline};

pub struct Renderer {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface,
    pub surface_format: wgpu::TextureFormat,

    //render_pipeline: wgpu::RenderPipeline,

    //camera: Camera,
}

impl Renderer {

    pub fn new(
        window: &winit::window::Window,
        runtime: &tokio::runtime::Runtime,
    ) -> Renderer {

        let instance = wgpu::Instance::new(
            wgpu::InstanceDescriptor {
                backends: wgpu::Backends::VULKAN,
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

        // TODO: Temporary
        // let shader_mod = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        //     label: None,
        //     source: wgpu::ShaderSource::Wgsl(include_str!("default.wgsl").into()),
        // });

        // let pipelines = {
        //     let mesh_debug = MeshDebugPipeline::new(
        //         &device,
        //         &shader_mod,
        //         &shader_mod,
        //         surface_format,
        //     );
        // };

        Self {
            device,
            queue,
            surface,
            surface_format,
        }
    }

    pub fn render<T: Renderable>(
        &self,
        renderable: &T,
        pipeline: super::Pipeline,
    ) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        {
            // TODO: color_attachments should be dictated by the active scene.
            // IDEA: Maybe Renderable should be substituted for something like SceneObject
            // which would contain a reference to the relevant scene that holds variables like bg_colour and so on.
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    }
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(pipeline.pipeline());
            render_pass.set_vertex_buffer(0, renderable.vertex_buffer().slice(..));

            render_pass.draw(0..renderable.num_vertices(), 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub trait Renderable {
    fn mesh(&self) -> &Mesh;

    fn vertex_buffer(&self) -> &wgpu::Buffer {
        &self.mesh().vertex_buffer
    }

    fn num_vertices(&self) -> u32 {
        self.mesh().num_vertices
    }
}