/// This module probably will have to be moved somewhere else in the future

use crate::utils::types::F32x3;


#[derive(Clone, Copy, Default)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}

impl Position {

    pub fn mut_up(&self, scalar: f32) -> Self {

        Self {
            x: self.x,
            y: self.y + scalar,
            z: self.z,
        }
    }
}

impl From<F32x3> for Position {
    
    fn from(value: F32x3) -> Self {

        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}