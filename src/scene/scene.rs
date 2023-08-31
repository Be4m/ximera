use uid::*;
use nalgebra::Vector3;

use std::collections::HashMap;

use super::SceneObject;

use crate::render::{camera::Camera, Model};

pub struct Scene {
    pub bg_colour: wgpu::Color,

    pub camera: Camera,

    pub scene_objects_map: HashMap<u8, SceneObject>,
}

impl Scene {

    pub fn new(
        resolution: winit::dpi::PhysicalSize<u32>,
        bg_colour: wgpu::Color,
    ) -> Self {       

        let camera = Camera {
            aspect_ratio: resolution.width as f32 / resolution.height as f32,
            ..Default::default()
        };

        Self {
            bg_colour,
            
            camera,
            scene_objects_map: HashMap::new(),
        }
    }

    // As of now used for debug purposes
    pub fn init(&mut self) {
        use crate::render::model_builder::ModelBuilder;
        use crate::render::pipelines::PipelineKind;

        let sphere_model = ModelBuilder::build_uv_sphere(1.0, 8, 8)
            .set_pipeline(PipelineKind::MeshDebug)
            .build();

        let sphere_id = self.add_object(sphere_model, Vector3::new(0.0, 0.0, 0.0));

        self.move_object(&sphere_id, Vector3::new(0.0, 0.0, -10.0)).unwrap();
    }

    pub fn add_object(
        &mut self,
        model: Model,
        position: Vector3<f32>,
    ) -> IdU8<SceneObject> {
        let id = IdU8::new();
        let scene_obj = SceneObject::new(position, model);
        self.scene_objects_map.insert(id.clone().get(), scene_obj);

        return id;
    }

    pub fn move_object(
        &mut self,
        object_id: &IdU8<SceneObject>,
        vector: Vector3<f32>,
    ) -> Result<(), ()> {
        let obj = self.scene_objects_map
            .get_mut(&object_id.clone().get())
            .ok_or(())?;

        obj.model.transform
            .matrix_mut()
            .append_translation_mut(&vector);

        Ok(())
    }

    pub fn update(&mut self) { todo!() }

    pub fn set_bg_colour(&mut self, colour: wgpu::Color) {
        self.bg_colour = colour;
    }
}