use crate::{
    atom::AtomObject,
    render::{
        Mesh, Model,
    },
};

pub struct SceneObject {
    pub position: nalgebra::Vector3<f32>,
    pub kind: SceneObjectKind,

    pub mesh: Mesh,
}

impl SceneObject {
    pub fn new(
        position: nalgebra::Vector3<f32>,
        kind: SceneObjectKind,
        mesh: Mesh,
    ) -> Self {
        Self {
            position,
            kind,

            mesh,
        }
    }
}

pub enum SceneObjectKind {
    Atom(AtomObject),    
}

impl SceneObjectKind {
    pub fn model_mut(&mut self) -> &mut Model {
        match self {
            SceneObjectKind::Atom(atom_obj) => {
                use std::borrow::BorrowMut;
                atom_obj.model.borrow_mut()
            }
        }
    }
}