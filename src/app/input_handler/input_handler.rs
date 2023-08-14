use winit::event::KeyboardInput;

use super::InputHandlerModule;

pub enum InputType<'a> {
    KeyboardInput(&'a KeyboardInput),
    MouseMotion(&'a (f64, f64)),
    MouseWheelSpin,
}

pub struct InputHandler {
    keyboard_handlers: Vec::<Box::<dyn InputHandlerModule>>,
    mouse_handlers: Vec::<Box::<dyn InputHandlerModule>>,
}

impl InputHandler {

    pub fn new() -> Self {

        Self {
            keyboard_handlers: vec![],
            mouse_handlers: vec![],
        }
    }

    pub fn forward_keyboard_input(&mut self, input: &KeyboardInput) {

        for handler in self.keyboard_handlers.iter_mut() {

            handler.process_input(InputType::KeyboardInput(input));
        }
    }

    pub fn forward_mouse_motion(&mut self, delta: &(f64, f64)) {

        for handler in self.keyboard_handlers.iter_mut() {

            handler.process_input(InputType::MouseMotion(delta));
        }
    }

    pub fn add_module(&mut self, module: Box::<dyn InputHandlerModule>) {

        for t in module.accepted_input_types().iter() {
            match t {
                InputType::KeyboardInput(_) => self.keyboard_handlers.push(module.clone()),
                InputType::MouseMotion(_) => self.mouse_handlers.push(module.clone()),
                _ => {}
            }
        }
    }

    pub fn process_modules(&self) {


    }
}

impl<T> InputHandlerModule for Box<T>
where
    T: InputHandlerModule + Clone + ?Sized + 'static
{

    #[inline]
    fn accepted_input_types(&self) -> Vec::<InputType> {
        (**self).accepted_input_types()
    }

    #[inline]
    fn process_input(&mut self, input_type: InputType) {
        (**self).process_input(input_type);
    }
}