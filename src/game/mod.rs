use ggez::{Context, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::EventHandler;

use common::math::IVec2;
use common::asset_manager::AssetManager;

pub mod game_entity;
pub mod renderer;
pub mod hub;
pub mod common;

use hub::location::Location;
use hub::item::*;
use game_entity::*;
use renderer::{Renderer, camera::Camera};

pub const TICK_PERIOD : f32 = 1.0;

pub struct Game {
    camera : Camera,
    renderer : Renderer,
    asset_manager : AssetManager,
    location : Location,

    from_last_tick : f32,
    tick_id : u32
}

impl Game {
    pub fn new(context: &mut Context) -> Game {
        let renderer = Renderer::new();
        let mut asset_manager = AssetManager::new();

        asset_manager.load_assets(context);

        let drawable_size = graphics::drawable_size(context);
        let res = IVec2::new(drawable_size.0 as isize, drawable_size.1 as isize);
        let camera = Camera::new(res);

        let items_dict = AssetManager::get_asset_id("dictionaries/items.json");
        let item_factory = ItemFactory::new(asset_manager.get_json(items_dict));

        let location = Location::new(&asset_manager);

        Game { 
            location, 
            renderer, 
            asset_manager, 
            camera,  
            
            from_last_tick : 0.0,
            tick_id : 0
        }
    }

    fn update_all(&mut self, parameters : &UpdateParameters) {
        self.location.update(parameters);
    }

    fn tick_all(&mut self) {
        self.location.tick(self.tick_id);
        
    } 
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let delta_time = ggez::timer::delta(_ctx).as_secs_f32();

        self.from_last_tick += delta_time;
        if self.from_last_tick > TICK_PERIOD {
            self.from_last_tick -= TICK_PERIOD;
            self.tick_all();
            self.tick_id += 1;
        }

        let update_parameters = UpdateParameters {
            delta_time,
            from_last_tick : self.from_last_tick,
            last_tick_id : self.tick_id
        };

        self.update_all(&update_parameters);

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, Color::WHITE);
        
        let transform = SpriteTransform::default();
        self.location.render(&mut self.renderer, transform);

        self.renderer.render_to_screen(context, &self.asset_manager, &self.camera);

        graphics::present(context)
    }
}