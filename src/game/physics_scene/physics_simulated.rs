use super::message::*;
use super::*;

pub trait PhysicsSimulated {
    fn get_all_bodies(&mut self) -> BodyCollection;
    fn handle_physics_messages(&mut self, messages: Vec<Message>);
}
