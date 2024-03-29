use super::collider::*;
use crate::game::common::math::Vec2;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct BodyId(pub u64);

static mut LAST_BODY_ID: u64 = 0;

impl BodyId {
    fn next() -> BodyId {
        unsafe {
            LAST_BODY_ID += 1;
        }
        BodyId(unsafe { LAST_BODY_ID })
    }
}

#[derive(PartialEq, Eq)]
pub enum BodyType {
    Static,
    Kinematic,
    Dynamic,
}

pub struct Body {
    pub id: BodyId,
    pub(super) body_type: BodyType,
    pub(super) position: Vec2,
    pub(super) collider_initial_position: Vec2,
    pub(super) collider: Collider,
    pub(super) mass: f32,
    pub(super) inv_mass: f32,
    pub(super) bouncity: f32,
    pub(super) ground_friction: f32,

    pub(super) velocity: Vec2,
    pub(super) force: Vec2,
}

impl Body {
    pub fn new_dynamic(collider: Collider, mass: f32, position: Vec2, friction: f32) -> Body {
        let collider_initial_position = collider.position;
        Body {
            id: BodyId::next(),
            body_type: BodyType::Dynamic,
            position,
            collider_initial_position,
            collider,
            mass,
            inv_mass: 1.0 / mass,
            bouncity: 0.0,
            ground_friction: friction,
            velocity: Vec2::zero(),
            force: Vec2::zero(),
        }
    }

    pub fn new_kinematic(collider: Collider, mass: f32, position: Vec2) -> Body {
        let collider_initial_position = collider.position;
        Body {
            id: BodyId::next(),
            body_type: BodyType::Kinematic,
            position,
            collider_initial_position,
            collider,
            mass,
            inv_mass: 1.0 / mass,
            bouncity: 0.0,
            ground_friction: 0.0,
            velocity: Vec2::zero(),
            force: Vec2::zero(),
        }
    }

    pub fn new_static(collider: Collider, position: Vec2) -> Body {
        let collider_initial_position = collider.position;
        Body {
            id: BodyId::next(),
            body_type: BodyType::Static,
            position,
            collider_initial_position,
            collider,
            mass: 0.0,
            inv_mass: 0.0,
            bouncity: 0.0,
            ground_friction: 0.0,
            velocity: Vec2::zero(),
            force: Vec2::zero(),
        }
    }

    pub fn is_kinematic(&self) -> bool {
        self.body_type == BodyType::Kinematic
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    pub fn set_position_kinematic(&mut self, position: Vec2, delta_time: f32) {
        self.velocity = (position - self.position) / delta_time;
        self.position = position;
    }
}
