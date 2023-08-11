use tokio::runtime::Builder;

use winit::event_loop::EventLoop;

use ximera::{
    app::App, 
    renderer::*,
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

    // ** let def_scene = Scene::new();
    // let scene = Scene::new(...);
    // scene.set_bg_colour(renderer::colours::WHITE);
    // scene.load_object(obj, (7.7, 6.7, 8.0));

    // Here we're going to load everything that we're going to use for the app.
    // Then things like scenes can be switched, modules deactivated.
    App::new(window, event_loop)
        // TODO: This is very disconnected from the actual functionality this module will do.
        // What I mean by that is that it is very unclear that this CameraIM module will modify the renderer
        // Maybe we can put this somewhere deeper in the API.
        .add_input_module(CameraIM::new(&mut rendr)) // TODO: this will probably won't be static
        // ** .load_scene(def_scene) // add_scene and load_scene are supposed to be different functions.
        .render(rendr)
        .run();

    // app.add_module(Module::INPUT_MODULE, renderer::CameraIM);
    // app.add_input_module(renderer::CameraIM);
    // app.add_input_module(renderer::CameraIM::new(&mut renderer));
    
    // app.load_scene(scene);
    // app.set_active_scene(scene);
}
