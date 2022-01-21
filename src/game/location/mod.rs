use crate::game::game_entity::*;
use crate::common::asset_manager::AssetManager;
use crate::common::math::IVec2;
use crate::game::renderer::Renderer;

use crate::game::building::transport_belt::TransportBelt;
use crate::game::resource::item::ItemFactory;
use crate::game::location::surface::SurfaceFactory;

use crate::common::direction::Direction;

pub mod cell;
mod field;
pub mod surface;

use field::Field;

pub struct Location {
    field : Field
}

impl Location {
    pub fn new(asset_manager : &AssetManager) -> Location {
        let mut field = Field::new(IVec2::new(-10, -10), IVec2::new(10, 10), asset_manager);

        let items_dict = AssetManager::get_asset_id("dictionaries/items.json");
        let item_factory = ItemFactory::new(asset_manager.get_json(items_dict));

        // DEBUG MINER
        let json_asset = AssetManager::get_asset_id("dictionaries/miners.json");
        let json = asset_manager.get_json(json_asset);
        let miners = serde_json::from_str(json.as_ref()).unwrap();
        let mut err = false;
        let miners = crate::common::json_reader::JsonReader::read_vec(&miners, "miners", &mut err);
        let mut miner = crate::game::building::miner::Miner::from_json_object(&miners[0]);
        miner.init(SurfaceFactory::get_surface_id_by_name("grass"), &item_factory);

        let cell = field.get_cell_mut(IVec2::new(0, 0)).unwrap();
        cell.build(Box::from(miner));
        // DEBUG RECYCLER
        let json_asset = AssetManager::get_asset_id("dictionaries/recyclers.json");
        let json = asset_manager.get_json(json_asset);
        let recyclers = serde_json::from_str(json.as_ref()).unwrap();
        let mut err = false;
        let recyclers = crate::common::json_reader::JsonReader::read_vec(&recyclers, "recyclers", &mut err);
        let mut recycler = crate::game::building::recycler::Recycler::from_json_object(&recyclers[0]);
        recycler.init_items(&item_factory);

        let cell = field.get_cell_mut(IVec2::new(1, 1)).unwrap();
        cell.build(Box::from(recycler));
        // DEBUG TRANSPORT BELT
        let json_asset = AssetManager::get_asset_id("dictionaries/transport_belts.json");
        let json = asset_manager.get_json(json_asset);
        let tbs = serde_json::from_str(json.as_ref()).unwrap();
        let mut err = false;
        let tbs = crate::common::json_reader::JsonReader::read_vec(&tbs, "transport_belts", &mut err);
        let mut tb = TransportBelt::from_json_object(&tbs[0]);
            // setup
        tb.set_config(vec![Direction::Left, Direction::Up], Direction::Right);
            // setup 
        let cell = field.get_cell_mut(IVec2::new(1, 0)).unwrap();
        cell.build(Box::from(tb));
        // DEBUG TRANSPORT BELT
        let json_asset = AssetManager::get_asset_id("dictionaries/transport_belts.json");
        let json = asset_manager.get_json(json_asset);
        let tbs = serde_json::from_str(json.as_ref()).unwrap();
        let mut err = false;
        let tbs = crate::common::json_reader::JsonReader::read_vec(&tbs, "transport_belts", &mut err);
        let mut tb = TransportBelt::from_json_object(&tbs[0]);
            // setup
        tb.set_config(vec![Direction::Left], Direction::Up);
            // setup 
        let cell = field.get_cell_mut(IVec2::new(2, 0)).unwrap();
        cell.build(Box::from(tb));
        // DEBUG TRANSPORT BELT
        let json_asset = AssetManager::get_asset_id("dictionaries/transport_belts.json");
        let json = asset_manager.get_json(json_asset);
        let tbs = serde_json::from_str(json.as_ref()).unwrap();
        let mut err = false;
        let tbs = crate::common::json_reader::JsonReader::read_vec(&tbs, "transport_belts", &mut err);
        let mut tb = TransportBelt::from_json_object(&tbs[0]);
            // setup
        tb.set_config(vec![Direction::Down], Direction::Up);
            // setup 
        let cell = field.get_cell_mut(IVec2::new(2, 1)).unwrap();
        cell.build(Box::from(tb));
         
        Location { field }
    }
}

impl GameEntity for Location {
    fn update(&mut self, parameters : &UpdateParameters) {
        self.field.update(parameters);
    }

    fn tick(&mut self, tick_id : u32) {
        self.field.tick(tick_id);
    }

    fn render(&mut self, renderer : &mut Renderer, transform : SpriteTransform) {
        self.field.render(renderer, transform);
    }
}