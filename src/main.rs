extern crate rust_tracer;

use rust_tracer::vec3::Vec3;
use rust_tracer::ray::Ray;

fn main() {
    let nx: i32 = 2000;
    let ny: i32 = 1000;
    
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    for j in (0..ny).rev() {
        for i in 0..nx{
            
            let r = (i as f64) / (nx as f64);
            let g = (j as f64) / (ny as f64);
            let b = 0.35;
            
            let irgb = Vec3::new(r, g, b) * 255.99;

            let ir = irgb[0] as i32;
            let ig = irgb[1] as i32;
            let ib = irgb[2] as i32;

            println!("{} {} {}", ir, ig, ib)
        }    
    }
}
