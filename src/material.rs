use ::ray::Ray;
use ::vec3::Vec3;
use ::hitable::HitRecord;
use ::color::rand_in_unit_sphere;

pub trait Material {
    fn scatter(&self, r_in: &Ray, hr: &HitRecord) -> MatRecord;
}

pub struct MatRecord {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, hr: &HitRecord) -> MatRecord {
        let target = rand_in_unit_sphere().v_add(&hr.p).v_add(&hr.normal);
        let new_r = Ray::new_v(&hr.p, &(target.v_sub(&hr.p)));
        MatRecord{ attenuation: self.albedo.clone(),
                   scattered: new_r}
    }
}
