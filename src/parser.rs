use rand::Rng;
use std::f64;
use std::str::FromStr;
use std::fs::File;
use std::io::Write;
use std::io::BufReader;
use std::io;
use std::io::prelude::*;

use ::vec3::Vec3;
use ::sphere::Sphere;
use ::hitlist::HitList;
use ::cam2::Camera2;
use ::camera::Camera;
use ::material::Material;
use ::light::Light;
use ::plane::Plane;

pub fn parse(source_file: &String) -> (HitList, Vec<Light>, Camera2){
    let f = File::open(source_file).unwrap();
    let file = BufReader::new(f);

    let mut world = HitList::new();
    let mut lights : Vec<Light> = Vec::new();
    let mut materials : Vec<Material> = Vec::new();

    let c_ori = Vec3::new(0.0, 0.0, 0.0);
    let c_dir = Vec3::new(0.0, 0.0, -1.0);
    let mut cam : Camera2 = Camera2::new_v(c_ori, c_dir, 1.0, 4., 2., 1200, 600);

    let default_material = Material::new('l', 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.0, 0.0, 0.0, 100);
    materials.push(default_material);
    let mut mat_counter = 0;

    for line in file.lines() {
        let line = line.unwrap();
        let split = line.split(" ");
        let line_vec = split.collect::<Vec<&str>>();
        match line_vec[0] {
            "m" => {
                let m = Material::new('l',
                                      f64::from_str(line_vec[1]).unwrap(),
                                      f64::from_str(line_vec[2]).unwrap(),
                                      f64::from_str(line_vec[3]).unwrap(),
                                      f64::from_str(line_vec[4]).unwrap(),
                                      f64::from_str(line_vec[5]).unwrap(),
                                      f64::from_str(line_vec[6]).unwrap(),
                                      f64::from_str(line_vec[8]).unwrap(),
                                      f64::from_str(line_vec[9]).unwrap(),
                                      f64::from_str(line_vec[10]).unwrap(),
                                      f64::from_str(line_vec[7]).unwrap() as i32);
                materials.push(m);
                mat_counter += 1;
            }
            "l" => {
                let l = Light::new('p',
                                   f64::from_str(line_vec[2]).unwrap(),  
                                   f64::from_str(line_vec[3]).unwrap(),  
                                   f64::from_str(line_vec[4]).unwrap(),  
                                   f64::from_str(line_vec[5]).unwrap(),  
                                   f64::from_str(line_vec[6]).unwrap(),  
                                   f64::from_str(line_vec[7]).unwrap());  
                lights.push(l);
            }
            "s" => {
                let s = Sphere::new(f64::from_str(line_vec[1]).unwrap(),
                                    f64::from_str(line_vec[2]).unwrap(),  
                                    f64::from_str(line_vec[3]).unwrap(),  
                                    f64::from_str(line_vec[4]).unwrap(),
                                    &materials[mat_counter]);
                world.add_sphere(s);    
            }
            "p" => {
                let p = Plane::new(f64::from_str(line_vec[1]).unwrap(),
                                   f64::from_str(line_vec[2]).unwrap(),  
                                   f64::from_str(line_vec[3]).unwrap(),  
                                   f64::from_str(line_vec[4]).unwrap(),
                                   &materials[mat_counter]);
                world.add_plane(p);
            }
            "c" => {
                let origin = Vec3::new(f64::from_str(line_vec[1]).unwrap(),
                                       f64::from_str(line_vec[2]).unwrap(),
                                       f64::from_str(line_vec[3]).unwrap());
                let cam_dir = Vec3::new(f64::from_str(line_vec[4]).unwrap(),
                                        f64::from_str(line_vec[5]).unwrap(),
                                        f64::from_str(line_vec[6]).unwrap());
                cam = Camera2::new_v(origin,
                                         cam_dir,
                                         f64::from_str(line_vec[7]).unwrap(),
                                         f64::from_str(line_vec[8]).unwrap(),
                                         f64::from_str(line_vec[9]).unwrap(),
                                         usize::from_str(line_vec[10]).unwrap(),
                                         usize::from_str(line_vec[11]).unwrap());
            }
            _ => ()//println!("not sick"),
        }
    }
    (world, lights, cam)
}
