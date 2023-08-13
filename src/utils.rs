pub mod geometry {

    use crate::renderer::{
        quad,
        Quad, Geometry,
    };


    // TODO: Maybe there's a more compact & efficient way to do this.
    pub fn gen_simple_cube(r: f32) -> Geometry {

        let mut geometry = Geometry::new();

        geometry.add_quads(&[
            Quad::new([
                (-r,  r, -r),
                ( r,  r, -r),
                ( r, -r, -r),
                (-r, -r, -r),
            ], quad::CW),

            Quad::new([
                ( r,  r, -r),
                ( r,  r,  r),
                ( r, -r,  r),
                ( r, -r, -r),
            ], quad::CW),

            Quad::new([
                ( r,  r,  r),
                (-r,  r,  r),
                (-r, -r,  r),
                ( r, -r,  r),
            ], quad:: CW),

            Quad::new([
                (-r,  r,  r),
                (-r,  r, -r),
                (-r, -r, -r),
                (-r, -r,  r),
            ], quad::CW),

            Quad::new([
                (-r,  r, -r),
                ( r,  r, -r),
                ( r,  r,  r),
                (-r,  r,  r),
            ], quad::CW),

            Quad::new([
                (-r, -r, -r),
                ( r, -r, -r),
                ( r, -r,  r),
                (-r, -r,  r),
            ], quad::CW),
        ]);

        return geometry;
    }
}

pub mod types {

    #[deprecated]
    pub type F32x3 = (f32, f32, f32);

    pub type Vertex   = F32x3;
    pub type Triangle = [Vertex; 3];
}