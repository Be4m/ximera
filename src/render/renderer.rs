use super::mesh::Mesh;

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,

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

        Self {
            device,
            queue,
            surface,
        }
    }
}


// DEV: Although I am now on a quest to eliminate as many trait objects from this program
// as possible, I think it's worth leaving a note here that a Renderable trait does make sence to me,
// especially now that Object is an enum.
pub trait Renderable {

    fn mesh(&self) -> &Mesh;

    fn position(&self) -> &crate::scene::Position;
}