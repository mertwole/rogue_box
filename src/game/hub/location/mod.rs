use crate::game::common::asset_manager::AssetManager;
use crate::game::common::math::IVec2;
use crate::game::game_entity::*;
use crate::game::renderer::Renderer;

use crate::game::hub::building::transport_belt::TransportBelt;
use crate::game::hub::item::ItemFactory;
use crate::game::hub::location::surface::SurfaceFactory;

use crate::game::common::direction::Direction;
use crate::game::hub::electric_port::PortId;

use crate::game::field::*;

pub mod cell;
pub mod surface;

use cell::Cell;

pub struct Location {
    field: Field<Cell>,
}

impl Location {
    pub fn new(asset_manager: &AssetManager) -> Location {
        let mut field = Field::<Cell>::new(IVec2::new(-100, -100), IVec2::new(100, 100));

        let surface_json = AssetManager::get_asset_id("dictionaries/surfaces.json");
        let surface_dict = asset_manager.get_json(surface_json);
        let surface_factory = SurfaceFactory::new(surface_dict);
        let grass_surface_id = SurfaceFactory::get_surface_id_by_name("grass");

        for y in -100..100 {
            for x in -100..100 {
                let cell = field.get_cell_mut(IVec2::new(x, y)).unwrap();
                let grass_surface = surface_factory.create_surface(grass_surface_id);
                *cell = Cell::new(grass_surface);
            }
        }

        let items_dict = AssetManager::get_asset_id("dictionaries/items.json");
        let item_factory = ItemFactory::new(asset_manager.get_json(items_dict));

        // DEBUG MINER
        // let json_asset = AssetManager::get_asset_id("dictionaries/miners.json");
        // let json = asset_manager.get_json(json_asset);
        // let miners = serde_json::from_str(json.as_ref()).unwrap();
        // let mut err = false;
        // let miners = crate::game::common::json_reader::JsonReader::read_vec(&miners, "miners", &mut err);
        // let mut miner = crate::game::hub::building::miner::Miner::from_json_object(&miners[0]);
        // miner.init(SurfaceFactory::get_surface_id_by_name("grass"), &item_factory);

        // let cell = field.get_cell_mut(IVec2::new(0, 0)).unwrap();
        // cell.build(Box::from(miner));
        // DEBUG RECYCLER
        let json_asset = AssetManager::get_asset_id("dictionaries/recyclers.json");
        let json = asset_manager.get_json(json_asset);
        let recyclers = serde_json::from_str(json.as_ref()).unwrap();
        let mut err = false;
        let recyclers = crate::game::common::json_reader::JsonReader::read_vec(
            &recyclers,
            "recyclers",
            &mut err,
        );
        let mut recycler =
            crate::game::hub::building::recycler::Recycler::from_json_object(&recyclers[0]);
        recycler.init_items(&item_factory);

        let cell = field.get_cell_mut(IVec2::new(1, 1)).unwrap();
        cell.build(Box::from(recycler));
        // DEBUG GENERATOR
        let json_asset = AssetManager::get_asset_id("dictionaries/recyclers.json");
        let json = asset_manager.get_json(json_asset);
        let recyclers = serde_json::from_str(json.as_ref()).unwrap();
        let mut err = false;
        let recyclers = crate::game::common::json_reader::JsonReader::read_vec(
            &recyclers,
            "recyclers",
            &mut err,
        );
        let mut recycler =
            crate::game::hub::building::recycler::Recycler::from_json_object(&recyclers[1]);
        recycler.init_items(&item_factory);

        let out = recycler.electric_ports[0].as_output_mut().unwrap();
        out.connect(IVec2::new(1, 1), PortId::new(0));

        let cell = field.get_cell_mut(IVec2::new(2, 2)).unwrap();
        cell.build(Box::from(recycler));
        // DEBUG TRANSPORT BELT
        let json_asset = AssetManager::get_asset_id("dictionaries/transport_belts.json");
        let json = asset_manager.get_json(json_asset);
        let tbs = serde_json::from_str(json.as_ref()).unwrap();
        let mut err = false;
        let tbs = crate::game::common::json_reader::JsonReader::read_vec(
            &tbs,
            "transport_belts",
            &mut err,
        );
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
        let tbs = crate::game::common::json_reader::JsonReader::read_vec(
            &tbs,
            "transport_belts",
            &mut err,
        );
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
        let tbs = crate::game::common::json_reader::JsonReader::read_vec(
            &tbs,
            "transport_belts",
            &mut err,
        );
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
    fn update(&mut self, parameters: &UpdateParameters) {
        self.field.update(parameters);
    }

    fn tick(&mut self, tick_id: u32) {
        self.field.tick(tick_id);
    }

    fn render(&mut self, renderer: &mut Renderer, transform: SpriteTransform) {
        self.field.render(renderer, transform);
    }
}
