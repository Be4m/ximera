pub type Vertex   = (f32, f32, f32);
pub type Triangle = [Vertex; 3];

/// I might just make a new struct "mesh" which will be a lower level representation
/// of object's geometry, meaning that a mesh has to be fully prepared and
/// with all safety checks done.
/// 
/// TODO:
/// New thought: I should probably replace all of this with a Mesh & MeshBuilder structs
/// that way we can use helpful functions like add_quad or add_triangle on the MeshBuilder
/// object, but when we build it, all of our convenient triangles and quads compile down to a
/// buffer & index buffer.
pub struct Geometry {
    // MESH ->
    vertices: Vec::<Vertex>,
    indices: Vec::<i16>,

    // GEOMETRY ->
    triangles: Vec::<Triangle>,
}

impl Geometry {

    pub fn new() -> Geometry {

        Self {
            vertices: vec![],
            indices: vec![],

            triangles: vec![],
        }
    }

    pub fn add_triangle(&mut self) {
    }

    pub fn add_quads(&mut self, quads: &[Quad]) {

        self.triangles.extend(
            quads.into_iter()
                .flat_map(|q| q.triangulate())
                .collect::<Vec<Triangle>>()
        );
    }

    // Turn triangles and quads into vertices and indices.
    //fn compile();
}

trait VertexComplex {

    fn triangulate(&self) -> Vec::<Triangle>;
}

pub struct Quad {
    v0: Vertex,
    v1: Vertex,
    v2: Vertex,
    v3: Vertex,
}

impl Quad {

    pub fn new(vertices: [Vertex; 4]) -> Quad {

        Self {
            v0: vertices[0],
            v1: vertices[1],
            v2: vertices[2],
            v3: vertices[3],
        }
    }
}

impl From<[Vertex; 4]> for Quad {

    fn from(value: [Vertex; 4]) -> Self {
        
        Self {
            v0: value[0],
            v1: value[1],
            v2: value[2],
            v3: value[3],
        }
    }
}

impl VertexComplex for Quad {

    // TODO: I'm not sure how is this going to hold up in the long run since
    // here we basically rely on the quad's creator that the vertices will be added in the right order.
    fn triangulate(&self) -> Vec::<Triangle> {

        vec![
            [self.v0, self.v1, self.v2],
            [self.v2, self.v3, self.v0],
        ]
    }
}