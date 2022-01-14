use crate::game::game_entity::GameEntity;
use crate::game::renderer::Renderer;

pub mod recycler;
pub mod transport_belt;

pub trait BuildingClone {
    fn clone_box(&self) -> Box<dyn Building>;
}

pub trait Building : GameEntity + BuildingClone {
    fn get_name(&self) -> &str;
}

struct ErrorBuilding { }

impl GameEntity for ErrorBuilding {
    fn update(&mut self, delta_time : f32) { }
    fn tick(&mut self) { }
    fn render(&mut self, renderer : &mut Renderer) { }
}

impl BuildingClone for ErrorBuilding {
    fn clone_box(&self) -> Box<dyn Building> { Box::from(ErrorBuilding::new()) }
}

impl Building for ErrorBuilding {
    fn get_name(&self) -> &str { "error" }
}

impl ErrorBuilding {
    pub fn new() -> ErrorBuilding { ErrorBuilding { } }
}