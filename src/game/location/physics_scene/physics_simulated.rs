use super::message::*;
use super::*;

pub trait PhysicsSimulated {
    fn get_bodies(&mut self) -> BodyHierarchyRoot;
    fn handle_physics_messages(&mut self, messages: MessageHierarchy);
    fn physics_update(&mut self, delta_time: f32);
}
