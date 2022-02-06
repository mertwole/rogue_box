use crate::game::common::math::Vec2;

#[derive(Clone)]
pub struct CollisionData {
    // Normal facing from other body to self.
    pub normal : Vec2,
    pub depth : f32
}

impl CollisionData {
    pub fn reverse(&mut self) {
        self.normal = self.normal * -1.0;
    } 
}