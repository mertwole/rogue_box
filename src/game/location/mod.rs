use ggez::Context;

use crate::game::common::{
    asset_manager::AssetManager,
    direction::Direction,
    math::{IVec2, Vec2},
};
use crate::game::{game_entity::*, gui::with_gui::*, message::*, renderer::Renderer};
pub mod field;
pub mod physics_scene;
mod player;

use field::{
    building::{craft_station::CraftStation, item::ItemFactory, transport_belt::TransportBelt},
    cell::{surface::SurfaceFactory, Cell},
    Field,
};
use physics_scene::{BodyCollection, BodyHierarchyRoot, PhysicsSimulated};

use super::renderer::camera::CameraProperties;

pub struct Location {
    field: Field,
    camera_properties: CameraProperties,
}

impl Location {
    pub fn new(asset_manager: &AssetManager) -> Location {
        let mut field = Field::new(IVec2::new(-5, -5), IVec2::new(5, 5));

        let surface_json = AssetManager::get_asset_id("dictionaries/surfaces.json");
        let surface_dict = asset_manager.get_json(surface_json);
        let surface_factory = SurfaceFactory::new(surface_dict);
        let grass_surface_id = SurfaceFactory::get_surface_id_by_name("grass");

        for cell in &mut field.iter_mut() {
            let grass_surface = surface_factory.create_surface(grass_surface_id);
            *cell = Cell::new(grass_surface);
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
        let mut recycler = field::building::recycler::Recycler::from_json_object(&recyclers[0]);
        recycler.init_items(&item_factory);

        let cell = field.get_cell_mut(IVec2::new(1, 1)).unwrap();
        cell.build(Box::from(recycler), Vec2::new(1.0, 1.0));
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
        let mut recycler = field::building::recycler::Recycler::from_json_object(&recyclers[1]);
        recycler.init_items(&item_factory);

        let cell = field.get_cell_mut(IVec2::new(2, 2)).unwrap();
        cell.build(Box::from(recycler), Vec2::new(2.0, 2.0));
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
        cell.build(Box::from(tb), Vec2::new(1.0, 0.0));
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
        cell.build(Box::from(tb), Vec2::new(2.0, 0.0));
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
        cell.build(Box::from(tb), Vec2::new(2.0, 1.0));
        // DEBUG CRAFT STATION
        let json_asset = AssetManager::get_asset_id("dictionaries/craft_stations.json");
        let json = asset_manager.get_json(json_asset);
        let craft_s = serde_json::from_str(json.as_ref()).unwrap();
        let craft_s = crate::game::common::json_reader::JsonReader::read_vec(
            &craft_s,
            "craft_stations",
            &mut false,
        );
        let craft_station = CraftStation::from_json_object(&craft_s[0], &item_factory);
        let cell = field.get_cell_mut(IVec2::new(4, 1)).unwrap();
        cell.build(Box::from(craft_station), Vec2::new(4.0, 1.0));

        Location {
            field,
            camera_properties: CameraProperties::default(),
        }
    }

    // TODO : IT'S DEBUG
    pub fn process_keyboard_input(&mut self, context: &Context) {
        self.field.process_keyboard_input(context);
    }

    pub fn get_camera_properties(&self) -> CameraProperties {
        let mut props = self.camera_properties.clone();
        props.offset = self.field.player.body.get_position();
        props
    }
}

impl GameEntity for Location {
    fn update(&mut self, parameters: &UpdateParameters) {
        self.field.update(parameters);
    }

    fn tick(&mut self, tick_id: u32) {
        self.field.tick(tick_id);

        let mut messages = self.field.pull_messages(tick_id);
        loop {
            let msg = match messages.pop() {
                Some(msg) => msg,
                None => break,
            };
            let msg = self.field.try_push_message(msg);
            messages.append(&mut msg.into_iter().collect());
        }
    }

    fn render(&mut self, renderer: &mut Renderer, transform: SpriteTransform) {
        self.field.render(renderer, transform.clone());
    }
}

impl PhysicsSimulated for Location {
    fn get_bodies(&mut self) -> BodyHierarchyRoot {
        BodyHierarchyRoot::new(vec![self.field.get_bodies()], BodyCollection::default())
    }

    fn handle_physics_messages(&mut self, mut messages: physics_message::MessageHierarchy) {
        self.field
            .handle_physics_messages(messages.nested.pop().unwrap())
    }

    fn physics_update(&mut self, delta_time: f32) {
        self.field.physics_update(delta_time);
    }
}

impl WithGui for Location {
    fn render_gui(&mut self, params: &mut GuiRenderParams) {
        self.field.render_gui(params);
    }
}
