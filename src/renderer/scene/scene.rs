use crate::renderer::{
    colours,
    utils::geometry,
    RenderUtil,
};

use super::SceneObj;

// NOTE: Just a note for future me when it's time to take performance into account:
// Research has to be done on the best ways to handle World objects.
//
// Maybe we can separate yet leave connected rendering and CPU-side worlds (scenes in this case).
// The rendering world can use faster memory and communication,
// while this scene struct can be a little "lazy". 

/// Since this isn't a huge program that has to render lanscapes and stuff like that
/// I'm thinking of letting the Scene struct act as a World struct, meaning that it will
/// keep track of all the objects' locations and update them.
/// 
/// The update function of this struct will be like the main game loop.
pub trait Scene {}

// TODO: Perhaps rename this trait to something like SceneLoader, so it makes more sense to the reader.
impl RenderUtil for dyn Scene {

    fn render(&self) -> Result<(), wgpu::SurfaceError> {

        Ok(())
    }
}

pub struct ExampleScene {
    bg_colour: (f32, f32, f32),
    
    cube_radius: f32,
}

impl ExampleScene {

    pub fn new(
        bg_colour: (f32, f32, f32),
        cube_radius: f32,
    ) -> ExampleScene {

        let cube = geometry::gen_simple_cube(1.0);
        let cube_obj = SceneObj::new(cube, (1.0, 0.0, 0.0).into());

        Self {
            bg_colour,
            cube_radius,
        }
    }
}

impl Scene for ExampleScene {}

impl Default for ExampleScene {

    fn default() -> Self {

        ExampleScene::new(
            colours::WHITE,
            1.0,
        )
    }
}