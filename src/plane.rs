use std::f64;
use ::vec3::Vec3;
use ::ray::Ray;
use ::hitable::{Hitable, HitRecord};
use ::material::Material;

pub struct Plane {
    pub normal: Vec3,
    pub d: f64,
    pub material: Material,
}

impl Plane {
    pub fn new(n: (f64, f64, f64), d: f64, m: &Material) -> Plane {
        Plane{ normal: Vec3::new(n.0, n.1, n.2).get_unit() * -1.0,
                d: d,
                material: m.clone()}
    }
    pub fn new_v(n: Vec3, d: f64, m: &Material) -> Plane {
        Plane{ normal: n.get_unit(),
                d: d,
                material: m.clone()}
    }
}

impl Hitable for Plane {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit = None;

        let denominator = self.normal.dot(&r.dir);

        if denominator != 0.0{
            let tmp = (-self.d - (-1.0 * self.normal).dot(&r.origin)) / denominator;
            if (tmp < t_max) & (tmp > t_min) {
                let p = r.pt_at_t(tmp);
                hit = Some(HitRecord {
                    t : tmp,
                    p : p,
                    normal : self.normal,
                    mat : self.material.clone(),
                });
                return hit
            }
            
        }

        hit

    }
}
