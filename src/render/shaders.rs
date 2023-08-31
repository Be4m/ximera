pub struct Shaders {
    pub model: wgpu::ShaderModule,
}

impl Shaders {
    pub fn create(device: &wgpu::Device) -> Self {
        let model = device.create_shader_module(wgpu::include_wgsl!("../assets/shaders/model.wgsl"));

        Self {
            model,
        }
    }
}