use ::vec3::Vec3;

pub struct Light{
    pub ltype : char,
    pub location: Vec3,
    pub color: Vec3,
    //pub intensity: f64,
}

impl Light {
    //pub fn new(t: char, loc: (f64, f64, f64), c: (f64, f64, f64), i: f64) -> Light {
    pub fn new(t: char, loc_x: f64, loc_y: f64, loc_z: f64, c_r: f64, c_g: f64, c_b: f64) -> Light {
        Light{ ltype : t,
               location : Vec3::new(loc_x, loc_y, loc_z),
               color : Vec3::new(c_r, c_g, c_b) }
               //intensity : i }
    }

    pub fn new_v(t: char, loc: Vec3, c: Vec3) -> Light {
        Light{ ltype : t,
               location : loc,
               color : c } 
               //intensity : i}
    }
}

