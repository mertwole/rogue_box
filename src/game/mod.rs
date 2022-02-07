use ggez::{Context, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::EventHandler;

use common::math::{Vec2, IVec2};
use common::asset_manager::AssetManager;

pub mod game_entity;
pub mod renderer;
pub mod common;

mod player;
mod physics_scene;

pub mod hub;
mod trip;

use player::Player;
use hub::location::Location;
use hub::item::*;
use game_entity::*;
use renderer::{Renderer, camera::Camera};
use physics_scene::*;

pub const TICK_PERIOD : f32 = 1.0;

pub struct Game {
    renderer : Renderer,
    asset_manager : AssetManager,
    location : Location,
    player : Player,

    from_last_tick : f32,
    tick_id : u32
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

        let player = Player::new(Vec2::new(-1.0, -1.0));

        Game { 
            player, 
            location, 
            asset_manager, 
            renderer,
            
            from_last_tick : 0.0,
            tick_id : 0
        }
    }

    fn simulate_physics(&mut self, delta_time : f32) -> Vec<message::Message> {
        let mut bodies = BodyCollection::new();

        let collider = Collider::new(ColliderShape::Box { size : Vec2::new_xy(1.0) }, Vec2::zero());
        let mut body = Body::new_static(collider, Vec2::new_xy(1.0));
        bodies.push(&mut body);

        let collider = Collider::new(ColliderShape::Box { size : Vec2::new_xy(1.0) }, Vec2::zero());
        let mut body = Body::new_static(collider, Vec2::new_xy(2.0));
        bodies.push(&mut body);

        bodies.append(self.player.get_all_bodies());

        let mut scene = PhysicsScene::new(bodies);
        scene.simulate(delta_time)
    }

    fn update_all(&mut self, parameters : &UpdateParameters) {
        self.location.update(parameters);
        self.player.update(parameters);
    }

    fn tick_all(&mut self) {
        self.location.tick(self.tick_id);
        self.player.tick(self.tick_id);
    } 

    fn render_all(&mut self) {
        let transform = SpriteTransform::default();
        self.location.render(&mut self.renderer, transform.clone());
        self.player.render(&mut self.renderer, transform);
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
            from_last_tick : self.from_last_tick,
            last_tick_id : self.tick_id
        };

        self.player.process_keyboard_input(context);

        let physics_messages = self.simulate_physics(update_parameters.delta_time);

        self.player.handle_physics_messages(physics_messages);

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