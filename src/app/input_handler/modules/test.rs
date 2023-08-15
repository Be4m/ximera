use std::rc::Rc;
use std::cell::RefCell;

use winit::event::VirtualKeyCode;

use crate::render::Renderer;

use crate::app::InputHandlerModule;
use crate::app::input_handler::{InputType, Input};

pub struct TestInputHandlerModule {}

impl TestInputHandlerModule {

    pub fn new(renderer: &mut Renderer) -> Self {

        Self {}
    }

    pub fn module(instance: Self) -> Rc::<RefCell::<Self>> {
        Rc::new(RefCell::new(instance))
    }
}

impl InputHandlerModule for TestInputHandlerModule {

    fn process_input(&mut self, input: Input) {

        let keyb_input = match input {
            Input::KeyboardInput(val) => val,  
            _ => { panic!("wrong input type specified!"); }
        };

        let key = keyb_input.virtual_keycode.unwrap();

        match key {

            VirtualKeyCode::W => {
                println!("TestIHM: W");
            },

            VirtualKeyCode::A => {
                println!("TestIHM: A");
            },

            VirtualKeyCode::D => {
                println!("TestIHM: D");
            },

            VirtualKeyCode::S => {
                println!("TestIHM: S");
            },

            _ => {}
        }
    }
    
    fn accepted_input_types(&self) -> Vec::<InputType> {
        vec![InputType::KeyboardInput]
    }
}