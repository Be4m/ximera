pub struct Shaders {
    pub dummy: wgpu::ShaderModule,
    pub model: wgpu::ShaderModule,
}

impl Shaders {
    pub fn create(device: &wgpu::Device) -> Self {
        let model = device.create_shader_module(wgpu::include_wgsl!("../assets/shaders/model.wgsl"));
        let dummy = device.create_shader_module(wgpu::include_wgsl!("../assets/shaders/dummy.wgsl"));

        Self {
            dummy,
            model,
        }
    }
}