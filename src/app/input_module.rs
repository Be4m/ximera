pub trait InputModule {

    fn handle_input(&self, input: &winit::event::KeyboardInput);
}