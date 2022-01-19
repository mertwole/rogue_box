use std::rc::Rc;

use crate::game::game_entity::GameEntity;
use crate::common::asset_manager::{AssetManager, AssetId};
use crate::common::math::{Vec2, IVec2};
use crate::game::renderer::{Renderer, Sprite};

use crate::game::building::transport_belt::{TransportBelt, TransportedItem};
use crate::game::resource::item::{Item, ItemFactory};

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

        let items_dict = AssetManager::get_asset_id("dictionaries/items.json");
        let item_factory = ItemFactory::new(asset_manager.get_json(items_dict));

        // DEBUG RECYCLER
        let json_asset = AssetManager::get_asset_id("dictionaries/recyclers.json");
        let json = asset_manager.get_json(json_asset);
        let recyclers = serde_json::from_str(json.as_ref()).unwrap();
        let mut err = false;
        let recyclers = crate::common::json_reader::JsonReader::read_vec(&recyclers, "recyclers", &mut err);
        let mut recycler = crate::game::building::recycler::Recycler::from_json_object(&recyclers[0]);
        recycler.init_items(&item_factory);

        let cell = field.get_cell_mut(IVec2::new(0, 0)).unwrap();
        cell.build(Box::from(recycler));
        // DEBUG TRANSPORT BELT
        let json_asset = AssetManager::get_asset_id("dictionaries/transport_belts.json");
        let json = asset_manager.get_json(json_asset);
        let tbs = serde_json::from_str(json.as_ref()).unwrap();
        let mut err = false;
        let tbs = crate::common::json_reader::JsonReader::read_vec(&tbs, "transport_belts", &mut err);
        let mut tb = TransportBelt::from_json_object(&tbs[0]);
            // setup
        tb.set_config(vec![Direction::Left], Direction::Right);
        tb.position = IVec2::new(1, 0);
        //let item_id = ItemFactory::get_item_id_by_name("copper");
        //tb.item_buffers.get_mut(&Direction::Left).unwrap()[0] = Some(TransportedItem::new(item_factory.create_item(item_id)));
        //let item_id = ItemFactory::get_item_id_by_name("copper");
        //tb.item_buffers.get_mut(&Direction::Left).unwrap()[1] = Some(TransportedItem::new(item_factory.create_item(item_id)));
            // setup 
        let cell = field.get_cell_mut(tb.position).unwrap();
        cell.build(Box::from(tb));
        
        let json_asset = AssetManager::get_asset_id("dictionaries/transport_belts.json");
        let json = asset_manager.get_json(json_asset);
        let tbs = serde_json::from_str(json.as_ref()).unwrap();
        let mut err = false;
        let tbs = crate::common::json_reader::JsonReader::read_vec(&tbs, "transport_belts", &mut err);
        let mut tb = TransportBelt::from_json_object(&tbs[0]);
            // setup
        tb.set_config(vec![Direction::Left], Direction::Right);
        tb.position = IVec2::new(2, 0);
            // setup 
        let cell = field.get_cell_mut(tb.position).unwrap();
        cell.build(Box::from(tb));

        let json_asset = AssetManager::get_asset_id("dictionaries/transport_belts.json");
        let json = asset_manager.get_json(json_asset);
        let tbs = serde_json::from_str(json.as_ref()).unwrap();
        let mut err = false;
        let tbs = crate::common::json_reader::JsonReader::read_vec(&tbs, "transport_belts", &mut err);
        let mut tb = TransportBelt::from_json_object(&tbs[0]);
            // setup
        tb.set_config(vec![Direction::Left], Direction::Right);
        tb.position = IVec2::new(3, 0);
            // setup 
        let cell = field.get_cell_mut(tb.position).unwrap();
        cell.build(Box::from(tb));
         
        Location { field }
    }
}

impl GameEntity for Location {
    fn update(&mut self, delta_time : f32) {
        self.field.update(delta_time);
    }

    fn tick(&mut self, tick_id : u32) {
        self.field.tick(tick_id);
    }

    fn render(&mut self, renderer : &mut Renderer) {
        self.field.render(renderer);
    }
}