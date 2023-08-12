use crate::{
    object::Object,
    geometry::Geometry,
    renderer::location::Location,
};


pub struct SceneObject {
    location: Location,
    // mesh: Mesh,
    geometry: Geometry,
}

impl SceneObject {

    pub fn new(
        geometry: Geometry, // TODO: Soon to be mesh
        location: Location,
    ) -> SceneObject {

        Self {
            location,
            geometry,
        }
    }
}

// I'm not sure about this one, but might come in handy in the future.
impl Object for SceneObject {}