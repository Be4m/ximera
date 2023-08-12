pub mod colours {

    use crate::renderer::utils::types::F32x3;

    pub static WHITE: F32x3 = (1.0, 1.0, 1.0);
    pub static GREY:  F32x3 = (0.7, 0.7, 0.7);
    pub static BLACK: F32x3 = (0.0, 0.0, 0.0);
}

/// Keeps track of data such as camera and object position.
/// 
/// Encapsulates all data required for rendering, for example: gpu info.
/// As well as, as mentioned above, probably will be responsible for recording object position in
/// visual space, or now I'm thinking maybe we'll do that in a world struct or something like that.
/// 
/// So distilling all of it down to a brief summary, this struct is responsible for providing and storing data
/// required for rendering, as well as having helper functions for rendering objects.
pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
}

impl Renderer {

    //! Creates a renderer object with default parameters.
    //! 
    //! # Unfinished
    //! - [ ] Error handling
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

        let pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Default pipeline layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            }
        );

        todo!()
    }
}

pub trait RenderUtil {

    fn render(&self) -> Result<(), wgpu::SurfaceError>;
}