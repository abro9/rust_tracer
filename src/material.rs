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
    pub diffuse: Vec3,
    pub specular: Vec3,
    pub phong: i32,
    pub ideal_spec: Vec3,
    pub mat_type: MatType,
}

impl Material {
    pub fn new(t: char,
               d0: f64, d1: f64, d2: f64,
               s0: f64, s1: f64, s2: f64,
               i0: f64, i1: f64, i2: f64,
               p: i32) -> Material {
        if t == 'l' {
            Material { diffuse : Vec3::new(d0, d1, d2),
                       specular : Vec3::new(s0, s1, s2),
                       phong : p,
                       ideal_spec : Vec3::new(i0, i1, i2),
                       mat_type : MatType::Lambertian } }
        else {
            Material { diffuse : Vec3::new(d0, d1, d2),
                       specular : Vec3::new(s0, s1, s2),
                       phong : p,
                       ideal_spec : Vec3::new(i0, i1, i2),
                       mat_type : MatType::Metal } }
    }

    pub fn scatter(&self, r_in: &Ray, hr: &HitRecord) -> MatRecord {
        match self.mat_type {
            MatType::Lambertian => {        
                let target = rand_in_unit_sphere() + hr.p + hr.normal;
                let new_r = Ray::new_v(hr.p, target - hr.p);
                MatRecord{ attenuation: self.diffuse,
                           scattered: new_r,
                           correct_dir: true }
            }

            MatType::Metal => {
                let reflected = reflect(r_in.dir, hr.normal);
                let new_r = Ray::new_v(hr.p, reflected);
                MatRecord{ attenuation: self.diffuse,
                           scattered: new_r,
                           correct_dir: hr.normal.dot(&reflected) > 0.0 }
            }
        }
    }
}

impl Clone for Material {
    fn clone(&self) -> Material {
        let d = (self.diffuse[0], self.diffuse[1], self.diffuse[2]);
        let s = (self.specular[0], self.specular[1], self.specular[2]);
        let i = (self.ideal_spec[0], self.ideal_spec[1], self.ideal_spec[2]);
        match self.mat_type {
            MatType::Lambertian => Material::new('l', d.0, d.1, d.2, s.0, s.1, s.2, i.0, i.1, i.2, self.phong),
            MatType::Metal => Material::new('m', d.0, d.1, d.2, s.0, s.1, s.2, i.0, i.1, i.2, self.phong),
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
