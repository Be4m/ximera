use tokio::runtime::Builder;

use winit::event_loop::EventLoop;

use ximera::{
    render::Renderer,
    app::App,
};

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

    let rendr = Renderer::new(&window, &tokio_runtime);

    let mut app = App::new(window, event_loop, rendr);

    app.init();
    app.run();
}