use crate::game::common::math::Vec2;

#[derive(Clone)]
pub struct SpriteTransform {
    pub translation : Vec2,
    pub rotation : f32,
    pub scale : Vec2
}

impl SpriteTransform {
    pub fn default() -> SpriteTransform {
        SpriteTransform {
            translation : Vec2::zero(),
            rotation : 0.0,
            scale : Vec2::new_xy(1.0)
        }
    }

    pub fn add_translation(mut self, translation : Vec2) -> SpriteTransform {
        self.translation = self.translation + translation;
        self
    }

    pub fn add_rotation(mut self, rotation : f32) -> SpriteTransform {
        self.rotation += rotation;
        self
    }

    pub fn add_scale(mut self, scale : Vec2) -> SpriteTransform {
        self.scale = self.scale * scale;
        self
    }

    pub fn combine(&self, other : &SpriteTransform) -> SpriteTransform {
        SpriteTransform {
            translation : self.translation + other.translation,
            rotation : self.rotation + other.rotation,
            scale : self.scale * other.scale
        }
    }
}