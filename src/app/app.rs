use std::sync::Mutex;
use std::rc::Rc;
use std::cell::RefCell;

use cgmath::Point3;

use winit::{
    event::*,
    event_loop::ControlFlow,
    window::Window,
};

use crate::{
    render::{Renderer, Pipeline, pipelines::MeshDebugPipeline, colours::Colour},
    scene::Scene,
};

use super::input_handler::{InputHandler, InputHandlerModule, Input};

type EventLoop = winit::event_loop::EventLoop<()>;

pub struct App {
    is_initialised: bool,

    window: Window,
    event_loop: EventLoop,

    input_handler: Mutex::<InputHandler>,

    scene: Scene,
    renderer: Renderer,
}

impl App {

    // TODO: renderer should be optional
    pub fn new(window: Window, event_loop: EventLoop, renderer: Renderer) -> Self {

        Self {
            is_initialised: false,

            window,
            event_loop,
            
            input_handler: Mutex::new(InputHandler::new()),

            scene: Scene::new(),
            renderer,
        }
    }

    pub fn init(mut self) -> Self {

        //Example Atom
        use crate::atom::{Atom, AtomObject};
        use crate::render::mesh_builder::MeshBuilder;

        let carbon = Atom {
            atomic_radius: 70.0,
            atomic_mass: 12.011,
            num_protons: 6,
            num_neutrons: 6,

            name: Some("Carbon"),
            symbol: Some("C"),
        };

        let atom_obj = AtomObject::new(
            carbon,
            MeshBuilder::gen_uv_sphere(0.5, 8, 8, Point3::<f32> { x: 0.0, y: 0.0, z: 1.0 }).build(&self.renderer.device),
            cgmath::Vector3 {
                x: 0.0,
                y: 0.0,
                z: -10.0,
            },
        );

        self.scene.set_bg_colour(Colour::new(1.0, 0.0, 0.0));
        self.scene.add_atom(atom_obj);

        self.is_initialised = true;
        return self;
    }

    pub fn run(self) {
        // Make sure the application is initialised before running it.
        assert!(self.is_initialised);

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

                    let shader_mod = self.renderer.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                        label: None,
                        source: wgpu::ShaderSource::Wgsl(include_str!("default.wgsl").into()),
                    });

                    for atom_obj in self.scene.atom_objects.iter() {

                        self.renderer.render(atom_obj, Pipeline::MeshDebugPipeline(
                            MeshDebugPipeline::new(
                                &self.renderer.device,
                                &shader_mod,
                                &shader_mod,
                                self.renderer.surface_format,
                            )
                        )).unwrap();
                    }
                },

                Event::MainEventsCleared => self.window.request_redraw(),

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