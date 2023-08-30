use crate::render::camera::{
    Camera, CameraController,
};

use super::Input;

bitflags::bitflags! {

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct InputTypes: u32 {
        const KEYBOARD_INPUT = 0 << 0;
        const MOUSE_INPUT = 0 << 1;
    }
}

#[derive(Clone)]
pub struct InputHandlerModule {
    pub accepted_input: InputTypes,
    pub kind: InputHandlerModuleKind,
}

#[derive(Clone)]
pub enum InputHandlerModuleKind {
    CameraControllerIHM(CameraController),
}

impl InputHandlerModuleKind {
    pub fn handle_input(&mut self, input: Input) {
        match self {
            InputHandlerModuleKind::CameraControllerIHM(camera_controller) => todo!(),
        }
    }
}