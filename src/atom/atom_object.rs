use uid::IdU32;

use super::Atom;

pub struct AtomObject {
    pub atom: Atom,

    pub uid: IdU32<Self>,
    pub mesh: crate::render::Mesh,
    pub position: crate::scene::Position,
}

impl AtomObject {

    pub fn new(
        atom: &Atom,
        mesh: &crate::render::Mesh,
        position: &crate::scene::Position,
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