
use ::vec3::Vec3;
use ::ray::Ray;
use ::light::Light;
use ::hitable::Hitable;
use ::hitlist::HitList;
use std::f64;

pub fn new_color(r: &Ray, world: &HitList, lights: &Vec<Light>) -> Vec3 {
    
    let t = world.hit(r, 0.0, f64::MAX);
    
    match t {
        Some(hr) => {
            let light_v = lights[0].location - hr.p;
            let atten = 1.0 / light_v.squared_length();
            let light_v = light_v.get_unit();

            let n_dot_l = light_v.dot(&hr.normal);
            hr.mat.diffuse * atten * 0.0_f64.max( n_dot_l) 
        }

        None => {
            Vec3::new(0.0, 0.0, 0.0)
        }
    }
}

pub fn color(r: &Ray, world: &HitList, depth: u32) -> Vec3 {

    let t = world.hit(r, 0.0, f64::MAX);

    match t {
        Some(hr) => {
            let m = &hr.mat;
            let r_record = m.scatter(r, &hr);
            if (depth < 50) & r_record.correct_dir {
                color( &r_record.scattered, world, depth + 1) * r_record.attenuation
            }
            else {
                //r_record.attenuation
                Vec3::new(1.0, 1.0, 1.0)                
            }
        }

        None => {
            //Vec3::new(1.0, 1.0, 1.0)
            let unit = r.dir.get_unit();
            let t = 0.5 * (unit[1] + 1.0);
            let v1 = Vec3::new(1.0, 1.0, 1.0);
            let v2 = Vec3::new(0.5, 0.7, 1.0);

            v1.s_mult(1.0 - t).v_add(&v2.s_mult(t))
        }
    }
}

