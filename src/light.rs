use ::vec3::Vec3;

pub struct Light{
    pub ltype : char,
    pub location: Vec3,
    pub color: Vec3,
}

impl Light {
    pub fn new(t: char, loc: (f64, f64, f64), c: (f64, f64, f64)) -> Light {
        Light{ ltype : t,
               location: Vec3::new(loc.0, loc.1, loc.2),
               color: Vec3::new(c.0, c.1, c.2) }
    }

    pub fn new_v(t: char, loc: Vec3, c: Vec3) -> Light {
        Light{ ltype : t,
               location: loc,
               color: c }
    }
}

