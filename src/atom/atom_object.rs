use uid::IdU32;

use super::Atom;

use crate::{render::Mesh, scene::Position};

pub struct AtomObject {
    pub atom: Atom,

    pub uid: IdU32::<Self>,
    pub mesh: Mesh,
    pub position: Position,
}

impl AtomObject {

    pub fn new(
        atom: &Atom,
        mesh: &Mesh,
        position: &Position,
    ) -> AtomObject {

        Self {
            atom: atom.clone(),

            uid: IdU32::new(),
            mesh: mesh.clone(),
            position: position.clone(),
        }
    }

    pub fn instance(&self) -> Self {

        let new_uid = IdU32::new();
        let new_pos = self.position.mut_up(1.0);

        Self {
            atom: self.atom.clone(),

            uid: new_uid,
            mesh: self.mesh.clone(),
            position: new_pos,
        }
    }
}