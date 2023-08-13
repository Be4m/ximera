use crate::utils::types::F32x3;


#[derive(Clone, Copy)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Colour {

    pub const fn new(r: f32, g: f32, b: f32) -> Colour {

        Self { r, g, b }
    }
}

impl From<F32x3> for Colour {

    fn from(value: F32x3) -> Self {
        
        Self {
            r: value.0,
            g: value.1,
            b: value.2,
        }
    }
}

pub static WHITE: Colour = Colour::new(1.0, 1.0, 1.0);
pub static GREY:  Colour = Colour::new(0.7, 0.7, 0.7);
pub static BLACK: Colour = Colour::new(0.0, 0.0, 0.0);