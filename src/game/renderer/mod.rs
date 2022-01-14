use ggez::{Context, graphics};
use ggez::mint::{Point2, Vector2};

use crate::common::math::Vec2;
use crate::common::asset_manager::AssetManager;

pub mod camera;
mod sprite;

use camera::*;
pub use sprite::Sprite;

pub struct Renderer {
    queued_sprites : Vec<Sprite>
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer { queued_sprites : Vec::new() }
    }

    pub fn queue_render_sprite(&mut self, sprite : &Sprite) {
        self.queued_sprites.push(sprite.clone());
    }

    pub fn render_to_screen(&mut self, context : &mut Context, asset_manager : &AssetManager, camera : &Camera) {
        for sprite in &self.queued_sprites {
            let tex = asset_manager.get_texture(sprite.texture);

            let screen_space_pos = camera.world_to_screen_space(sprite.position);

            let dest = Point2::from([screen_space_pos.x as f32, screen_space_pos.y as f32]);
            let rotation = sprite.rotation;

            let tex_size = Vec2::new(tex.dimensions().w, tex.dimensions().h);
            let scale = sprite.scale * (Vec2::new_xy(camera.get_cell_size()) / tex_size);

            let scale = Vector2::from([scale.x, scale.y]);
            let offset = Point2::from([0.5, 0.5]);

            graphics::draw(context, tex.as_ref(), 

            graphics::DrawParam::new()
            .dest(dest)
            .rotation(rotation)
            .offset(offset)
            .scale(scale)
            ).unwrap();
        }

        self.queued_sprites.clear();
    }
}