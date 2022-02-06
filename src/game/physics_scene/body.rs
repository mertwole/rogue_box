use crate::game::common::math::Vec2;
use super::collider::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct BodyId(pub u64);

static mut LAST_BODY_ID : u64 = 0;

impl BodyId {
    fn next() -> BodyId {
        unsafe { LAST_BODY_ID += 1; }
        BodyId(unsafe { LAST_BODY_ID })
    }
}

pub enum BodyType {
    Static,
    Dynamic
}

pub struct Body {
    pub(in super) id : BodyId,
    pub(in super) body_type : BodyType,
    pub(in super) position : Vec2,
    pub(in super) collider_initial_position : Vec2,
    pub(in super) collider : Collider,
    pub(in super) mass : f32,
    pub(in super) inv_mass : f32,
    pub(in super) bouncity : f32,

    pub(in super) velocity : Vec2,
    pub(in super) force : Vec2
}

impl Body {
    pub fn new_dynamic(collider : Collider, mass : f32, position : Vec2) -> Body {
        let collider_initial_position = collider.position;
        Body {
            id : BodyId::next(),
            body_type : BodyType::Dynamic,
            position,
            collider_initial_position,
            collider, 
            mass,
            inv_mass : 1.0 / mass,
            bouncity : 0.0,
            velocity : Vec2::zero(),
            force : Vec2::zero()
        }
    }

    pub fn new_static(collider : Collider, position : Vec2) -> Body {
        let collider_initial_position = collider.position;
        Body {
            id : BodyId::next(),
            body_type : BodyType::Static,
            position : position,
            collider_initial_position,
            collider,
            mass : 0.0,
            inv_mass : 0.0,
            bouncity : 0.0,
            velocity : Vec2::zero(),
            force : Vec2::zero()
        }
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    } 

    pub fn set_position(&mut self, position : Vec2) {
        self.position = position;
    }
}