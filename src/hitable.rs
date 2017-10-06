use ::ray::Ray;
use ::vec3::Vec3;

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
}

impl Hitable for Vec<Box<Hitable>> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit = None;
        let mut closest_so_far = t_max;

        for obj in self{
            if let Some(hr) = obj.hit(r, t_min, closest_so_far){
                closest_so_far = hr.t;
                hit = Some(hr)
            }
        }

        hit

    }
}
