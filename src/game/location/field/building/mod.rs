use crate::game::{game_entity::*, gui::with_gui::*, message::*, renderer::Renderer};

pub mod craft_station;
pub mod error_building;
pub mod item;
pub mod miner;
pub mod recycler;
pub mod transport_belt;

pub trait BuildingClone {
    fn clone_box(&self) -> Box<dyn Building>;
}

pub trait Building: GameEntity + BuildingClone + MessageReceiver + MessageSender + WithGui {
    fn get_name(&self) -> &str;
}
