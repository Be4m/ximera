use tokio::runtime::Builder;

use winit::event_loop::EventLoop;

use ximera::{
    render::Renderer,
    app::App,
};

use ximera::app::input_handler::modules::CameraIHM;

fn main() {

    let tokio_runtime = Builder::new_current_thread()
        .build()
        .unwrap();

    let event_loop = EventLoop::new();

    // create window
    let window = winit::window::WindowBuilder::new()
        .with_title("Ximera")
        .build(&event_loop)
        .unwrap();
    // ------------

    let mut rendr = Renderer::new(&window, &tokio_runtime);

    let camera_ihm = CameraIHM::new(&mut rendr);


    App::new(window, event_loop, rendr)
        .add_input_handler_module(CameraIHM::module(camera_ihm))
        .run();
}
