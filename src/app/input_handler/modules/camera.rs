use std::rc::Rc;
use std::cell::RefCell;

use winit::event::VirtualKeyCode;

use crate::render::Renderer;

use crate::app::InputHandlerModule;
use crate::app::input_handler::{InputType, Input};

pub struct CameraInputHandlerModule {}

impl CameraInputHandlerModule {

    pub fn new(renderer: &mut Renderer) -> Self {

        Self {}
    }

    pub fn module(instance: Self) -> Rc::<RefCell::<Self>> {
        Rc::new(RefCell::new(instance))
    }
}

impl InputHandlerModule for CameraInputHandlerModule {

    fn process_input(&mut self, input: Input) {

        let keyb_input = match input {
            Input::KeyboardInput(val) => val,  
            _ => { panic!("wrong input type specified!"); }
        };

        let key = keyb_input.virtual_keycode.unwrap();

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
    
    fn accepted_input_types(&self) -> Vec::<InputType> {
        vec![InputType::KeyboardInput]
    }
}