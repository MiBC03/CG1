use std::ops::{Add, Mul, Div, Sub, Neg};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        return Vec3 {
            x:x,
            y:y,
            z:z
        }
    }

    pub fn clamp(self, min: f32, max: f32) -> Vec3 {
        return Vec3 {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
            z: self.z.clamp(min, max)
        };
    }

    pub fn dot(self, v2: Vec3) -> f32{
        return self.x * v2.x + self.y * v2.y + self.z * v2.z
        
    }

    pub fn norm(self) -> f32{
        let norm = self.dot(self);
        return norm.sqrt();
    }

    pub fn normalize(self) -> Vec3 {
        return self/(self.norm());
    }

}

impl Add<Vec3> for Vec3{
    type Output = Vec3;

    fn add(self, v2: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x + v2.x,
            y: self.y + v2.y,
            z: self.z + v2.z
        }
    }
}

impl Sub<Vec3> for Vec3{
    type Output = Vec3;

    fn sub(self, v2: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x - v2.x,
            y: self.y - v2.y,
            z: self.z - v2.z
        }
    }
}

impl Mul<f32> for Vec3{
    type Output = Vec3;

    fn mul(self, f: f32) -> Vec3 {
        return Vec3 {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f
        }
    }
}

impl Mul<Vec3> for f32{
    type Output = Vec3;

    fn mul(self, v1: Vec3) -> Vec3 {
        return Vec3 {
            x: v1.x * self,
            y: v1.y * self,
            z: v1.z * self
        }
    }
}

impl Mul<Vec3> for Vec3{
    type Output = Vec3;

    fn mul(self, v1: Vec3) -> Vec3 {
        return Vec3 {
            x: v1.x * self.x,
            y: v1.y * self.y,
            z: v1.z * self.z
        }
    }
}

impl Div<f32> for Vec3{
    type Output = Vec3;

    fn div(self, f: f32) -> Vec3 {
        return Vec3 {
            x: self.x / f,
            y: self.y / f,
            z: self.z / f
        }
    }
}

impl Div<Vec3> for f32{
    type Output = Vec3;

    fn div(self, v1: Vec3) -> Vec3 {
        return Vec3 {
            x: v1.x / self,
            y: v1.y / self,
            z: v1.z / self
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        return Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        };
    }
}
