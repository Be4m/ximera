use winit::{
    window::Window,
    event::*,
    event_loop::ControlFlow,
};

use crate::renderer::Renderer;
use crate::renderer::scene::{Scene, ExampleScene, SceneLoader};

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

    // TODO: The option part is going to be removed after we set the app to have
    // a default renderer and thus a SceneLoader.
    scene_loader: Option<SceneLoader>,
    active_scene: Box<dyn Scene>,
}

impl App {

    pub fn new(window: Window, event_loop: EventLoop) -> App {

        Self {
            window,
            event_loop,
            
            input_modules: vec![],

            scene_loader: None,
            active_scene: Box::new(ExampleScene::default()),
        }
    }

    pub fn run(self) {

        // -- CHECKS --
        // TODO: There should be a default scene loader.
        let _scene_loader = self.scene_loader.expect("The scene loader is not initialised!
            Please initialise the scene loader by calling init_scene_loader on the App object.");


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

                // scene_loader.update(&self.active_scene);
                // scene_loader.render(&self.active_scene);
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

    // TODO: As the app expands we might want to have separate renderers for each scene,
    // that's why this design should in future be replaced with .load_scene(scene, renderer).
    pub fn init_scene_loader(mut self, renderer: Renderer) -> App {

        self.scene_loader = Some(SceneLoader::new(renderer));

        return self;
    }

    // TODO: This will be implemented as the program gets more complicated
    pub fn add_scene<T: Scene>(mut self, scene: T) -> App {
        self
    }

    pub fn load_scene<T: Scene>(mut self, scene: T) -> App {
        self
    }
}

pub trait InputModule {

    fn handle_input(&self, input: &KeyboardInput);
}