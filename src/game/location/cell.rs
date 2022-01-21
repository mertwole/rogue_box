use crate::game::game_entity::*;
use crate::common::asset_manager::AssetManager;
use crate::game::building::Building;
use crate::game::renderer::{Renderer, Sprite};

pub struct Cell {
    sprite : Sprite,
    pub building : Option<Box<dyn Building>>
}

impl Cell {
    pub fn new() -> Cell {
        let texture = AssetManager::get_asset_id("textures/surfaces/grass.png");
        let sprite = Sprite::new(texture);
        Cell {
            sprite,
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
        renderer.queue_render_sprite(self.sprite.clone(), transform.clone());
        
        match self.building.as_mut() {
            Some(building) => { 
                building.render(renderer, transform); 
            }
            _ => { }
        }
    }
}