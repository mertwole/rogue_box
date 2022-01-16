use std::rc::Rc;

use crate::game::game_entity::GameEntity;
use crate::common::asset_manager::{AssetId, AssetManager};
use crate::game::building::Building;
use crate::game::building::transport_belt::TransportBelt;
use crate::game::renderer::{Renderer, Sprite};
use crate::common::math::IVec2;

pub struct Cell {
    position : IVec2,
    sprite : Sprite,
    pub building : Option<Box<dyn Building>>
}

impl Cell {
    pub fn new(position : IVec2) -> Cell {
        let texture = AssetManager::get_asset_id("textures/surfaces/stone.png");
        let mut sprite = Sprite::new(texture);
        sprite.position = position.to_vec2();
        Cell {
            position,
            sprite,
            building : None
        }
    }

    pub fn build(&mut self, building : Box<dyn Building>) {
        self.building = Some(building);
    }
}

impl GameEntity for Cell {
    fn update(&mut self, delta_time : f32) {
        match self.building.as_mut() {
            Some(building) => { 
                building.update(delta_time); 
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

    fn render(&mut self, renderer : &mut Renderer) {
        renderer.queue_render_sprite(&self.sprite);
        
        match self.building.as_mut() {
            Some(building) => { 
                building.render(renderer); 
            }
            _ => { }
        }
    }
}