use std::collections::HashMap;
use uid::IdU32;

use crate::render::{
    colours,
    colours::Colour,
    Renderer,
};

use crate::atom::AtomObject;

pub struct Scene {
    renderer: Renderer,

    bg_colour: Colour,

    atom_objects: HashMap::<IdU32::<AtomObject>, AtomObject>,
}

impl Scene {

    pub fn new(renderer: Renderer) -> Scene {

        Self {
            renderer,

            bg_colour: colours::WHITE,

            atom_objects: HashMap::new(),
        }
    }

    pub fn update(&self) {
    }

    pub fn set_bg_colour(&mut self, colour: Colour) {
        self.bg_colour = colour;
    }
}

impl Default for Scene {

    fn default() -> Self {

        // let atom = Atom {
        //     atomic_radius: 58.0,
        //     atomic_mass: 120.0,
        //     num_protons: 7,
        //     num_neutrons: 9,
            
        //     name: Some("Example Atom"),
        //     symbol: Some("Ex")
        // };

        // let atom_obj = AtomObject::new(
        //     &atom,
        //     AtomObject::gen_icospherical_mesh(&atom),
        //     super::Position::new(),
        // );

        // let atom_map = HashMap::new();
        // atom_map.insert(atom_obj.uid, atom_obj);

        // Self {
        //     bg_colour: colours::WHITE,

        //     atom_objects: atom_map,
        //     renderer: Renderer::default(),
        // }

        todo!()
    }
}