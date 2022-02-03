use super::*;

pub trait PhysicsSimulated {
    fn get_all_bodies(&mut self) -> BodyCollection;
}
