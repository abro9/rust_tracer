extern crate rand;

use rand::Rng;

use ::ray::Ray;
use ::vec3::Vec3;
use ::hitable::HitRecord;

pub struct MatRecord {
    pub attenuation: Vec3,
    pub scattered: Ray,
    pub correct_dir: bool,
}

pub enum MatType {
    Lambertian,
    Metal,
}

pub struct Material{
    pub albedo: Vec3,
    pub mat_type: MatType,
}

impl Material {
    pub fn new(t: char, a0: f64, a1: f64, a2: f64) -> Material {
        if t == 'l' {
            Material { albedo : Vec3::new(a0, a1, a2),
                       mat_type : MatType::Lambertian } }
        else {
            Material { albedo : Vec3::new(a0, a1, a2),
                       mat_type : MatType::Metal } }
    }

    pub fn scatter(&self, r_in: &Ray, hr: &HitRecord) -> MatRecord {
        match self.mat_type {
            MatType::Lambertian => {        
                let target = rand_in_unit_sphere() + hr.p + hr.normal;
                let new_r = Ray::new_v(hr.p, (target - hr.p).get_unit());
                MatRecord{ attenuation: self.albedo,
                           scattered: new_r,
                           correct_dir: true }
            }

            MatType::Metal => {
                let reflected = reflect(r_in.dir, hr.normal);
                let new_r = Ray::new_v(hr.p, reflected);
                MatRecord{ attenuation: self.albedo,
                           scattered: new_r,
                           //correct_dir: true }
                           correct_dir: hr.normal.dot(&reflected) >= 0.0 }
            }
        }
    }
}

impl Clone for Material {
    fn clone(&self) -> Material {
        let a = (self.albedo[0], self.albedo[1], self.albedo[2]);
        match self.mat_type {
            MatType::Lambertian => Material::new('l', a.0, a.1, a.2),
            MatType::Metal => Material::new('m', a.0, a.1, a.2),
        }
    }
}

//pub fn reflect( v: &Vec3, n: &Vec3) -> Vec3 {
//    v.v_sub(&(&n.s_mult(2.0)).s_mult(v.dot(n)))
//}

pub fn reflect( v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(&n) * n
}

pub fn rand_in_unit_sphere() -> Vec3 {
    let mut t = rand::thread_rng();
    let mut p = Vec3::new(1.0, 1.0, 1.0);
    let ones = Vec3::new(1.0, 1.0, 1.0);

    while p.dot(&p) > 1.0 {
        p = Vec3::new(t.next_f64(), t.next_f64(), t.next_f64()).s_mult(2.0).v_sub(&ones) * Vec3::new(1.0, 1.0, 1.0);
    }
    p
}
