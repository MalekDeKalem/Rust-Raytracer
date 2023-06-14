
use crate::Color;


#[derive(Debug, Clone, Copy)]
pub struct Vector3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3d {
    pub fn new(new_x: f32, new_y: f32, new_z: f32) -> Vector3d {
        let new_vec = Self {
            x: new_x,
            y: new_y,
            z: new_z
        };
        new_vec
    }

    pub fn sub(self, v: Vector3d) -> Vector3d {

        let new_vec = Vector3d {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        };
        new_vec
    }

    pub fn add(self, v: Vector3d) -> Vector3d {
        let new_vec = Vector3d {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        };
        new_vec
    }

    pub fn mult(self, v: Vector3d) -> Vector3d {
        let new_vec = Vector3d {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
        };
        new_vec
    }

    pub fn div(self, t: f32) -> Vector3d {
        let new_vec = Vector3d {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t
        };
        new_vec
    }

    pub fn len(self) -> f32 {
        self.len_sqrt().sqrt()
    }

    pub fn len_sqrt(self) -> f32 {
       self.x * self.x + self.y * self.y + self.z * self.z 
    }
    
    pub fn dot(self, v: Vector3d) -> f32 {
        self.x * v.x + self.y * v.y + self.z + v.z
    }

    pub fn cross(self, v: Vector3d) -> Vector3d {
       let new_vec = Vector3d {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        };
        new_vec
    }

    pub fn scalar(self, sca: f32) -> Vector3d {
        let new_vec = Vector3d {
            x: self.x * sca,
            y: self.y * sca,
            z: self.z * sca,
        };
        new_vec
    }

    pub fn normalize(&self) -> Vector3d {
        let new_vec = Vector3d {
            x: self.x / self.len(),
            y: self.y / self.len(),
            z: self.z / self.len(),
        };
        new_vec
    }

    pub fn toColor(&self) -> Color {
        Color::new(self.x, self.y, self.z)
    }
}


// This is for testing
impl PartialEq for Vector3d {
    fn eq(&self, other: &Self) -> bool {
        self.x.round() == other.x.round() && self.y.round() == other.y.round() && self.z.round() == other.z.round()
    }
}
