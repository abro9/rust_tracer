extern crate rust_tracer;

use std::env;
use rust_tracer::parser;

// RPP = Rays per pixel
const RPP: usize = 25;

fn main() {
    let args: Vec<String> = env::args().collect();
    let image_file = &args[1];
    let scene_file = &args[2];
    let out_type = &args[3];

    let (world, lights, cam) = parser::parse(scene_file);

    cam.render_scene(&world, &lights, RPP, image_file, out_type);
}

