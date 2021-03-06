
use ::vec3::Vec3;
use ::ray::Ray;
use ::light::Light;
use ::hitable::Hitable;
use ::hitlist::HitList;
use ::material::reflect;
use std::f64;

pub fn new_color(r: &Ray, world: &HitList, lights: &Vec<Light>, depth: u32) -> Vec3 {

    let t;
    if depth == 0 {
        t = world.hit(r, 0.0, f64::MAX);
    }
    else {
        t = world.hit(r, 0.01, f64::MAX);
    }

    match t {
        Some(hr) => {
            let mut accum = (0.0, 0.0, 0.0);
            for l in lights.iter() {
                let light_v = l.location - hr.p;
                let light_max_t = light_v.length();
                let atten = 1.0 / light_v.squared_length();
                let light_v = light_v.get_unit();
                let h = (light_v + (-1.0 * r.dir)).get_unit();

                let shadow_ray = Ray::new_v(hr.p, light_v);
                let t = world.hit(&shadow_ray, 0.1, light_max_t);
                let shadow_vec = match t {
                    Some(_) => Vec3::new(0.0, 0.0, 0.0),
                    None => Vec3::new(1.0, 1.0, 1.0)
                };

                let n_dot_l = light_v.dot(&hr.normal);
                let n_dot_h = h.dot(&hr.normal);
                let diffuse = hr.mat.diffuse * atten * n_dot_l.max(0.0) * l.color * shadow_vec;
                let spec = hr.mat.specular * atten * n_dot_h.max(0.0).powi(hr.mat.phong) * l.color * shadow_vec;

                accum.0 += diffuse[0] + spec[0];
                accum.1 += diffuse[1] + spec[1];
                accum.2 += diffuse[2] + spec[2];
            }
            let accum_vec = Vec3::new(accum.0, accum.1, accum.2);

            if depth < 10 {
                let rfl_ray = Ray::new_v(hr.p, reflect(r.dir, hr.normal)); 
                accum_vec + hr.mat.ideal_spec * new_color(&rfl_ray, world, lights, depth + 1) 
            }
            else { Vec3::new(0.0, 0.0, 0.0) }
        }

        None => {
            Vec3::new(0.0, 0.0, 0.0)
        }
    }
}
