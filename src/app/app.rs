use std::sync::Mutex;

use winit::{
    window::Window,
    event::*,
};

use crate::scene::Scene;

use super::input_handler::{InputHandler, InputHandlerModule, InputType};

type EventLoop = winit::event_loop::EventLoop<()>;

pub struct App {
    window: Window,
    event_loop: EventLoop,

    input_handler: Mutex::<InputHandler>,

    active_scene: Scene,
}

impl App {

    pub fn new(window: Window, event_loop: EventLoop) -> Self {

        Self {
            window,
            event_loop,
            
            input_handler: Mutex::new(InputHandler::new()),

            active_scene: Scene::default(),
        }
    }

    pub fn run(self) {

        self.event_loop.run(move |event, _, control_flow| {

            let mut input_handler = self.input_handler.lock().unwrap();

            match event {

                Event::WindowEvent {
                    window_id,
                    ref event,
                } if self.window.id() == window_id => match event {

                    // Input Handling
                    WindowEvent::KeyboardInput { input, .. } => {
                        input_handler.forward(InputType::KeyboardInput(input));
                    },

                    _ => {}
                },

                Event::DeviceEvent {
                    ref event,
                    ..
                } => match event {

                    DeviceEvent::MouseMotion { delta } => {
                        input_handler.forward(InputType::MouseMotion(delta));
                    },

                    DeviceEvent::MouseWheel { delta } => {
                        // input_handler.forward_mouse_wheel()
                    },

                    _ => {}
                },

                _ => {}
            }
        });
    }

    pub fn add_input_handler_module(&mut self, module: Box::<dyn InputHandlerModule + 'static>) {

        let mut input_handler = self.input_handler.lock().unwrap();

        input_handler.add_module(module);
    }
}