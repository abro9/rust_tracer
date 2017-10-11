extern crate rust_tracer;
extern crate rand;
extern crate pbr;

use pbr::ProgressBar;
use rand::Rng;
use std::f64;
use std::fs::File;
use std::io::Write;

use rust_tracer::vec3::Vec3;
use rust_tracer::sphere::Sphere;
use rust_tracer::color;
use rust_tracer::hitlist::HitList;
use rust_tracer::camera::Camera;
use rust_tracer::material::Material;
use rust_tracer::light::Light;
use rust_tracer::plane::Plane;

const NX: usize = 1200;
const NY: usize = 600;
const RPP: usize = 50;

fn main() {
    let nx_f = NX as f64;
    let ny_f = NY as f64;
    let rpp_f = RPP as f64;
    
    let data_size: usize = NX * NY;
    let mut rgb_data: Vec<(i32,i32,i32)> = Vec::with_capacity(data_size);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let cam = Camera::new_v(lower_left_corner, horizontal, vertical, origin);

    let m = Material::new('l', 0.3, 0.3, 0.8, 0.5, 0.5, 0.5, 8000);
    let m2 = Material::new('m', 0.3, 0.8, 0.3, 0.5, 0.5, 0.5, 3000);
    let m3 = Material::new('l', 0.8, 0.3, 0.3, 0.5, 0.5, 0.5, 100);
    let m4 = Material::new('l', 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 10);
    let m5 = Material::new('l', 1.0, 1.0, 1.0, 0.5, 0.5, 0.5, 10);

    let s = Sphere::new((0.0, 0.0, -1.0), 0.5, &m);
    let s2 = Sphere::new((0.0, -100.5, -1.0), 100.0, &m4);
    let s3 = Sphere::new((2.0, -0.02488, -2.0), 0.5, &m3);
    let s4 = Sphere::new((-2.0, -0.02488, -2.0), 0.5, &m2);
    //let s3 = Sphere::new((1.0, 0.0, -1.0), 0.5, &m2);
    //let s4 = Sphere::new((-1.0, 0.0, -1.0), 0.5, &m2);

    let p = Plane::new((0.0, 0.0, -1.0), 1.0, &m5);

    let mut world = HitList::new();
    world.add_sphere(s);
    world.add_sphere(s2);
    world.add_sphere(s3);
    world.add_sphere(s4);
    world.add_plane(p);

    let l = Light::new('p', (-7.0, 3.5, 5.0), (1.0, 1.0, 1.0), 50.0);
    let mut lights : Vec<Light> = Vec::new();
    lights.push(l);


    let mut pb = ProgressBar::new(NY as u64);

    for j in (0..NY).rev() {
        pb.inc();
        for i in 0..NX{
            let mut rgb: (f64, f64, f64) = (0.0 , 0.0, 0.0);
            for _ in 0..RPP{        
                let u = ((i as f64) + rand::thread_rng().next_f64()) / nx_f;
                let v = (j as f64 + rand::thread_rng().next_f64()) / ny_f;

                let ray = (cam).get_ray(u, v);

                let c = color::new_color(&ray, &world, &lights);
                //let c = color::color(&ray, &world, 0);
                rgb.0 += c[0];
                rgb.1 += c[1];
                rgb.2 += c[2];
            }

            rgb.0 = (rgb.0 / rpp_f ).sqrt() * 255.99; 
            rgb.1 = (rgb.1 / rpp_f ).sqrt() * 255.99; 
            rgb.2 = (rgb.2 / rpp_f ).sqrt() * 255.99; 

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
    save_file(&rgb_data);
}

fn save_file(data: &Vec<(i32, i32, i32)>){
    let mut file = File::create("test8.ppm").unwrap();
    file.write_fmt(format_args!("P3\n{} {}\n{}\n", NX, NY, 255)).unwrap();

    let mut pb = ProgressBar::new(data.len() as u64);

    for rgb in data.iter() {
        file.write_fmt(format_args!("{} {} {}\n", rgb.0, rgb.1, rgb.2)).unwrap();
        pb.inc();
    }
    pb.finish_print("\nimage saved!");
}
