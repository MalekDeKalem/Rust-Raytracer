use crate::Vector3d;
use crate::Color;


pub struct Hit {
    pub hitpoint: Vector3d,
    pub normal: Vector3d,
    pub t: f32,
    pub col: Color
}
