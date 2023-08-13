use winit::{
    window::Window,
    event::*,
    event_loop::ControlFlow,
};

use crate::scene::Scene;

use super::{
    AssetLoaderModule, InputModule,
    errors::AppInitError,
};

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

    input_modules: Vec::<Box<dyn InputModule>>,
    asset_loader_modules: Vec::<Box<dyn AssetLoaderModule<Item = dyn super::Asset>>>,

    active_scene: Scene,
}

impl App {

    pub fn new(window: Window, event_loop: EventLoop) -> App {

        Self {
            window,
            event_loop,
            
            input_modules: vec![],
            asset_loader_modules: vec![],

            active_scene: Scene::default(), // TODO
        }
    }

    // TODO: Add debugging messages for loading screen.
    fn init(&self) -> Result<(), AppInitError> {

        // self.geometry_compiler_modules
        //     .iter()
        //     .for_each(|gcm| gcm.compile_all());

        for alm in self.asset_loader_modules.iter() {
            alm.load_all()?;
        }

        Ok(())
    }

    pub fn run(self) {

        // -- Init --
        self.init().unwrap();

        self.event_loop.run(move |event, _, control_flow| match event {

            Event::WindowEvent {
                window_id,
                ref event,
            } if self.window.id() == window_id => match event {

                // --- Default events ---

                // Close the Window
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,

                // --- Handle custom input modules ---
                WindowEvent::KeyboardInput { input, .. } => {

                    self.input_modules
                        .iter()
                        .for_each(|m| m.handle_input(input));
                },

                _ => {}
            },

            Event::RedrawRequested(window_id) if self.window.id() == window_id => {

                // self.active_scene.update(); // 
                // self.renderer.queue_render(&mut queue)
            },

            Event::MainEventsCleared => self.window.request_redraw(),

            _ => {}
        });
    }

    pub fn add_input_module<T>(mut self, input_mod: T) -> App
    where
        T: InputModule + 'static
    {
        self.input_modules.push(Box::new(input_mod));
        return self;
    }

    pub fn add_asset_loader_module<T>(mut self, asset_loader_mod: T) -> App
    where
        T: AssetLoaderModule<Item = dyn super::Asset> + 'static
    {
        self.asset_loader_modules.push(Box::new(asset_loader_mod));
        return self;
    }
}