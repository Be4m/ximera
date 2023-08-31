use crate::{
    atom::AtomObject,
    render::Model,
};

#[derive(Clone)]
pub struct SceneObject {
    pub position: nalgebra::Vector3<f32>,
    pub model: Model,
}

impl SceneObject {
    pub fn new(
        position: nalgebra::Vector3<f32>,
        model: Model,
    ) -> Self {
        Self {
            position,
            model,
        }
    }
}

#[derive(Clone)]
pub enum SceneObjectKind {
    Atom(AtomObject),    
}

impl SceneObjectKind {
    pub fn model(&self) -> &Model {
        match self {
            SceneObjectKind::Atom(atom_obj) => {
                &atom_obj.model
            }
        }
    }

    pub fn model_mut(&mut self) -> &mut Model {
        match self {
            SceneObjectKind::Atom(atom_obj) => {
                use std::borrow::BorrowMut;
                atom_obj.model.borrow_mut()
            }
        }
    }
}