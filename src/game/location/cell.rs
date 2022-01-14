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
    building : Option<Rc<dyn Building>>
}

impl Cell {
    pub fn new(position : IVec2) -> Cell {
        let texture = AssetManager::get_asset_id("textures/surfaces/grass.png");
        let mut sprite = Sprite::new(texture);
        sprite.position = position.to_vec2();
        Cell {
            position,
            sprite,
            building : None
        }
    }

    pub fn build(&mut self, building : Rc<dyn Building>) {
        self.building = Some(building);
    }
}

impl GameEntity for Cell {
    fn update(&mut self, delta_time : f32) {
        // match self.building.clone() {
        //     Some(mut building) => { 
        //         let building = Rc::get_mut(&mut building).unwrap();
        //         building.update(delta_time); 
        //     }
        //     _ => { }
        // }
    }

    fn tick(&mut self) {
        // match self.building.clone() {
        //     Some(mut building) => { 
        //         let building = Rc::get_mut(&mut building).unwrap();
        //         building.tick(); 
        //     }
        //     _ => { }
        // }
    }

    fn render(&mut self, renderer : &mut Renderer) {
        renderer.queue_render_sprite(&self.sprite);
        
        // match self.building.clone() {
        //     Some(mut building) => { 
        //         let building = Rc::get_mut(&mut building).unwrap();
        //         building.render(renderer); 
        //     }
        //     _ => { }
        // }
    }
}