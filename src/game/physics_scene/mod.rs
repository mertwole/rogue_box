use crate::game::common::math::Math;

mod body;
mod collider;
pub mod collision_data;
pub mod message;
mod physics_simulated;

pub use body::Body;
pub use collider::{Collider, ColliderShape};
pub use physics_simulated::PhysicsSimulated;

use body::*;
use message::*;

pub struct BodyCollection<'a> {
    static_bodies: Vec<&'a mut Body>,
    dynamic_bodies: Vec<&'a mut Body>,
}

impl<'a> BodyCollection<'a> {
    pub fn new() -> BodyCollection<'a> {
        BodyCollection {
            static_bodies: Vec::new(),
            dynamic_bodies: Vec::new(),
        }
    }

    pub fn push(&mut self, body: &'a mut Body) {
        match body.body_type {
            BodyType::Static => {
                self.static_bodies.push(body);
            }
            BodyType::Dynamic => {
                self.dynamic_bodies.push(body);
            }
        }
    }

    pub fn append(&mut self, mut other: BodyCollection<'a>) {
        self.dynamic_bodies.append(&mut other.dynamic_bodies);
        self.static_bodies.append(&mut other.static_bodies);
    }
}

pub struct PhysicsScene<'a> {
    bodies: BodyCollection<'a>,

    positional_correction_percent: f32,
    positional_correction_slop: f32,
}

impl<'a> PhysicsScene<'a> {
    pub fn new(bodies: BodyCollection<'a>) -> PhysicsScene<'a> {
        PhysicsScene {
            bodies,
            positional_correction_percent: 0.6,
            positional_correction_slop: 0.01,
        }
    }

    pub fn simulate(&mut self, delta_time: f32) -> Vec<message::Message> {
        self.update_collider_positions();

        let messages = self.resolve_collisions();
        self.move_bodies(delta_time);

        self.update_collider_positions();

        messages
    }

    fn update_collider_positions(&mut self) {
        for body in &mut self.bodies.dynamic_bodies {
            body.collider.position = body.collider_initial_position + body.position;
        }

        for body in &mut self.bodies.static_bodies {
            body.collider.position = body.collider_initial_position + body.position;
        }
    }

    fn move_bodies(&mut self, delta_time: f32) {
        for body in &mut self.bodies.dynamic_bodies {
            body.velocity = body.velocity + body.force * delta_time * body.inv_mass;
            body.position = body.position + body.velocity * delta_time;
        }
    }

    fn resolve_collision(
        a: &mut Body,
        b: &mut Body,
        pc_percent: f32,
        pc_slop: f32,
    ) -> Option<message::Message> {
        let collision_data = a.collider.collide(&b.collider);
        match collision_data {
            Some(data) => {
                let bouncity = Math::min(a.bouncity, b.bouncity);

                let relative_velocity = b.velocity - a.velocity;
                let velocity_along_normal = relative_velocity.dot(data.normal);

                let msg = Some(Message {
                    body: MessageBody::Collided(data.clone()),
                    causer: a.id,
                    affected: b.id,
                });

                if velocity_along_normal > 0.0 {
                    return msg;
                }

                let inv_sum = a.inv_mass + b.inv_mass;
                let impulse = -1.0 * (1.0 + bouncity) * velocity_along_normal / inv_sum;
                let impulse_vec = data.normal * impulse;

                a.velocity = a.velocity - a.inv_mass * impulse_vec;
                b.velocity = b.velocity + b.inv_mass * impulse_vec;

                // Positional correction.
                let correction = (Math::max(data.depth - pc_slop, 0.0) / inv_sum) * pc_percent;
                let correction_vec = correction * data.normal;

                a.position = a.position - a.inv_mass * correction_vec;
                b.position = b.position + b.inv_mass * correction_vec;

                msg
            }
            None => None,
        }
    }

    fn resolve_collisions(&mut self) -> Vec<message::Message> {
        let mut messages = Vec::new();

        let dynamic_body_count = self.bodies.dynamic_bodies.len();
        for a in 0..dynamic_body_count {
            for b in 0..dynamic_body_count {
                if a <= b {
                    continue;
                }
                let (head, tail) = self.bodies.dynamic_bodies.split_at_mut(a);
                let a_body = &mut tail[0];
                let b_body = &mut head[b];

                let msg = Self::resolve_collision(
                    a_body,
                    b_body,
                    self.positional_correction_percent,
                    self.positional_correction_slop,
                );
                messages.extend(msg.into_iter());
            }
        }

        for a_body in &mut self.bodies.dynamic_bodies {
            for b_body in &mut self.bodies.static_bodies {
                let msg = Self::resolve_collision(
                    a_body,
                    b_body,
                    self.positional_correction_percent,
                    self.positional_correction_slop,
                );
                messages.extend(msg.into_iter());
            }
        }

        messages
    }
}
