use std::f64;
use ::vec3::Vec3;
use ::ray::Ray;
use ::hitable::{Hitable, HitRecord};
use ::material::Material;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(c_x: f64, c_y: f64, c_z: f64, r: f64, m: &Material) -> Sphere {
        Sphere{ center: Vec3::new(c_x, c_y, c_z),
                radius: r,
                material: m.clone()}
    }
    pub fn new_v(c: Vec3, r: f64, m: &Material) -> Sphere {
        Sphere{ center: c,
                radius: r,
                material: m.clone()}
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.dir.dot(&r.dir);
        let b = oc.dot(&r.dir);
        let c = oc.dot(&oc) - (self.radius * self.radius);
        let disc = b*b - a*c;

        let mut hit = None;
        
        if disc >= 0.0{
            let tmp = (-b - disc.sqrt()) / a;
            if (tmp < t_max) & (tmp > t_min) {
                let p = r.pt_at_t(tmp);
                hit = Some(HitRecord {
                    t : tmp,
                    p : r.pt_at_t(tmp),
                    normal : (p.v_sub(&self.center)).s_mult(1.0 / self.radius),
                    mat : self.material.clone(),
                });
                return hit
            }

            let tmp = (-b + disc.sqrt()) / a;
            if (tmp < t_max) & (tmp > t_min) {
                let p = r.pt_at_t(tmp);
                hit = Some(HitRecord {
                    t : tmp,
                    p : r.pt_at_t(tmp),
                    normal : (p.v_sub(&self.center)).s_mult(1.0 / self.radius),
                    mat : self.material.clone(),
                });
                return hit
            }
        }

        hit

    }
}
