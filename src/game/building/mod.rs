use crate::game::game_entity::*;
use crate::game::renderer::Renderer;
use crate::game::message::*;

pub mod recycler;
pub mod transport_belt;
pub mod error_building;

pub trait BuildingClone {
    fn clone_box(&self) -> Box<dyn Building>;
}

pub trait Building : GameEntity + BuildingClone + MessageReceiver + MessageSender {
    fn get_name(&self) -> &str;
}