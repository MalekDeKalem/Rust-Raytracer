use crate::Hit;
use crate::Vector3d;
use crate::Color;


pub struct Sphere {
    center: Vector3d,
    radius: f32,
    col: Color
}

impl Sphere {
    pub fn new(center: Vector3d, radius: f32, col: Color) -> Self {
        let new_sphere = Self {
            center: center,
            radius: radius,
            col: col
        };
        new_sphere
    }

    pub fn intersection(self, ray: Ray, threshold_min: f32, threshold_max: f32) -> Hit {
        let oc: Vector3d = Vector3d::sub(ray.origin, self.center);
        // Defining a, b and c for the intersection checking
        let a: f32 = ray.dir.len_sqrt();
        let b: f32 = Vector3d::dot(oc, ray.dir);
        let c: f32 = oc.len_sqrt() - self.radius * self.radius;

        let dis: f32 = b * b - a * c;
        
        if dis < 0 {
            return false;
        }

        let dis_sqrt: f32 = sqrt(dis);
        let mut root: Vector3d = (-b - dis_sqrt) / a;
        
        if root < threshold_min || threshold_max < root {
            root = (-b + dis_sqrt) / a;
            if root < threshold_min || threshold_max < root {
                return false;
            }
        }

        let h: Hit = Hit::new(ray.at(root), Vector3d::div(Vector3d::sub(ray.at(root), self.center), self.radius), root, self.col);

        return h;
    }
}
