use std::f32;
use std::ops::Add;
use std::ops::Index;

fn main() {
    let nx = 2000;
    let ny = 1000;
    
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let vec1 = Vec3::new( 1.0, 2.0, 3.0 );
    let vec2 = Vec3::new( 2.0, 3.0, 4.0 );

    let d = vec1.dot(&vec2);
    let l = vec1.length();

    let vec3 = vec1 + vec2;

    println!("dot: {}, length: {}", d, l);
    println!("e0: {}, e1: {}", vec3[0], vec3[1]);

    for j in (0..ny).rev(){
        for i in 0..nx{
            let r = (i as f32) / (nx as f32);
            let g = (j as f32) / (ny as f32);
            let b = 0.35;

            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;

            //println!("{} {} {}", ir, ig, ib)
        }    
    }
}

struct Vec3{
    e : [f32; 3],
}

impl Vec3{
    fn new(e0: f32, e1: f32, e2: f32) -> Vec3{
        Vec3{ e: [e0, e1, e2] }
    }
}

impl Add for Vec3{
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3{
        Vec3::new(self.e[0] + other.e[0],
                  self.e[1] + other.e[1],
                  self.e[2] + other.e[2])
    } 
}

impl Index<usize> for Vec3{
    type Output = f32;

    fn index(&self, i: usize) -> &f32{
        &self.e[i]
    } 
}

impl Vec3{
    fn dot(&self, other: &Vec3) -> f32{
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }
}

impl Vec3{
    fn length(&self) -> f32{
        (self.e[0] *self.e[0] + self.e[1] * self.e[1] + self.e[2] *self.e[2]).sqrt()
    }
}
