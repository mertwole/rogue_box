use ggez::{Context, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::EventHandler;

use crate::common::math::IVec2;
use crate::common::asset_manager::AssetManager;

pub mod location;
pub mod building;
pub mod resource;
pub mod game_entity;
pub mod renderer;
pub mod message;

use location::Location;
use game_entity::*;
use renderer::{Renderer, camera::Camera};
use resource::item::*;

const TICK_PERIOD : f32 = 1.0;

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

    fn update_all(&mut self, delta_time : f32) {
        self.location.update(delta_time);
    }

    fn tick_all(&mut self) {
        self.location.tick(self.tick_id);
        self.tick_id += 1;
    } 
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let delta_time = ggez::timer::delta(_ctx).as_secs_f32();

        self.from_last_tick += delta_time;
        if self.from_last_tick > TICK_PERIOD {
            self.from_last_tick -= TICK_PERIOD;
            self.tick_all();
        }

        self.update_all(delta_time);

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, Color::WHITE);
        
        self.location.render(&mut self.renderer);

        self.renderer.render_to_screen(context, &self.asset_manager, &self.camera);

        graphics::present(context)
    }
}