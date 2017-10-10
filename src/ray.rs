use ::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(o: (f64, f64, f64), d: (f64, f64, f64)) -> Ray {
        Ray{ origin: Vec3::new(o.0, o.1, o.2),
             dir: Vec3::new(d.0, d.1, d.2).get_unit() }
    }

    pub fn new_v(s: Vec3, e: Vec3) -> Ray {
        Ray{ origin: s,
             dir: e.get_unit() }
    }

    pub fn pt_at_t(&self, t: f64) -> Vec3 {
        self.origin + self.dir * t
        //self.origin.v_add(&self.dir.s_mult(t))    
    }
}

