use nalgebra::{Vector3};
#[derive(Clone)]
pub struct Camera{
    pub position: Vector3<f32>,
    pub fov: f32,
}
