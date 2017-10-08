use std::f64;
use ::vec3::Vec3;
use ::ray::Ray;
use ::hitable::{Hitable, HitRecord};
use ::material::Material;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Box<Material>,
}

impl Sphere {
    pub fn new(c: (f64, f64, f64), r: f64, m_type: char, m_alb: (f64, f64, f64)) -> Sphere {
        //if m_type == 'l' {         <- use an enum for different types of materials
        Sphere{ center: Vec3::new(c.0, c.1, c.2),
                radius: r
                material: }
    }
    pub fn new_v(c: &Vec3, r: f64, m: &Vec3) -> Sphere {
        Sphere{ center: Vec3::new(c[0], c[1], c[2]),
                radius: r}
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin.v_sub(&self.center);
        let a = r.dir.dot(&r.dir);
        let b = oc.dot(&r.dir);
        let c = oc.dot(&oc) - (self.radius * self.radius);
        let disc = b*b - a*c;

        let mut hit = None;
        
        if disc > 0.0{
            let tmp = (-b - disc.sqrt()) / a;
            if (tmp < t_max) & (tmp > t_min) {
                let p = r.pt_at_t(tmp);
                hit = Some(HitRecord {
                    t : tmp,
                    p : r.pt_at_t(tmp),
                    normal : p.v_sub(&self.center).s_mult(1.0 / self.radius),
                });
                return hit
            }

            let tmp = (-b + disc.sqrt()) / a;
            if (tmp < t_max) & (tmp > t_min) {
                let p = r.pt_at_t(tmp);
                hit = Some(HitRecord {
                    t : tmp,
                    p : r.pt_at_t(tmp),
                    normal : p.v_sub(&self.center).s_mult(1.0 / self.radius),
                });
                return hit
            }
        }

        hit

    }
}
