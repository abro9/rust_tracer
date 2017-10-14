use std::f64;
use std::ops::*;

#[derive(Copy)]
pub struct Vec3 {
    e : [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3{ e: [e0, e1, e2] }
    }

    pub fn s_mult(&self, scalar: f64) -> Vec3 {
        Vec3::new(self[0] * scalar, self[1] * scalar, self[2] * scalar)
    }
    
    pub fn v_add(&self, op: &Vec3) -> Vec3 {
        Vec3::new(self[0] + op[0],
                  self[1] + op[1],
                  self[2] + op[2])
    }

    pub fn v_sub(&self, op: &Vec3) -> Vec3 {
        Vec3::new(self[0] - op[0],
                  self[1] - op[1],
                  self[2] - op[2])
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        let x_val = self[1] * other[2] - self[2] * other[1];
        let y_val = self[2] * other[0] - self[0] * other[2];
        let z_val = self[0] * other[1] - self[1] * other[0];
        Vec3::new(x_val, y_val, z_val)
    }

    pub fn squared_length(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn get_unit(&self) -> Vec3 {
        let l = self.length();
        Vec3::new(self.e[0] / l,
                  self.e[1] / l,
                  self.e[2] / l)
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Vec3 {
        Vec3::new(self[0], self[1], self[2])
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.e[0] + other.e[0],
                  self.e[1] + other.e[1],
                  self.e[2] + other.e[2])
    } 
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.e[0] - other.e[0],
                  self.e[1] - other.e[1],
                  self.e[2] - other.e[2])
    } 
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other:Vec3) -> Vec3 {
        Vec3::new(self[0] * other[0], self[1] * other[1], self[2] * other[2])
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, other:f64) -> Vec3 {
        Vec3::new(self.e[0] * other,
                  self.e[1] * other,
                  self.e[2] * other)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other:Vec3) -> Vec3 {
        Vec3::new(self * other[0],
                  self * other[1],
                  self * other[2])
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f64) -> Vec3 {
        Vec3::new(self.e[0] / other,
                  self.e[1] / other,
                  self.e[2] / other)
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        &self.e[i]
    }
}
