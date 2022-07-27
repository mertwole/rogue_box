use ggez::event::EventHandler;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};

use common::asset_manager::AssetManager;
use common::math::{IVec2, Vec2};

pub mod common;
pub mod game_entity;
pub mod location;
pub mod message;
pub mod renderer;

use game_entity::*;
use location::{
    field::building::item::ItemFactory,
    physics_scene::{PhysicsScene, PhysicsSimulated},
    Location,
};
use renderer::{camera::Camera, Renderer};

pub const TICK_PERIOD: f32 = 1.0;

pub struct Game {
    renderer: Renderer,
    asset_manager: AssetManager,
    location: Location,

    from_last_tick: f32,
    tick_id: u32,
}

impl Game {
    pub fn new(context: &mut Context) -> Game {
        let mut asset_manager = AssetManager::new();

        asset_manager.load_assets(context);

        let drawable_size = graphics::drawable_size(context);
        let res = IVec2::new(drawable_size.0 as isize, drawable_size.1 as isize);
        let camera = Camera::new(res);
        let renderer = Renderer::new(camera);

        let items_dict = AssetManager::get_asset_id("dictionaries/items.json");
        let item_factory = ItemFactory::new(asset_manager.get_json(items_dict));

        let location = Location::new(&asset_manager);

        Game {
            location,
            asset_manager,
            renderer,

            from_last_tick: 0.0,
            tick_id: 0,
        }
    }

    fn update_all(&mut self, parameters: &UpdateParameters) {
        self.location.update(parameters);
    }

    fn tick_all(&mut self) {
        self.location.tick(self.tick_id);
    }

    fn render_all(&mut self) {
        let transform = SpriteTransform::default();
        self.location.render(&mut self.renderer, transform.clone());
    }
}

impl EventHandler for Game {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        let delta_time = ggez::timer::delta(context).as_secs_f32();

        self.from_last_tick += delta_time;
        if self.from_last_tick > TICK_PERIOD {
            self.from_last_tick -= TICK_PERIOD;
            self.tick_all();
            self.tick_id += 1;
        }

        let update_parameters = UpdateParameters {
            delta_time,
            from_last_tick: self.from_last_tick,
            last_tick_id: self.tick_id,
        };

        self.location.process_keyboard_input(context);

        let hierarchy = self.location.get_bodies();
        let mut scene = PhysicsScene::new(hierarchy);
        let messages = scene.simulate(delta_time);
        self.location.handle_physics_messages(messages);

        self.update_all(&update_parameters);

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, Color::WHITE);

        self.render_all();
        self.renderer.render_to_screen(context, &self.asset_manager);

        graphics::present(context)
    }
}
