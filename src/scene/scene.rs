use std::collections::HashMap;
use uid::IdU32;

use crate::render::mesh_builder::MeshBuilder;
use crate::render::{
    colours,
    colours::Colour,
    Renderer,
};

use crate::atom::{
    Atom, AtomObject,
};

pub struct Scene {
    pub bg_colour: Colour,

    pub atom_objects: Vec<AtomObject>,
}

impl Scene {

    pub fn new() -> Scene {

        Self {
            bg_colour: colours::WHITE,

            atom_objects: vec![],
        }
    }

    pub fn update(&self) {
    }

    pub fn add_atom(&mut self, atom_object: AtomObject) {
        self.atom_objects.push(atom_object);
    }

    pub fn set_bg_colour(&mut self, colour: Colour) {
        self.bg_colour = colour;
    }
}

// impl Default for Scene {

//     fn default() -> Self {
//         let atom = Atom {
//             atomic_radius: 58.0,
//             atomic_mass: 120.0,
//             num_protons: 7,
//             num_neutrons: 9,
            
//             name: Some("Example Atom"),
//             symbol: Some("Ex")
//         };

//         let atom_obj = AtomObject::new(
//             atom,
//             MeshBuilder::gen_uv_sphere(2.0, 8, 8).build(),
//             cgmath::Vector3 {
//                 x: 0.0,
//                 y: 0.0,
//                 z: -1.0,
//             },
//         );

//         let atom_map = HashMap::new();
//         atom_map.insert(atom_obj.uid, atom_obj);

//         Self {
//             bg_colour: colours::WHITE,

//             atom_objects: atom_map,
//             renderer: Renderer::default(),
//         }
//     }
// }