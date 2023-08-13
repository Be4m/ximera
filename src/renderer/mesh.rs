type Vertex = (f32, f32, f32);


/// The struct Mesh is all what Renderer needs to render an 'object' or mesh.
/// The Mesh struct, not like Geometry struct, is low-level representation of a geometry,
/// containing only a vertex and index buffers.
/// 
/// The Geometry struct is based on triangles and has convinient functions, like: add_quad.
/// Before rendering, geometry has to be compiled down to a Mesh struct. To do so you can use
/// a method of Geometry's called `gen_mesh`.
#[derive(Clone)]
pub struct Mesh {
    vertex_buffer: Vec::<Vertex>,
    index_buffer: Vec::<i16>,
}

impl Mesh {

    pub fn new(
        vertex_buffer: Vec::<Vertex>,
        index_buffer: Vec<i16>,
    ) -> Mesh {

        Self {
            vertex_buffer,
            index_buffer,
        }
    }
}