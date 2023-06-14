pub use crate::vector3d::Vector3d;

pub struct Ray {
    pub origin: Vector3d,
    pub dir: Vector3d
}

impl Ray {

    pub fn new(org: Vector3d, dir: Vector3d) -> Ray {
         let new_ray = Self {
            origin: org,
            dir: dir
        };
        new_ray
    }

    pub fn at(self,t: f32) -> Vector3d {
        Vector3d::add(self.origin, Vector3d::scalar(self.dir, t))
    }
}
