extern crate rand;

use rand::Rng;

use ::vec3::Vec3;
use ::ray::Ray;
use ::hitable::Hitable;
use ::hitlist::HitList;
use std::f64;

pub fn rand_in_unit_sphere() -> Vec3 {
    let mut t = rand::thread_rng();    
    let mut p = Vec3::new(1.0, 1.0, 1.0);
    let ones = Vec3::new(1.0, 1.0, 1.0);

    while p.dot(&p) >= 1.0 {
        p = Vec3::new(t.next_f64(), t.next_f64(), t.next_f64()).s_mult(2.0).v_sub(&ones); 
    }
    p
}

pub fn color(r: &Ray, world: &HitList) -> Vec3 {

    let t = world.hit(r, 0.0, f64::MAX);

    match t {
        Some(hr) => {
            let target = rand_in_unit_sphere().v_add(&hr.p).v_add(&hr.normal);
            let new_r = Ray::new_v(&hr.p, &(target.v_sub(&hr.p)));
            color( &new_r, world) * 0.5 
        }

        None => {
            let unit = r.dir.get_unit();
            let t = 0.5 * (unit[1] + 1.0);
            let v1 = Vec3::new(1.0, 1.0, 1.0);
            let v2 = Vec3::new(0.5, 0.7, 1.0);

            v1.s_mult(1.0 - t).v_add(&v2.s_mult(t))
        }
    }
}

