use rand::Rng;
use std::f64;
use std::fs::File;
use std::io::Write;

use rust_tracer::vec3::Vec3;
use rust_tracer::sphere::Sphere;
use rust_tracer::hitlist::HitList;
//use rust_tracer::camera::Camera;
use rust_tracer::material::Material;
use rust_tracer::light::Light;
use rust_tracer::plane::Plane;

fn parse(source_file: &String) -> (Hitlist, Vec<Light> ){
    let mut file = File::open(source_file).expect("file now found");
    file.write_fmt(format_args!("P3\n{} {}\n{}\n", NX, NY, 255)).unwrap();

    let mut pb = ProgressBar::new(data.len() as u64);

    for rgb in data.iter() {
        file.write_fmt(format_args!("{} {} {}\n", rgb.0, rgb.1, rgb.2)).unwrap();
        pb.inc();
    }
    pb.finish_print("\nimage saved!");
}
