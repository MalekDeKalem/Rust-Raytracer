mod vector3d;
mod ray;
mod color;
use vector3d::Vector3d;
use ray::Ray;
use color::Color;

pub fn rayCol(r: Ray) -> Color {
    let normal: Vector3d = Vector3d::normalize(&r.dir);
    let t: f32 = 0.5 * (normal.y + 1.0);
    let v: Vector3d = Vector3d::add(Vector3d::scalar(Vector3d::new(0.3, 0.125, 0.2), (1.0 - t)), Vector3d::scalar(Vector3d::new(0.75, 0.7, 0.9), t));
    v.toColor()
}


fn main() {
    // Write PPM file 
    const RATIO: f32 = 16.0 / 9.0;
    const WIDTH: u32 = 1400;
    const HEIGHT: u32 = (800.0 / RATIO) as u32; 
    
    let vph: f32 = 2.0;
    let vpw: f32 = RATIO * vph;
    let fl: f32 = 1.0;

    let origin: Vector3d = Vector3d::new(0.0, 0.0, 0.0);
    let horizontal: Vector3d = Vector3d::new(vpw, 0.0, 0.0);
    let vertical: Vector3d = Vector3d::new(0.0, vph, 0.0);
    let llc: Vector3d = Vector3d::sub(Vector3d::sub(origin, Vector3d::scalar(horizontal, 0.5)), Vector3d::sub(Vector3d::scalar(vertical, 0.5), Vector3d::new(0.0, 0.0, fl)));

    println!("P3\n{} {}\n255", WIDTH, HEIGHT);

    for j in (0..HEIGHT).rev() {
        for i in 0..WIDTH {
            let u: f32 = i as f32 / (WIDTH - 1) as f32;
            let v: f32 = j as f32 / (HEIGHT - 1) as f32;

            let direction: Vector3d = Vector3d::add(Vector3d::add(llc, Vector3d::scalar(horizontal, u)), Vector3d::sub(Vector3d::scalar(vertical, v), origin));
            let r: Ray = Ray::new(origin, direction);
            let col: Color = rayCol(r);

            let ir: i32 = (255.99 * col.r as f32) as i32;
            let ig: i32 = (255.99 * col.g as f32) as i32;
            let ib: i32 = (255.99 * col.b as f32) as i32;

            println!("{} {} {}", ir, ig, ib);
            
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::Vector3d;
    use crate::Ray;

    
    #[test]
    fn test_add() {
        let a = Vector3d::new(3.5, -2.0, 5.2);
        let b = Vector3d::new(2.2, 1.5, 6.6);
        let c: Vector3d = Vector3d::add(a, b);
        assert_eq!(c, Vector3d::new(5.7, -0.5, 11.8));

    }

    #[test]
    fn test_sub() {
        let a = Vector3d::new(3.5, -2.0, 5.2);
        let b = Vector3d::new(2.2, 1.5, 6.6);
        let c: Vector3d = Vector3d::sub(a, b);
        assert_eq!(c, Vector3d::new(1.3, -3.5, -1.4));
    }
    
    #[test]
    fn test_normalize() {
        let a = Vector3d::new(5.0, 3.0, 2.0);
        assert_eq!(a.normalize(), Vector3d::new(1., 0., 0.));
    }

    #[test]
    fn test_ray() {
        let ray = Ray::new(Vector3d::new(0.0, 0.0, 0.0), Vector3d::new(4.0, 3.0, -5.0));
        assert_eq!(ray.at(3.0), Vector3d::new(12.0, 9.0, -15.0));
    }
    
    #[test]
    fn test_mult() {
        let a = Vector3d::new(5.6, 3.2, 1.1);
        let b = Vector3d::new(0.5, 2.2, 9.4);
        let c = Vector3d::mult(a, b);
        assert_eq!(c, Vector3d::new(2.8, 7.04, 10.34));
    }

    #[test]
    fn test_scalr() {
        let a = Vector3d::new(4.4, -1.2, 6.6);
        let t = 2.4;
        assert_eq!(Vector3d::scalar(a, t), Vector3d::new(10.56, -2.88, 15.84));
    }

}
