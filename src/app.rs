use winit::{
    window::Window,
    event::*,
    event_loop::ControlFlow,
};

use crate::renderer::{Scene, Renderer};

type EventLoop = winit::event_loop::EventLoop<()>;


/// The app struct encapsulates the whole program and provides a way for the
/// inner components of the program to share data with each other.
/// 
/// # Design
/// All functions in this struct except `run` should return the App struct.
pub struct App {
    window: Window,
    event_loop: EventLoop,

    input_modules: Vec<Box<dyn InputModule>>,

    active_scene: Scene,
}

impl App {

    pub fn new(window: Window, event_loop: EventLoop) -> App {

        Self {
            window,
            event_loop,
            
            input_modules: vec![],

            active_scene: Scene::default(),
        }
    }

    pub fn run(mut self) {

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

                self.active_scene.update();

                use crate::renderer::RenderUtil;
                self.active_scene.render().unwrap();
            },

            Event::MainEventsCleared => self.window.request_redraw(),

            _ => {}
        });
    }

    pub fn render(self, renderer: Renderer) -> App {
        self
    }

    pub fn add_input_module<T>(mut self, input_mod: T) -> App
    where
        T: InputModule + 'static
    {
        self.input_modules.push(Box::new(input_mod));
        return self;
    }

    #[allow(unused)] // TODO: This will be implemented as the program gets more complicated
    pub fn add_scene(mut self, scene: Scene) -> App {
        self
    }

    pub fn load_scene(mut self, scene: Scene) -> App {
        self
    }
}

pub trait InputModule {

    fn handle_input(&self, input: &KeyboardInput);
}