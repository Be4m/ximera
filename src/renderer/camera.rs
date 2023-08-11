
use super::Renderer;

use winit::event::VirtualKeyCode;

use crate::app::InputModule;

#[derive(Clone, Copy)]
pub struct CameraInputModule {}

impl CameraInputModule {

    pub fn new(renderer: &mut Renderer) -> CameraInputModule {

        Self {}
    }
}

impl InputModule for CameraInputModule {

    fn handle_input(&self, input: &winit::event::KeyboardInput) {

        // For now, in case the provided input variable doesn't contain a virtual_key value, this panics.
        let key = input.virtual_keycode.unwrap();

        match key {

            VirtualKeyCode::W => {
                println!("CameraIM: W");
            },

            VirtualKeyCode::A => {
                println!("CameraIM: A");
            },

            VirtualKeyCode::D => {
                println!("CameraIM: D");
            },

            VirtualKeyCode::S => {
                println!("CameraIM: S");
            },

            _ => {}
        }
    }
}