use std::collections::{HashMap, HashSet};

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

#[derive(Default)]
pub struct BodyCollection<'a> {
    static_bodies: Vec<&'a mut Body>,
    dynamic_bodies: Vec<&'a mut Body>,
}

impl<'a> BodyCollection<'a> {
    fn get_body_ids(&self) -> HashSet<BodyId> {
        self.static_bodies
            .iter()
            .chain(self.dynamic_bodies.iter())
            .map(|body| body.id)
            .collect()
    }

    fn concatenate(&mut self, other: BodyCollection<'a>) {
        self.static_bodies.extend(other.static_bodies);
        self.dynamic_bodies.extend(other.dynamic_bodies);
    }

    pub fn push(&mut self, body: &'a mut Body) {
        match body.body_type {
            BodyType::Static => {
                self.static_bodies.push(body);
            }
            BodyType::Dynamic | BodyType::Kinematic => {
                self.dynamic_bodies.push(body);
            }
        }
    }
}

#[derive(Default)]
pub struct BodyHierarchyRoot<'a> {
    bodies: BodyCollection<'a>,
    hierarchy: BodyHierarchyNode,
}

impl<'a> BodyHierarchyRoot<'a> {
    pub fn new(
        child_hierarchies: Vec<BodyHierarchyRoot<'a>>,
        bodies: BodyCollection<'a>,
    ) -> BodyHierarchyRoot<'a> {
        let body_ids = bodies.get_body_ids();
        let mut body_collection = bodies;
        let mut children = vec![];
        child_hierarchies.into_iter().for_each(|hierarchy| {
            body_collection.concatenate(hierarchy.bodies);
            children.push(hierarchy.hierarchy);
        });
        BodyHierarchyRoot {
            hierarchy: BodyHierarchyNode {
                bodies: body_ids,
                children,
            },
            bodies: body_collection,
        }
    }

    pub fn add_body(&mut self, body: &'a mut Body) {
        self.hierarchy.bodies.insert(body.id);
        self.bodies.push(body);
    }
}

#[derive(Default)]
pub struct BodyHierarchyNode {
    children: Vec<BodyHierarchyNode>,
    bodies: HashSet<BodyId>,
}

pub struct PhysicsScene<'a> {
    hierarchy: BodyHierarchyRoot<'a>,

    positional_correction_percent: f32,
    positional_correction_slop: f32,
}

impl<'a> PhysicsScene<'a> {
    pub fn new(hierarchy: BodyHierarchyRoot<'a>) -> PhysicsScene<'a> {
        PhysicsScene {
            hierarchy,
            positional_correction_percent: 0.6,
            positional_correction_slop: 0.01,
        }
    }

    pub fn simulate(&mut self, delta_time: f32) -> MessageHierarchy {
        self.update_collider_positions();
        let messages = self.gather_messages();
        self.move_bodies(delta_time);
        self.update_collider_positions();
        Self::construct_message_hierarchy(&self.hierarchy.hierarchy, &messages)
    }

    fn gather_messages(&mut self) -> HashMap<BodyId, Vec<Message>> {
        let mut messages: HashMap<BodyId, Vec<Message>> = HashMap::new();

        for a_body in &mut self.hierarchy.bodies.static_bodies {
            for b_body in &mut self.hierarchy.bodies.dynamic_bodies {
                let msgs = resolve_collision(
                    a_body,
                    b_body,
                    self.positional_correction_percent,
                    self.positional_correction_slop,
                );
                for msg in msgs {
                    messages.entry(msg.affected).or_insert(vec![]).push(msg);
                }
            }
        }

        let dynamic_body_count = self.hierarchy.bodies.dynamic_bodies.len();
        if dynamic_body_count < 2 {
            return messages;
        }

        for a in 0..(dynamic_body_count - 1) {
            for b in (a + 1)..dynamic_body_count {
                let (head, tail) = self.hierarchy.bodies.dynamic_bodies.split_at_mut(a + 1);
                let b_body = &mut tail[b - a - 1];
                let a_body = &mut head[a];

                let msgs = resolve_collision(
                    a_body,
                    b_body,
                    self.positional_correction_percent,
                    self.positional_correction_slop,
                );
                for msg in msgs {
                    messages.entry(msg.affected).or_insert(vec![]).push(msg);
                }
            }
        }

        messages
    }

    fn update_collider_positions(&mut self) {
        for body in self
            .hierarchy
            .bodies
            .static_bodies
            .iter_mut()
            .chain(self.hierarchy.bodies.dynamic_bodies.iter_mut())
        {
            body.collider.position = body.collider_initial_position + body.position;
        }
    }

    fn move_bodies(&mut self, delta_time: f32) {
        for body in &mut self.hierarchy.bodies.dynamic_bodies {
            if !body.is_kinematic() {
                body.velocity = body.velocity + body.force * delta_time * body.inv_mass;
                body.velocity = body.velocity * f32::powf(1.0 - body.ground_friction, delta_time);
                body.position = body.position + body.velocity * delta_time;
            }
        }
    }

    fn construct_message_hierarchy(
        hierarchy: &BodyHierarchyNode,
        messages: &HashMap<BodyId, Vec<Message>>,
    ) -> MessageHierarchy {
        let mut nested = vec![];

        for nested_hierarchy in &hierarchy.children {
            nested.push(Self::construct_message_hierarchy(
                nested_hierarchy,
                messages,
            ));
        }

        let mut msgs = HashMap::new();
        for body_id in &hierarchy.bodies {
            if let Some(msg) = messages.get(body_id) {
                msgs.insert(*body_id, msg.clone());
            }
        }

        MessageHierarchy {
            messages: msgs,
            nested,
        }
    }
}

fn resolve_collision(
    a: &mut Body,
    b: &mut Body,
    pc_percent: f32,
    pc_slop: f32,
) -> Vec<message::Message> {
    let collision_data = a.collider.collide(&b.collider);
    match collision_data {
        Some(data) => {
            let bouncity = Math::min(a.bouncity, b.bouncity);

            let relative_velocity = b.velocity - a.velocity;
            let velocity_along_normal = relative_velocity.dot(data.normal);

            if velocity_along_normal < 0.0 {
                let inv_sum = a.inv_mass + b.inv_mass;
                let impulse = -1.0 * (1.0 + bouncity) * velocity_along_normal / inv_sum;
                let impulse_vec = data.normal * impulse;

                if !a.is_kinematic() {
                    a.velocity = a.velocity - a.inv_mass * impulse_vec;
                }
                if !b.is_kinematic() {
                    b.velocity = b.velocity + b.inv_mass * impulse_vec;
                }

                // Positional correction.
                let correction = (Math::max(data.depth - pc_slop, 0.0) / inv_sum) * pc_percent;
                let correction_vec = correction * data.normal;

                if !a.is_kinematic() {
                    a.position = a.position - a.inv_mass * correction_vec;
                }
                if !b.is_kinematic() {
                    b.position = b.position + b.inv_mass * correction_vec;
                }
            }

            if !a.collider.is_trigger && !b.collider.is_trigger {
                let mut reversed_data = data.clone();
                reversed_data.reverse();
                vec![
                    Message {
                        body: MessageBody::Collided(data),
                        causer: a.id,
                        affected: b.id,
                    },
                    Message {
                        body: MessageBody::Collided(reversed_data),
                        causer: b.id,
                        affected: a.id,
                    },
                ]
            } else {
                let mut messages = vec![];
                if b.collider.is_trigger {
                    messages.push(Message {
                        body: MessageBody::TriggerEntered,
                        causer: a.id,
                        affected: b.id,
                    });
                }
                if a.collider.is_trigger {
                    messages.push(Message {
                        body: MessageBody::TriggerEntered,
                        causer: b.id,
                        affected: a.id,
                    });
                }
                messages
            }
        }
        None => vec![],
    }
}
