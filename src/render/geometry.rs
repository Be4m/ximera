use super::{
    Mesh,
    Quad,
};

use crate::utils::types::*;

/// High level representation of a 3d object, before rendering has to be compiled down
/// to a Mesh struct.
pub struct Geometry {
    triangles: Vec::<Triangle>,
}

impl Geometry {

    pub fn new() -> Geometry {

        Self {
            triangles: vec![],
        }
    }

    /// Compiles Geometry to a renderable Mesh.
    /// 
    /// This method is very slow and expensive, so any geometry compilation should be
    /// done when the app initialises.
    pub fn compile(&self) -> Mesh {

        todo!()
    }

    pub fn add_triangles(&mut self, triangles: &[Triangle]) {
        self.triangles.extend(triangles);
    }

    pub fn add_quads(&mut self, quads: &[Quad]) {

        self.triangles.extend(
            quads.iter()
                .flat_map(|q| q.triangulate())
                .collect::<Vec<Triangle>>()
        );
    }
}