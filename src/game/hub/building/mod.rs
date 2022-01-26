use crate::game::game_entity::*;
use crate::game::renderer::Renderer;
use crate::game::hub::message::*;
use crate::game::hub::electric_port::ElectricPort;

pub mod recycler;
pub mod miner;
pub mod transport_belt;
pub mod error_building;

pub trait BuildingClone {
    fn clone_box(&self) -> Box<dyn Building>;
}

pub trait Building : GameEntity + BuildingClone + MessageReceiver + MessageSender {
    fn get_name(&self) -> &str;
    fn get_electric_ports(&self) -> Vec<&dyn ElectricPort>;
    fn get_electric_ports_mut(&mut self) -> Vec<&mut Box<dyn ElectricPort>>;
}