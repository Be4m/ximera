use crate::renderer::colours;
use crate::renderer::colours::Colour;

use crate::atom::AtomObject;

use super::SceneObjectManager;

pub struct Scene {
    bg_colour: Colour,

    atom_manager: SceneObjectManager::<AtomObject>,
}

impl Scene {

    pub fn new() -> Scene {

        Self {
            bg_colour: colours::WHITE,

            atom_manager: SceneObjectManager::new(),
        }
    }

    pub fn update(&self) {
    }

    pub fn set_bg_colour(&mut self, colour: Colour) {
        self.bg_colour = colour;
    }
}

impl Default for Scene {

    fn default() -> Self {
        
        Self {
            bg_colour: colours::WHITE,

            atom_manager: SceneObjectManager::new(),
        }
    }
}