use std::sync::Mutex;

use winit::{
    window::Window,
    event::*,
};

use crate::scene::Scene;

use super::input_handler::InputHandler;

type EventLoop = winit::event_loop::EventLoop<()>;


/// The app struct encapsulates the whole program and provides a way for the
/// inner components of the program to share data with each other.
/// 
/// # Design
/// All functions in this struct except `run` should return the App struct.
/// 
/// TODO: Fix lifetime parameters and hopefuly that horrendous asset_loader_modules declaration.
pub struct App {
    window: Window,
    event_loop: EventLoop,

    input_handler: Mutex<InputHandler>,

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
                        input_handler.forward_keyboard_input(input);
                    },

                    _ => {}
                },

                Event::DeviceEvent {
                    ref event,
                    ..
                } => match event {

                    DeviceEvent::MouseMotion { delta } => {
                        input_handler.forward_mouse_motion(delta);
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

    // pub fn add_input_module<T>(mut self, input_mod: T) -> App
    // where
    //     T: InputModule + 'static
    // {
    //     self.input_modules.push(Box::new(input_mod));
    //     return self;
    // }

    // pub fn add_asset_loader_module<T>(mut self, asset_loader_mod: T) -> App
    // where
    //     T: AssetLoaderModule<Item = dyn super::Asset> + 'static
    // {
    //     self.asset_loader_modules.push(Box::new(asset_loader_mod));
    //     return self;
    // }
}