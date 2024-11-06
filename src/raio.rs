use std::f32::INFINITY;

use crate::{esfera::Esfera, Vec3};
pub struct Raio {
    pub p0: Vec3,
    pub dr: Vec3
}

impl Raio {
    pub fn new (p0: Vec3, dr: Vec3) -> Raio {
        return Raio {
            p0: p0,
            dr: dr
        };
    }

    pub fn deteccao (&self, esfera: &Esfera) -> (bool, f32, f32) {
        let v = esfera.c - self.p0;
        let a = self.dr.dot(self.dr);
        let b = - 2.0 * self.dr.dot(v);
        let c = v.dot(v) - (esfera.r * esfera.r);
        let delta = b*b - 4.0 * a * c;

        if delta > 0.0 {
            let t1: f32 = (-b + delta.sqrt()) / (2.0*a);
            let t2: f32 = (-b - delta.sqrt()) / (2.0*a);
            return (true, t1, t2);
        } else {
            return (false, -INFINITY, -INFINITY);
        } 
    }

    pub fn em_t (&self, t: f32) -> Vec3 {
        return self.p0 + t * self.dr;
    }
}