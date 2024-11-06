use crate::Vec3;

pub struct Esfera {
    pub c: Vec3,
    pub r: f32,
    pub k_e: f32,
    pub k_amb: Vec3,
    pub k_dif: Vec3,
    pub k_esp: Vec3
}

impl Esfera {

    pub fn new(c: Vec3, r: f32, k_e: f32, k_amb: Vec3, k_dif: Vec3, k_esp: Vec3) -> Esfera {

        return Esfera {
            c: c,
            r: r,
            k_e: k_e,
            k_amb: k_amb,
            k_dif: k_dif,
            k_esp: k_esp
        }
    }
}

