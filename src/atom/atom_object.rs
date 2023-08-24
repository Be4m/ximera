use uid::IdU32;

use super::Atom;

use crate::render::Mesh;

pub struct AtomObject {
    pub atom: Atom,

    pub uid: IdU32::<Self>,
    pub mesh: Mesh,
    pub position: cgmath::Vector3::<f32>,
}

impl AtomObject {

    pub fn new(
        atom: Atom,
        mesh: Mesh,
        position: cgmath::Vector3::<f32>,
    ) -> AtomObject {

        Self {
            atom: atom,

            uid: IdU32::new(),
            mesh: mesh,
            position: position,
        }
    }

    pub fn instance(&self) -> Self {

        Self {
            atom: self.atom,

            uid: IdU32::new(),
            mesh: self.mesh.clone(),
            position: self.position,
        }
    }
}