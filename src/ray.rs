use ::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(o: (f64, f64, f64), d: (f64, f64, f64)) -> Ray {
        Ray{ origin: Vec3::new(o.0, o.1, o.2),
             dir: Vec3::new(d.0, d.1, d.2) }
    }

    pub fn new_v(s: &Vec3, end: &Vec3) -> Ray {
        let o = Vec3::new(s[0], s[1], s[2]);
        let e = Vec3::new(end[0], end[1], end[2]);
        Ray{ origin: o,
             dir: e }
    }

    pub fn pt_at_t(self, t: f64) -> Vec3 {
        self.origin + (self.dir * t)    
    }

    pub fn color(&self) -> Vec3 {
        let unit = self.dir.get_unit();
        let t = 0.5 * (unit[1] + 1.0);
        let v1 = Vec3::new(1.0, 1.0, 1.0);
        let v2 = Vec3::new(0.5, 0.7, 1.0);

        v1.s_mult(1.0 - t) + v2.s_mult(t)
    }
}

