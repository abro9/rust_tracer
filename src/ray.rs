use ::vec3::Vec3;


pub struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(o: (f64, f64, f64), d: (f64, f64, f64)) -> Ray {
        Ray{ origin: Vec3::new(o.0, o.1, o.2),
             dir: Vec3::new(d.0, d.1, d.2) }
    }

    pub fn pt_at_t(self, t: f64) -> Vec3 {
        self.origin + (self.dir * t)    
    }

}

