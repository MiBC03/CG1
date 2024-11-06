use crate::Vec3;
pub struct FonteLuz {
    pub posicao: Vec3,
    pub cor: Vec3,
    pub intensidade: f32
}

impl FonteLuz {
    pub fn new (posicao: Vec3, cor: Vec3, intensidade: f32) -> FonteLuz{
        
        return FonteLuz{
            posicao: posicao,
            cor: cor,
            intensidade: intensidade
        }

    }
}