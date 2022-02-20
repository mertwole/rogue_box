use ggez::mint::{Point2, Vector2};
use ggez::{graphics, Context};

use crate::game::common::asset_manager::AssetManager;
use crate::game::common::math::{Rect, Vec2};

pub mod camera;
mod sprite;
pub mod sprite_transform;

use camera::*;
pub use sprite::Sprite;
pub use sprite_transform::SpriteTransform;

pub struct Renderer {
    queued_sprites: Vec<(Sprite, SpriteTransform)>,
    camera: Camera,
}

impl Renderer {
    pub fn new(camera: Camera) -> Renderer {
        Renderer {
            queued_sprites: Vec::new(),
            camera,
        }
    }

    pub fn queue_render_sprite(&mut self, sprite: Sprite, transform: SpriteTransform) {
        self.queued_sprites.push((sprite, transform));
    }

    pub fn get_render_bounds(&self) -> Rect {
        self.camera.get_bounds()
    }

    pub fn render_to_screen(&mut self, context: &mut Context, asset_manager: &AssetManager) {
        for (sprite, transform) in &self.queued_sprites {
            let tex = asset_manager.get_texture(sprite.texture);

            let transform = transform.combine(&sprite.local_transform);

            let screen_space_pos = self.camera.world_to_screen_space(transform.translation);

            let dest = Point2::from([screen_space_pos.x as f32, screen_space_pos.y as f32]);
            let rotation = transform.rotation;

            let tex_size = Vec2::new(tex.dimensions().w, tex.dimensions().h);
            let scale = transform.scale * (Vec2::new_xy(self.camera.get_cell_size()) / tex_size);

            let scale = Vector2::from([scale.x, scale.y]);
            let offset = Point2::from([0.5, 0.5]);

            graphics::draw(
                context,
                tex.as_ref(),
                graphics::DrawParam::new()
                    .dest(dest)
                    .rotation(rotation)
                    .offset(offset)
                    .scale(scale),
            )
            .unwrap();
        }

        self.queued_sprites.clear();
    }
}
