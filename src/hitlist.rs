use ::ray::Ray;
use ::sphere::Sphere;
use ::plane::Plane;
use ::hitable::{Hitable, HitRecord};

pub struct HitList {
    pub members: Vec<Box<Hitable>>,
}

impl HitList {
    pub fn new() -> HitList {
        let v: Vec<Box<Hitable>> = Vec::new();
        HitList { members: v }
    }

    pub fn add_sphere(&mut self, s: Sphere){
        let b = Box::new(s);
        self.members.push(b);
    }

    pub fn add_plane(&mut self, p: Plane){
        let b = Box::new(p);
        self.members.push(b);
    }
}

impl Hitable for HitList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>{
        let mut hit = None;
        let mut closest_so_far = t_max;

        for obj in self.members.iter(){
            if let Some(hr) = obj.hit(r, t_min, closest_so_far) {
                closest_so_far = hr.t;
                //if hr.mat.phong == 5 {println!("padpfja");};
                hit = Some(hr)
            }                            
        }

        hit

    }
}
