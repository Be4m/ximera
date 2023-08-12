/// This module probably will have to be moved somewhere else in the future

use super::utils::types::F32x3;


pub struct Location {
    x: f32,
    y: f32,
    z: f32,
}

impl From<F32x3> for Location {
    
    fn from(value: F32x3) -> Self {

        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}