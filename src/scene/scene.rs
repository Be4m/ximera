use nalgebra::Point3;

use super::{
    SceneObject, SceneObjectKind
};

use crate::render::{
    colours,
    colours::Colour,
    camera::Camera,
};

pub struct Scene {
    pub bg_colour: Colour,

    pub camera: Camera,

    pub scene_objects: Vec<SceneObject>,
}

impl Scene {

    pub fn new(scene_size: winit::dpi::PhysicalSize<u32>) -> Self {       

        let camera = Camera {
            position: Point3::<f32>::new(0.0, 0.0, 1.0),
            target: Point3::<f32>::new(0.0, 0.0, 0.0),
            aspect_ratio: scene_size.width as f32 / scene_size.height as f32,
            fovy: std::f32::consts::PI / 2.0,
        };

        Self {
            bg_colour: colours::WHITE,

            camera,

            scene_objects: vec![],
        }
    }

    pub fn init(&mut self) {

        use crate::render::model_builder::ModelBuilder;
        use crate::atom::{Atom, AtomObject};

        let atom = Atom {
            atomic_radius: 70.0,
            atomic_mass: 12.011,
            num_protons: 6,
            num_neutrons: 6,

            name: None,
            symbol: None,
        };

        let sphere_model = ModelBuilder::build_uv_sphere(1.0, 8, 8).build();

        let atom_obj = AtomObject::new(atom, sphere_model);
        self.add_object(
            SceneObjectKind::Atom(atom_obj),
            nalgebra::Vector3::<f32>::new(0.0, 0.0, -1.0),
        );
    }

    // TODO: This might return a SceneObjectHandle or something like that.
    pub fn add_object(
        &mut self,
        object: SceneObjectKind,
        position: nalgebra::Vector3<f32>,
    ) { todo!() }

    pub fn update(&mut self) { todo!() }

    pub fn set_bg_colour(&mut self, colour: Colour) {
        self.bg_colour = colour;
    }
}