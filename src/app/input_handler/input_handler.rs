use std::rc::{Rc, Weak};
use std::cell::RefCell;

use winit::event::KeyboardInput;

use super::InputHandlerModule;

pub enum Input<'a> {
    KeyboardInput(&'a KeyboardInput),
    MouseMotion(&'a (f64, f64)),
    MouseWheelSpin,
}

pub enum InputType {
    KeyboardInput,
    MouseMotion,
    MouseWheelSpin,
}

pub struct InputHandler {
    modules: Vec::<Rc::<RefCell::<dyn InputHandlerModule>>>,

    keyboard_handlers: Vec::<Weak::<RefCell::<dyn InputHandlerModule>>>,
    mouse_handlers: Vec::<Weak::<RefCell::<dyn InputHandlerModule>>>,
}

impl InputHandler {

    pub fn new() -> Self {

        Self {
            modules: vec![],

            keyboard_handlers: vec![],
            mouse_handlers: vec![],
        }
    }

    pub fn forward(&mut self, input_type: Input) {

        match input_type {

            Input::KeyboardInput(input) => {
                for handler in self.keyboard_handlers.iter_mut() {

                    handler.upgrade()
                        .unwrap()
                        .borrow_mut()
                        .process_input(Input::KeyboardInput(input));
                }
            },

            Input::MouseMotion(delta) => {
                for handler in self.mouse_handlers.iter_mut() {

                    handler.upgrade()
                        .unwrap()
                        .borrow_mut()
                        .process_input(Input::MouseMotion(delta));
                }
            },

            Input::MouseWheelSpin => todo!(),
        }
    }

    pub fn add_module(&mut self, module: Rc::<RefCell::<dyn InputHandlerModule>>) {

        for t in module.borrow().accepted_input_types().iter() {
            match t {
                InputType::KeyboardInput => self.keyboard_handlers.push(Rc::downgrade(&module)),
                InputType::MouseMotion => self.mouse_handlers.push(Rc::downgrade(&module)),
                InputType::MouseWheelSpin => todo!(),
            }
        }

        self.modules.push(module);
    }
}