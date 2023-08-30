use winit::event::KeyboardInput;

use super::{
    InputHandlerModule,
    input_handler_module::InputTypes,
};

pub enum Input<'a> {
    KeyboardInput(&'a KeyboardInput),
    MouseMotion(&'a (f64, f64)),
    MouseWheelSpin,
}

pub struct InputHandler {
    modules: Vec<InputHandlerModule>,
}

impl InputHandler {

    pub fn new(modules: &[InputHandlerModule]) -> Self {

        let modules = modules.into_iter()
            .cloned()
            .collect();

        Self {
            modules,
        }
    }

    pub fn forward(&mut self, input: Input) {

        match input {
            Input::KeyboardInput(input) => {
                self.modules.iter_mut()
                    .filter(|m| m.accepted_input.contains(InputTypes::KEYBOARD_INPUT))
                    .for_each(|m| m.kind.handle_input(Input::KeyboardInput(input)));
            },
            Input::MouseMotion(delta) => {
                self.modules.iter_mut()
                    .filter(|m| m.accepted_input.contains(InputTypes::MOUSE_INPUT))
                    .for_each(|m| m.kind.handle_input(Input::MouseMotion(delta)));
            },
            _ => todo!()
        }
    }
}