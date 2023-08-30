use nalgebra::{
    Isometry3, Perspective3,
    Vector3, Point3, Matrix4,
};

use crate::app::Input;

#[derive(Clone)]
pub struct CameraController {
}

impl CameraController {
    pub fn process_input(&mut self, input: Input) {}

    pub fn update_camera(&self, camera: &mut Camera) {}
}

pub struct Camera {
    pub position: Point3<f32>,
    pub target: Point3<f32>,
    pub aspect_ratio: f32,
    pub fovy: f32,
}

impl Camera {
    pub fn compute_vp_matrix(&self) -> Matrix4<f32> {
        let view = Isometry3::look_at_lh(&self.position, &self.target, &Vector3::y());
        let projection = Perspective3::new(self.aspect_ratio, self.fovy, 0.1, 100.0);

        view.to_homogeneous() * projection.as_matrix()
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Point3::<f32>::new(0.0, 0.0, 1.0),
            target: Point3::<f32>::new(0.0, 0.0, 0.0),
            aspect_ratio: 16.0 / 9.0,
            fovy: std::f32::consts::PI / 2.0,
        }
    }
}