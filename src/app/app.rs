use std::sync::Mutex;

use winit::{
    event::*,
    event_loop::ControlFlow,
    window::Window,
};

use crate::{
    render::{Renderer, camera::CameraController},
    scene::Scene,
};

use super::{
    InputHandler, Input,
    InputHandlerModule, InputTypes, InputHandlerModuleKind,
};

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

    pub fn new(window: Window, event_loop: EventLoop, renderer: Renderer) -> Self {

        let scene = Scene::new(renderer.resolution, wgpu::Color::RED);

        let camera_controller = CameraController {};

        // TODO: Probably a good idea to move module creation code somewhere else.
        let input_handler = InputHandler::new(&mut [
            InputHandlerModule {
                accepted_input: InputTypes::KEYBOARD_INPUT | InputTypes::MOUSE_INPUT,
                kind: InputHandlerModuleKind::CameraControllerIHM(camera_controller),
            },
        ]);

        Self {
            is_initialised: false,

            window,
            event_loop,
            
            input_handler: Mutex::new(input_handler),

            scene,
            renderer,
        }
    }

    pub fn init(&mut self) {

        // self.scene.init();

        self.is_initialised = true;
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
                    
                    // SPHERE DEMO
                    use crate::render::model_builder::ModelBuilder;
                    use crate::render::pipelines::PipelineKind;

                    let sphere_model = ModelBuilder::build_uv_sphere(0.5, 32, 32)
                        .set_pipeline(PipelineKind::MeshDebug)
                        .build();

                    let sphere_mesh = sphere_model.create_mesh(
                        &self.renderer.device,
                        &self.renderer.bind_group_layouts,
                    );

                    self.renderer.render_mesh(&sphere_mesh).unwrap();
                },

                Event::MainEventsCleared => self.window.request_redraw(),

                _ => {}
            }
        });
    }
}