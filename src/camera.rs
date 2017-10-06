
use ::vec3::Vec3;
use ::ray::Ray;

pub struct Camera {
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
}

impl Camera{
    pub fn new_v(llc: &Vec3, h: &Vec3, v:&Vec3, o: &Vec3) -> Camera {
        Camera{ lower_left_corner: llc.clone(),
                horizontal: h.clone(),
                vertical: v.clone(),
                origin: o.clone(),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let horiz_u = self.horizontal.s_mult(u);
        let vert_v = self.vertical.s_mult(v);
        let dir = self.lower_left_corner.v_add(&(horiz_u + vert_v));

        Ray::new_v(&self.origin, &dir)
    }
}

