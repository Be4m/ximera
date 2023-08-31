use nalgebra::{
    Point3, Transform3,
};

use super::{
    Vertex,
    mesh::Model,
    pipelines::PipelineKind,
};

pub struct ModelBuilder {
    points: Vec<Point3<f32>>,

    triangles: Vec<[i16; 3]>,
    quads: Vec<[i16; 4]>,

    special_pipeline: Option<PipelineKind>,
}

impl ModelBuilder {

    pub fn new() -> Self {

        Self {
            points: vec![],

            triangles: vec![],
            quads: vec![],

            special_pipeline: None,
        }
    }

    pub fn add_point(&mut self, position: Point3<f32>) {
        self.points.push(position);
    }

    pub fn index_triangle(&mut self, triangle: [i16; 3]) {
        self.triangles.push(triangle);
    }

    pub fn index_quad(&mut self, quad: [i16; 4]) {
        self.quads.push(quad);
    }

    fn triangulate_quad(quad: [i16; 4]) -> [[i16; 3]; 2] {
        let tri0 = [
            quad[0],
            quad[1],
            quad[2],
        ];
        let tri1 = [
            quad[2],
            quad[3],
            quad[0],
        ];

        return [tri0, tri1];
    }

    pub fn set_pipeline(mut self, pipeline: PipelineKind) -> Self {
        self.special_pipeline = Some(pipeline);
        return self;
    }

    pub fn build(self) -> Model {
        let index_data = vec![
            self.triangles.into_iter()
                .flatten()
                .collect::<Vec<_>>(),
            self.quads.into_iter()
                .flat_map(Self::triangulate_quad)
                .flatten()
                .collect::<Vec<_>>()
        ].concat();
        
        let vertex_data = self.points
            .into_iter()
            .map(From::from)
            .collect::<Vec<Vertex>>();

        let num_vertices = vertex_data.len() as u32;
        let num_indices = index_data.len() as u32;

        Model {
            transform: Transform3::<f32>::identity(),
            vertex_data,
            index_data,
            num_vertices,
            num_indices,
            pipeline_kind: self.special_pipeline.unwrap_or(PipelineKind::Model),
        }
    }

    pub fn build_uv_sphere(
        radius: f32,
        num_slices: i16,
        num_stacks: i16,
    ) -> Self {
        let mut model_builder = ModelBuilder::new();

        // Add top point
        model_builder.add_point(Point3::new(0.0, radius, 0.0));

        // Generate points for each slice in each stack
        for stack in 0..(num_stacks - 1) {
            let phi = std::f32::consts::PI / (num_stacks as f32) * (stack as f32 + 1.0);

            for slice in 0..num_slices {
                let theta = std::f32::consts::TAU / (num_slices as f32) * (slice as f32);

                // Convert to cartesian coordinate system
                let x = phi.sin() * theta.cos() * radius;
                let y = phi.cos() * radius;
                let z = phi.sin() * theta.sin() * radius;

                model_builder.add_point(Point3::new(x, y, z));
            }
        }

        // Add bottom point
        model_builder.add_point(Point3::new(0.0, -radius, 0.0));

        // Add top & bottom triangles
        let itop = 0;
        let ibottom = (num_stacks - 1) * num_slices + 1;

        for slice in 0..num_slices {
            // We're adding 1 at the end of each index to offset the top point.
            let i0 = slice + 1;
            let i1 = (slice + 1) % num_slices + 1;
            model_builder.index_triangle([i0, i1, itop]);

            let offset = (num_stacks - 2) * num_slices;
            let i0 = offset + i0;
            let i1 = offset + i1;
            model_builder.index_triangle([i0, i1, ibottom]);
        }

        return model_builder;
    }
}