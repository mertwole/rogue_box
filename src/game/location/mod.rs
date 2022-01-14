use std::rc::Rc;

use crate::game::game_entity::GameEntity;
use crate::common::asset_manager::{AssetManager, AssetId};
use crate::common::math::{Vec2, IVec2};
use crate::game::renderer::{Renderer, Sprite};

use crate::game::building::transport_belt::{TransportBelt, transport_belt_manager::TransportBeltManager};
use crate::game::building::*;
use crate::common::direction::Direction;

pub mod cell;
mod field;

use field::Field;

pub struct Location {
    field : Field
}

impl Location {
    pub fn new(asset_manager : &AssetManager) -> Location {
        let mut field = Field::new(IVec2::new(-10, -10), IVec2::new(10, 10));
        // DEBUG RECYCLER
        let json_asset = AssetManager::get_asset_id("dictionaries/recyclers.json");
        let json = asset_manager.get_json(json_asset);
        let recyclers = serde_json::from_str(json.as_ref()).unwrap();
        let mut err = false;
        let recyclers = crate::common::json_reader::JsonReader::read_vec(&recyclers, "recyclers", &mut err);
        let recycler = crate::game::building::recycler::Recycler::from_json_object(&recyclers[0]);

        let cell = field.get_cell_mut(IVec2::new(0, 0)).unwrap();
        cell.build(Rc::from(recycler));
        // DEBUG TRANSPORT BELT MANAGER
        let mut tb_manager = TransportBeltManager::new();
        // DEBUG TRANSPORT BELT
        let json_asset = AssetManager::get_asset_id("dictionaries/transport_belts.json");
        let json = asset_manager.get_json(json_asset);
        let tbs = serde_json::from_str(json.as_ref()).unwrap();
        let mut err = false;
        let tbs = crate::common::json_reader::JsonReader::read_vec(&tbs, "transport_belts", &mut err);
        let mut tb = TransportBelt::from_json_object(&tbs[0]);
            // setup
        tb.inputs = vec![Direction::Left];
        tb.output = Direction::Right;
        tb.position = IVec2::new(1, 0);
            // setup 
        let cell = field.get_cell_mut(tb.position).unwrap();
        let tb_rc = Rc::from(tb);
        let pos = tb_rc.as_ref().position;
        tb_manager.add_transport_belt(tb_rc.clone(), pos);
        cell.build(tb_rc);
        
        let json_asset = AssetManager::get_asset_id("dictionaries/transport_belts.json");
        let json = asset_manager.get_json(json_asset);
        let tbs = serde_json::from_str(json.as_ref()).unwrap();
        let mut err = false;
        let tbs = crate::common::json_reader::JsonReader::read_vec(&tbs, "transport_belts", &mut err);
        let mut tb = TransportBelt::from_json_object(&tbs[0]);
            // setup
        tb.inputs = vec![Direction::Left];
        tb.output = Direction::Right;
        tb.position = IVec2::new(2, 0);
            // setup 
        let cell = field.get_cell_mut(tb.position).unwrap();
        let tb_rc = Rc::from(tb);
        let pos = tb_rc.as_ref().position;
        tb_manager.add_transport_belt(tb_rc.clone(), pos);
        cell.build(tb_rc);

        let json_asset = AssetManager::get_asset_id("dictionaries/transport_belts.json");
        let json = asset_manager.get_json(json_asset);
        let tbs = serde_json::from_str(json.as_ref()).unwrap();
        let mut err = false;
        let tbs = crate::common::json_reader::JsonReader::read_vec(&tbs, "transport_belts", &mut err);
        let mut tb = TransportBelt::from_json_object(&tbs[0]);
            // setup
        tb.inputs = vec![Direction::Left];
        tb.output = Direction::Right;
        tb.position = IVec2::new(3, 0);
            // setup 
        let cell = field.get_cell_mut(tb.position).unwrap();
        let tb_rc = Rc::from(tb);
        let pos = tb_rc.as_ref().position;
        tb_manager.add_transport_belt(tb_rc.clone(), pos);
        cell.build(tb_rc);

        let mut tick_order = tb_manager.get_tick_order();
        loop {
            match tick_order.next() {
                Some(pos) => { print!("({}, {}) -> ", pos.x, pos.y); }
                None => { println!("end"); break; }
            }
        }

        Location { field }
    }
}

impl GameEntity for Location {
    fn update(&mut self, delta_time : f32) {
        self.field.update(delta_time);
    }

    fn tick(&mut self) {
        self.field.tick();
    }

    fn render(&mut self, renderer : &mut Renderer) {
        self.field.render(renderer);
    }
}