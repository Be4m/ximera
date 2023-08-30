use crate::render::Model;

use super::Atom;

pub struct AtomObject {
    pub atom: Atom,
    pub model: Model,
}

impl AtomObject {

    pub fn new(
        atom: Atom,
        model: Model,
    ) -> AtomObject {

        Self {
            atom: atom,
            model,
        }
    }

    // pub fn instance(&self) -> Self {

    //     Self {
    //         atom: self.atom,

    //         uid: IdU32::new(),
    //         mesh: self.mesh,
    //         position: self.position,
    //     }
    // }
}