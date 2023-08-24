use std::sync::Mutex;
use std::rc::Rc;
use std::cell::RefCell;

use winit::{
    event::*,event_loop::ControlFlow,
    window::Window,
};

use crate::render::Renderer;

use crate::scene::Scene;

use super::input_handler::{InputHandler, InputHandlerModule, Input};

type EventLoop = winit::event_loop::EventLoop<()>;

pub struct App {
    window: Window,
    event_loop: EventLoop,

    input_handler: Mutex::<InputHandler>,

    active_scene: Scene,
}

impl App {

    // TODO: renderer should be optional
    pub fn new(window: Window, event_loop: EventLoop, renderer: Renderer) -> Self {

        Self {
            window,
            event_loop,
            
            input_handler: Mutex::new(InputHandler::new()),

            active_scene: Scene::new(renderer),
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

                    // Close Window
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Q | VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => *control_flow = ControlFlow::Exit,

                    WindowEvent::KeyboardInput { input, .. } => {
                        input_handler.forward(Input::KeyboardInput(input));
                    },

                    _ => {}
                },

                Event::DeviceEvent {
                    ref event,
                    ..
                } => match event {

                    DeviceEvent::MouseMotion { delta } => {
                        input_handler.forward(Input::MouseMotion(delta));
                    },

                    DeviceEvent::MouseWheel { delta } => {
                        //input_handler.forward(Input::MouseWheelSpin(delta));
                    },

                    _ => {}
                },

                Event::RedrawRequested(window_id) if self.window.id() == window_id => {

                    // #1: self.scene_render_manager.render(&self.active_scene);
                    // #2: self.scene.render();
                }

                _ => {}
            }
        });
    }

    pub fn add_input_handler_module(
        self,
        module: Rc::<RefCell::<dyn InputHandlerModule + 'static>>,
    ) -> Self {

        {
            let mut input_handler = self.input_handler.lock().unwrap();

            input_handler.add_module(module);
        }

        return self;
    }
}