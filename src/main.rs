extern crate rust_tracer;
extern crate rand;

use rand::Rng;

use rust_tracer::vec3::Vec3;
use rust_tracer::sphere::Sphere;
use rust_tracer::color;
use rust_tracer::hitlist::HitList;
use rust_tracer::camera::Camera;

fn main() {
    let nx: i32 = 2000;
    let ny: i32 = 1000;
    let ns: i32 = 100;
    
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let cam = Camera::new_v(&lower_left_corner, &horizontal, &vertical, &origin);

    let s = Sphere::new((0.0, 0.0, -1.0), 0.5);
    let s2 = Sphere::new((0.0, -100.5, -1.0), 100.0);

    let mut world = HitList::new();
    world.add_sphere(s);
    world.add_sphere(s2);

    for j in (0..ny).rev() {
        for i in 0..nx{
            let mut rgb: (f64, f64, f64) = (0.0 , 0.0, 0.0);
            for _ in 0..ns{        
                let u = ((i as f64) + rand::thread_rng().next_f64()) / (nx as f64);
                let v = (j as f64 + rand::thread_rng().next_f64()) / (ny as f64);

                let ray = cam.get_ray(u, v);

                let c = color::color(&ray, &world) * 255.99;
                rgb.0 += c[0];
                rgb.1 += c[1];
                rgb.2 += c[2];
            }

            let irgb = Vec3::new(rgb.0, rgb.1, rgb.2) / (ns as f64);            

            let ir = irgb[0] as i32;
            let ig = irgb[1] as i32;
            let ib = irgb[2] as i32;

            println!("{} {} {}", ir, ig, ib)
        }    
    }
}
