pub use VertexOrder::ClockWise as CW;
pub use VertexOrder::CounterClockWise as CCW;

use crate::utils::types::*;


pub enum VertexOrder {
    ClockWise,
    CounterClockWise,
}

pub struct Quad {
    vertex_order: VertexOrder,

    v0: Vertex,
    v1: Vertex,
    v2: Vertex,
    v3: Vertex,
}

impl Quad {

    pub fn new(
        vertices: [Vertex; 4],
        vertex_order: VertexOrder,
    ) -> Quad {

        Self {
            vertex_order,

            v0: vertices[0],
            v1: vertices[1],
            v2: vertices[2],
            v3: vertices[3],
        }
    }

    // TODO: I'm not sure how is this going to hold up in the long run since
    // here we basically rely on the quad's creator that the vertices will be added in the right order.
    pub fn triangulate(&self) -> [Triangle; 2] {

        match self.vertex_order {

            VertexOrder::ClockWise => {
                [
                    [self.v0, self.v1, self.v2],
                    [self.v2, self.v3, self.v0],
                ]
            },

            VertexOrder::CounterClockWise => {
                [
                    [self.v0, self.v3, self.v2],
                    [self.v2, self.v1, self.v0],
                ]
            }
        }
    }
}