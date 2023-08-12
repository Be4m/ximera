use tokio::runtime::Builder;

use winit::event_loop::EventLoop;

use ximera::{
    app::App, 
    renderer::{*, scene::{Scene, ExampleScene}},
};

fn main() {

    // --- INIT ---
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

    // Here we're going to load everything that we're going to use for the app.
    // Then things like scenes can be switched, modules deactivated.
    App::new(window, event_loop)
        .add_input_module(CameraIM::new(&mut rendr))
        .init_scene_loader(rendr)
        .run();

    // app.add_module(Module::INPUT_MODULE, renderer::CameraIM);
    // app.add_input_module(renderer::CameraIM);
    // app.add_input_module(renderer::CameraIM::new(&mut renderer));
    
    // app.load_scene(scene);
    // app.set_active_scene(scene);
}
