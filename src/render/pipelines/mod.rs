pub mod mesh_debug;

pub use self::{
    mesh_debug::MeshDebugPipeline,
};

pub enum Pipeline {
    MeshDebugPipeline(MeshDebugPipeline),
}

impl Pipeline {

    pub fn pipeline(&self) -> &wgpu::RenderPipeline {

        match self {
            Pipeline::MeshDebugPipeline(val) => &val.render_pipeline,
        }
    }
}