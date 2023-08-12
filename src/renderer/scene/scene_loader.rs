use crate::renderer::Renderer;

use super::Scene;


pub struct SceneLoader {
    renderer: Renderer,
}

impl SceneLoader {

    pub fn new(renderer: Renderer) -> SceneLoader {

        Self {
            renderer,
        }
    }

    pub fn update(&self, scene: &dyn Scene) {

        todo!()
    }

    pub fn render(&self, scene: &dyn Scene) {

        todo!()
    }
}