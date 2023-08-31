pub mod mesh_debug;
pub mod model;

pub type BindGroupLayoutMap = [bool; BindGroupLayouts::NUM_LAYOUTS];

// Reexports
pub use self::{
    mesh_debug::MeshDebugPipeline,
    model::ModelPipeline,
};

macro_rules! bind_group_map {
    (
        $bound: expr,
        $( $layout: expr ),*
    ) => {
        {
            #[allow(unused_mut)] // This must be a bug
            let mut temp_array: [bool; $bound] = [false; $bound];

            $(
                temp_array[$layout as usize] = true;
            )*
            temp_array
        }
    };
}
pub(crate) use bind_group_map;

#[derive(Clone, Copy)]
pub enum PipelineKind {
    Dummy,
    Model,
    MeshDebug,
}

impl PipelineKind {
    pub fn get_pipeline<'a>(&'a self, pipelines: &'a Pipelines) -> Pipeline {
        match self {
            PipelineKind::MeshDebug => Pipeline::MeshDebugPipeline(&pipelines.mesh_debug),
            _ => todo!(),
        }
    }

    // TODO: Since we're using match anyway, This should done even less abstract.
    pub fn used_bind_groups(&self) -> Vec<BindGroupKind> {
        match self {
            PipelineKind::Model => {
                ModelPipeline::BIND_GROUP_MAP
                    .into_iter()
                    .enumerate()
                    .filter(|(_, b)| *b)
                    .map(|(i, _)| {
                        <usize as TryInto<BindGroupKind>>::try_into(i).unwrap()
                    })
                    .collect::<Vec<_>>()
            },
            PipelineKind::MeshDebug => {
                MeshDebugPipeline::BIND_GROUP_MAP
                    .into_iter()
                    .enumerate()
                    .filter(|(_, b)| *b)
                    .map(|(i, _)| {
                        <usize as TryInto<BindGroupKind>>::try_into(i).unwrap()
                    })
                    .collect::<Vec<_>>()
            },
            _ => todo!()
        }
    }
}

pub enum Pipeline<'a> {
    MeshDebugPipeline(&'a MeshDebugPipeline),
}

impl<'a> Pipeline<'a> {
    pub fn render_pipeline(&self) -> &wgpu::RenderPipeline {
        match self {
            Pipeline::MeshDebugPipeline(pipeline) => &pipeline.render_pipeline,
        }
    }
}

pub struct Pipelines {
    pub mesh_debug: MeshDebugPipeline,
}

#[repr(u8)]
pub enum BindGroupKind {
    Model = 0,
}

impl std::convert::TryFrom<usize> for BindGroupKind {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BindGroupKind::Model),
            _ => Err(()),
        }
    }
}

pub struct BindGroupLayouts {
    pub model: wgpu::BindGroupLayout,
}

impl BindGroupLayouts {
    const NUM_LAYOUTS: usize = 1;

    pub fn create(device: &wgpu::Device) -> Self {
        let model = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        Self {
            model,
        }
    }
}