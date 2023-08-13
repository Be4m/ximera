use crate::scene::Object;


#[derive(Clone)]
pub struct Atom {
    atomic_radius: f32,
    atomic_mass: f32,
    num_protons: u32,
    num_neutrons: u32,
    
    name: Option<String>,
    symbol: Option<String>,
    
    //mesh: crate::scene::Mesh,
}

pub struct AtomObject {
    atom: Atom,

    mesh: crate::renderer::Mesh,
    position: crate::scene::Position,
}

impl AtomObject {

    pub fn instance(
        atom: &Atom,
        mesh: &crate::renderer::Mesh,
        position: &crate::scene::Position,
    ) -> AtomObject {

        Self {
            atom: atom.clone(),

            mesh: mesh.clone(),
            position: position.clone(),
        }
    }
}

impl Object for AtomObject {

    fn mesh(&self) -> &crate::renderer::Mesh {
        &self.mesh
    }

    fn position(&self) -> &crate::scene::Position {
        &self.position
    }

    // THIS IS WHERE YOU CAN DEFINE CUSTOM BEHAVIOUR FOR ALL ATOMS
    fn update(&mut self) {
        
    }
}

impl crate::app::Asset for Atom {}

macro_rules! load_atom_from_file {
    () => {

    }
}