use ggez::event::{EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};

use common::asset_manager::AssetManager;
use common::math::{IVec2, Vec2};

pub mod common;
pub mod game_entity;
pub mod gui;
pub mod location;
pub mod message;
pub mod renderer;

use game_entity::*;
use gui::Gui;
use location::{
    physics_scene::{PhysicsScene, PhysicsSimulated},
    Location,
};
use renderer::{camera::Camera, Renderer};

use self::gui::with_gui::WithGui;

pub const TICK_PERIOD: f32 = 1.0;

pub struct Game {
    gui: Gui,
    renderer: Renderer,
    asset_manager: AssetManager,
    location: Location,

    from_last_tick: f32,
    tick_id: u32,
    frame_time: f32,
    avg_frame_time: f32,
    frames_times_collected: u32,
}

impl Game {
    pub fn new(context: &mut Context) -> Game {
        let mut asset_manager = AssetManager::new();

        asset_manager.load_assets(context);

        let drawable_size = graphics::drawable_size(context);
        let res = IVec2::new(drawable_size.0 as isize, drawable_size.1 as isize);
        let camera = Camera::new(res);
        let renderer = Renderer::new(camera);

        let location = Location::new(&asset_manager);

        Game {
            gui: Gui::new(context),
            location,
            asset_manager,
            renderer,

            from_last_tick: 0.0,
            tick_id: 0,
            frame_time: 0.0,
            avg_frame_time: 0.0,
            frames_times_collected: 0,
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

        self.frame_time += delta_time;
        self.frames_times_collected += 1;

        self.from_last_tick += delta_time;
        if self.from_last_tick > TICK_PERIOD {
            self.from_last_tick -= TICK_PERIOD;
            self.tick_all();
            self.tick_id += 1;

            self.avg_frame_time = self.frame_time / (self.frames_times_collected as f32);
            self.frame_time = 0.0;
            self.frames_times_collected = 0;
        }

        let update_parameters = UpdateParameters {
            delta_time,
            from_last_tick: self.from_last_tick,
            last_tick_id: self.tick_id,
        };

        self.location.process_keyboard_input(context);

        for _ in 0..2 {
            let hierarchy = self.location.get_bodies();
            let mut scene = PhysicsScene::new(hierarchy);
            let messages = scene.simulate(delta_time / 2.0);
            self.location.handle_physics_messages(messages);
            self.location.physics_update(delta_time / 2.0);
        }

        self.update_all(&update_parameters);

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, Color::WHITE);

        self.render_all();
        self.renderer.render_to_screen(context, &self.asset_manager);
        self.gui
            .render(context, &self.asset_manager, 1.0, |mut params| {
                self.location.render_gui(&mut params);

                imgui::Window::new("debug info")
                    .size([200.0, 100.0], imgui::Condition::Always)
                    .position_pivot([1.0, 0.0])
                    .position([params.screen_size.x, 0.0], imgui::Condition::Always)
                    .flags(imgui::WindowFlags::NO_RESIZE | imgui::WindowFlags::NO_COLLAPSE)
                    .build(&params.ui, || {
                        params.ui.text(format!("tick id: {}", self.tick_id));
                        params
                            .ui
                            .text(format!("from last tick: {}", self.from_last_tick));
                        params
                            .ui
                            .text(format!("avg frame time: {}", self.avg_frame_time));
                        params
                            .ui
                            .text(format!("avg fps: {}", 1.0 / self.avg_frame_time));
                    });
            });

        graphics::present(context)
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.gui.update_mouse_pos(x, y);
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.gui.update_mouse_down(button);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        self.gui.update_mouse_up(button);
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        _repeat: bool,
    ) {
        self.gui.update_key_down(keycode, keymods);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        self.gui.update_key_up(keycode, keymods);
    }

    fn text_input_event(&mut self, _ctx: &mut Context, val: char) {
        self.gui.update_text(val);
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, width, height))
            .unwrap();
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        self.gui.update_scroll(x, y);
    }
}
