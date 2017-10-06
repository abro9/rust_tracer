extern crate rust_tracer;

use rust_tracer::vec3::Vec3;
use rust_tracer::ray::Ray;
use rust_tracer::sphere::Sphere;
use rust_tracer::color;
use rust_tracer::hitlist::HitList;

fn main() {
    let nx: i32 = 2000;
    let ny: i32 = 1000;
    
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let s = Sphere::new((0.0, 0.0, -1.0), 0.5);
    let s2 = Sphere::new((0.0, -100.5, -1.0), 100.0);

    let mut world = HitList::new();
    world.add_sphere(s);
    world.add_sphere(s2);

    for j in (0..ny).rev() {
        for i in 0..nx{
        
            let u = (i as f64) / (nx as f64);
            let v = (j as f64) / (ny as f64);

            let horiz_u = horizontal.s_mult(u);
            let vert_v = vertical.s_mult(v);
            let dir = lower_left_corner.v_add(&(horiz_u + vert_v));

            let r = Ray::new_v(&origin, &dir);

            let irgb = color::color(&r, &world) * 255.99;

            let ir = irgb[0] as i32;
            let ig = irgb[1] as i32;
            let ib = irgb[2] as i32;

            println!("{} {} {}", ir, ig, ib)
        }    
    }
}
