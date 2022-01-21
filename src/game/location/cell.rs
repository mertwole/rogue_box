use crate::game::game_entity::*;
use crate::game::building::Building;
use crate::game::renderer::Renderer;
use super::surface::Surface;

pub struct Cell {
    surface : Surface,
    pub building : Option<Box<dyn Building>>
}

impl Cell {
    pub fn new(surface : Surface) -> Cell {
        Cell {
            surface,
            building : None
        }
    }

    pub fn build(&mut self, building : Box<dyn Building>) {
        self.building = Some(building);
    }
}

impl GameEntity for Cell {
    fn update(&mut self, parameters : &UpdateParameters) {
        match self.building.as_mut() {
            Some(building) => { 
                building.update(parameters); 
            }
            _ => { }
        }
    }

    fn tick(&mut self, tick_id : u32) {
        match self.building.as_mut() {
            Some(building) => { 
                building.tick(tick_id); 
            }
            _ => { }
        }
    }

    fn render(&mut self, renderer : &mut Renderer, transform : SpriteTransform) {
        self.surface.render(renderer, transform.clone());

        match self.building.as_mut() {
            Some(building) => { 
                building.render(renderer, transform); 
            }
            _ => { }
        }
    }
}