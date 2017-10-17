extern crate rand;
extern crate pbr;

use pbr::ProgressBar;

use std::fs::File;
use std::io::Write;
use rand::Rng;

use ::color;
use ::vec3::Vec3;
use ::ray::Ray;
use ::hitlist::HitList;
use ::light::Light;

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
                  ph : ph,
                  u : _u.get_unit(),
                  v : _v.get_unit(),
                  w: _w,
        }
    }

    // [i=0, j=0] -> bottom left
    pub fn generate_ray(&self, i: f64, j: f64) -> Ray {
        let mut t_rng = rand::thread_rng();
        let r = 0.5 * self.iw;
        let l = -0.5 * self.iw;

        let t = 0.5 * self.ih;
        let b = -0.5 * self.ih;

        let u = l + (r - l) * (i + t_rng.next_f64()) / self.pw as f64;
        let v = b + (t - b) * (j + t_rng.next_f64()) / self.ph as f64;

        let dir = self.w * -self.f_length + self.u * u + self.v * v;
        Ray::new_v(self.origin, dir)
    }

    pub fn render_scene(&self, world: &HitList, lights: &Vec<Light>, rpp: usize, image_file: &String){
        let rpp_f = rpp as f64;
        let mut pb = ProgressBar::new(self.ph as u64);
        let mut rgb_data: Vec<(i32, i32, i32)> = Vec::with_capacity(self.pw * self.ph);
        for j in (0..self.ph).rev() {
            pb.inc();
            for i in 0..self.pw {
                let mut rgb: (f64, f64, f64) = (0.0, 0.0, 0.0);
                for _ in 0..rpp {
                    let ray = self.generate_ray(i as f64, j as f64);
                    
                    let c = color::new_color(&ray, world, lights, 0);
                    rgb.0 += c[0];
                    rgb.1 += c[1];
                    rgb.2 += c[2];
                }
                
                rgb.0 = (rgb.0 / rpp_f).sqrt() * 255.99;
                rgb.1 = (rgb.1 / rpp_f).sqrt() * 255.99;
                rgb.2 = (rgb.2 / rpp_f).sqrt() * 255.99;

                rgb.0 = if rgb.0 > 255.99 {255.99} else {rgb.0};
                rgb.1 = if rgb.1 > 255.99 {255.99} else {rgb.1};
                rgb.2 = if rgb.2 > 255.99 {255.99} else {rgb.2};

                let ir = rgb.0 as i32;
                let ig = rgb.1 as i32;
                let ib = rgb.2 as i32;

                rgb_data.push((ir, ig, ib));
            }
        }
        pb.finish_print("\ndone! writing ppm now.\n");
        self.save_file(image_file, &rgb_data);
    }

    pub fn save_file(&self, image_file: &String, data: &Vec<(i32, i32, i32)>){
        let mut file = File::create(image_file).unwrap();
        file.write_fmt(format_args!("P3\n{} {}\n{}\n", self.pw, self.ph, 255)).unwrap();

        let mut pb = ProgressBar::new(data.len() as u64);

        for rgb in data.iter() {
            file.write_fmt(format_args!("{} {} {}\n", rgb.0, rgb.1, rgb.2)).unwrap();
            pb.inc();
        }
        pb.finish_print("\nimage saved!");
    }
}

