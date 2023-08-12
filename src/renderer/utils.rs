pub mod geometry {
    use crate::geometry::{Geometry, Quad, Triangle};


    // TODO: Maybe there's a more compact & efficient way to do this.
    pub fn gen_simple_cube(r: f32) -> Geometry {

        let mut geometry = Geometry::new();

        geometry.add_quads(&[
            Quad::new([
                (-r,  r, -r),
                ( r,  r, -r),
                ( r, -r, -r),
                (-r, -r, -r),
            ]),

            Quad::new([
                ( r,  r, -r),
                ( r,  r,  r),
                ( r, -r,  r),
                ( r, -r, -r),
            ]),

            Quad::new([
                ( r,  r,  r),
                (-r,  r,  r),
                (-r, -r,  r),
                ( r, -r,  r),
            ]),

            Quad::new([
                (-r,  r,  r),
                (-r,  r, -r),
                (-r, -r, -r),
                (-r, -r,  r),
            ]),

            Quad::new([
                (-r,  r, -r),
                ( r,  r, -r),
                ( r,  r,  r),
                (-r,  r,  r),
            ]),

            Quad::new([
                (-r, -r, -r),
                ( r, -r, -r),
                ( r, -r,  r),
                (-r, -r,  r),
            ]),
        ]);

        return geometry;
    }

    #[inline]
    pub fn empty_triangle() -> Triangle {
        [(0.0, 0.0, 0.0); 3]
    }
}

pub mod types {

    pub type F32x3 = (f32, f32, f32);
}