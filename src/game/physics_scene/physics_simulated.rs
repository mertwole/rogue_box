use super::*;
use super::message::*;

pub trait PhysicsSimulated {
    fn get_all_bodies(&mut self) -> BodyCollection;
    fn handle_physics_messages(&mut self, messages : Vec<Message>);
}
