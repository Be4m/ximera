use super::{
    colours,
    renderer::RenderUtil,
};


pub struct Scene {
    bg_colour: (f32, f32, f32),
}

impl Scene {

    pub fn new() -> Scene {

        Self {
            bg_colour: colours::WHITE,
        }
    }

    pub fn update(&mut self) {}

    pub fn set_bg_colour(&mut self, colour: (f32, f32, f32)) {
        self.bg_colour = colour;
    }
}

impl Default for Scene {

    fn default() -> Self {
        
        Self {
            bg_colour: colours::WHITE,
        }
    }
}

impl RenderUtil for Scene {

    fn render(&self) -> Result<(), wgpu::SurfaceError> {

        Ok(())
    }
}