use ::vec3::Vec3;
use ::ray::Ray;
use ::hitable::Hitable;
use ::hitlist::HitList;
use std::f64;

pub fn color(r: &Ray, world: &HitList) -> Vec3 {

    let t = world.hit(r, 0.0, f64::MAX);

    match t {
        Some(hr) => {
            let n = hr.normal;
            Vec3::new(n[0] + 1.0, n[1] + 1.0, n[2] + 1.0).s_mult(0.5)
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

