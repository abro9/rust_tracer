use std::f64;
use ::vec3::Vec3;
use ::ray::Ray;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(c: &Vec3, r: f64) -> Sphere {
        Sphere{ center: Vec3::new(c[0], c[1], c[2]),
                radius: r}
    }

    pub fn hit(&self, r: &Ray) -> f64 {
        let oc = r.origin.v_sub(&self.center);
        let a = r.dir.dot(&r.dir);
        let b = 2.0 * oc.dot(&r.dir);
        let c = oc.dot(&oc) - (self.radius * self.radius);
        let disc = b*b - 4.0*a*c;
        
        if disc < 0.0{
            -1.0
        }
        else{
            (-b - disc.sqrt()) / (2.0 * a) 
        }
    }
}
