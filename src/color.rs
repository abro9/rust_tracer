use ::vec3::Vec3;
use ::ray::Ray;
use ::sphere::Sphere;


pub fn color(r: &Ray, s: &Sphere) -> Vec3 {
    if s.hit(r) {
        Vec3::new(1.0, 0.0, 0.0)
    }
    else {
        let unit = r.dir.get_unit();
        let t = 0.5 * (unit[1] + 1.0);
        let v1 = Vec3::new(1.0, 1.0, 1.0);
        let v2 = Vec3::new(0.5, 0.7, 1.0);

        v1.s_mult(1.0 - t) + v2.s_mult(t)
    }
}

