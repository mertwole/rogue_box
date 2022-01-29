use std::collections::hash_map::HashMap;
use std::cell::RefCell;

use crate::game::common::math::{Vec2, Math};

mod body;
mod collider;

pub use body::Body;
pub use collider::{Collider, ColliderShape};

use body::*;
use collider::*;

pub struct PhysicsScene {
    static_bodies : HashMap<BodyId, Body>,
    dynamic_bodies : HashMap<BodyId, RefCell<Body>>,
    last_body_id : u64,

    positional_correction_percent : f32,
    positional_correction_slop : f32
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct BodyId(u64);

impl PhysicsScene {
    pub fn new() -> PhysicsScene {
        PhysicsScene {
            static_bodies : HashMap::new(),
            dynamic_bodies : HashMap::new(),
            last_body_id : 0,

            positional_correction_percent : 0.6,
            positional_correction_slop : 0.01
        }
    }

    pub fn add_body(&mut self, body : Body) -> BodyId {
        self.last_body_id += 1;
        let id = BodyId(self.last_body_id);
        match body.body_type {
            BodyType::Static => { 
                self.static_bodies.insert(id, body); 
            }
            BodyType::Dynamic => { 
                self.dynamic_bodies.insert(id, RefCell::new(body)); 
            }
        }
        id
    }

    pub fn remove_body(&mut self, id : BodyId) {
        if self.static_bodies.contains_key(&id) {
            self.static_bodies.remove(&id);
        } else if self.dynamic_bodies.contains_key(&id) {
            self.dynamic_bodies.remove(&id);
        } else {
            log::warn!("Trying to remove physics body that doesn't exist ({})", id.0);
        }
    }

    pub fn simulate(&mut self, delta_time : f32) {
        self.resolve_collisions();
        self.move_bodies(delta_time);
    }

    fn move_bodies(&mut self, delta_time : f32) {
        for body_id in self.dynamic_bodies.keys() {
            let body = &mut *self.dynamic_bodies[body_id].borrow_mut();
            
            body.velocity = body.velocity + body.force * delta_time;
            let new_pos = *body.position + body.velocity * delta_time;
            body.set_position(new_pos);
        }
    }   

    fn resolve_collision(a : &mut Body, b : &mut Body, pc_percent : f32, pc_slop : f32) {
        let collision_data = a.collider.collide(&b.collider);
        match collision_data {
            Some(data) => {
                let bouncity = Math::min(a.bouncity, b.bouncity);

                let relative_velocity = b.velocity - a.velocity;
                let velocity_along_normal = relative_velocity.dot(data.normal);

                if velocity_along_normal > 0.0 { return; }

                let inv_sum = a.inv_mass + b.inv_mass;
                let impulse = -1.0 * (1.0 + bouncity) * velocity_along_normal / inv_sum;
                let impulse_vec = data.normal * impulse;

                a.velocity = a.velocity - a.inv_mass * impulse_vec;
                b.velocity = b.velocity + b.inv_mass * impulse_vec;

                // Positional correction.
                let correction = (Math::max(data.depth - pc_slop, 0.0)
                / inv_sum) * pc_percent;
                let correction_vec = correction * data.normal;

                let new_a_pos = *a.position - a.inv_mass * correction_vec;
                let new_b_pos = *b.position + b.inv_mass * correction_vec;

                a.set_position(new_a_pos);
                b.set_position(new_b_pos);
            }
            None => {  }
        }
    }

    fn resolve_collisions(&mut self) {
        for a in self.dynamic_bodies.keys() {
            for b in self.dynamic_bodies.keys() {
                if a.0 <= b.0 { continue; }
                let a_body = &mut *self.dynamic_bodies[a].borrow_mut();
                let b_body = &mut *self.dynamic_bodies[b].borrow_mut();
                Self::resolve_collision(a_body, b_body, 
                    self.positional_correction_percent, self.positional_correction_slop);
            }
        }

        for a in self.dynamic_bodies.keys() {
            for b_body in self.static_bodies.values_mut() {
                let a_body = &mut *self.dynamic_bodies[a].borrow_mut();
                Self::resolve_collision(a_body, b_body, 
                    self.positional_correction_percent, self.positional_correction_slop);
            }
        }
    }
}