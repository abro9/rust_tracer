
use ::vec3::Vec3;
use ::ray::Ray;

pub struct Camera2 {
    pub origin: Vec3,
    pub dir: Vec3,
    pub f_length: f64,
    pub iw: f64,
    pub ih: f64,
    pub pw: usize,
    pub ph: usize,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl Camera2 {
    pub fn new_v(o: Vec3, dir: Vec3, d: f64, iw: f64, ih: f64, pw: usize, ph: usize) -> Camera2 {
        let _w = (dir * -1.0).get_unit();
        let mut _u = Vec3::new(0.0, 0.0, 0.0);
        let mut _v = Vec3::new(0.0, 0.0, 0.0);

        if 1.0 - f64::abs(_w[1]) < 0.0001 {
            _u = Vec3::new(1.0, 0.0, 0.0);
            _v = _u.cross(&dir);
        }
        else {
            let up = Vec3::new(0.0, 1.0, 0.0);
            _u = dir.cross(&up);
            _v = _u.cross(&dir);
        }

        Camera2 { origin : o,
                  dir : dir,
                  f_length : d,
                  iw : iw,
                  ih : ih,
                  pw : pw,
                  ph : pw,
                  u : _u.get_unit(),
                  v : _v.get_unit(),
                  w: _w,
        }
    }
}

