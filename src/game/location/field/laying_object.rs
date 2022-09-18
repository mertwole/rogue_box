use crate::game::common::asset_manager::AssetManager;
use crate::game::common::math::Vec2;
use crate::game::game_entity::*;
use crate::game::location::physics_scene::{message::MessageHierarchy, BodyCollection, *};
use crate::game::renderer::Sprite;

pub struct LayingObject {
    sprite: Sprite,
    pub body: Body,
}

impl LayingObject {
    pub fn new(position: Vec2) -> LayingObject {
        let tex = AssetManager::get_asset_id(&format!(
            "textures/{}.png",
            rand::random::<u32>() % 23 + 13
        ));
        let sprite = Sprite::new(tex);

        let collider = Collider::new(
            ColliderShape::Box {
                size: Vec2::new_xy(0.5),
            },
            Vec2::zero(),
        );
        let body = Body::new_dynamic(collider, 1.0, position);

        LayingObject { sprite, body }
    }
}

impl GameEntity for LayingObject {
    fn update(&mut self, parameters: &UpdateParameters) {}

    fn tick(&mut self, tick_id: u32) {}

    fn render(&mut self, renderer: &mut Renderer, transform: SpriteTransform) {
        renderer.queue_render_sprite(
            self.sprite.clone(),
            transform.add_translation(self.body.get_position()),
        )
    }
}

impl PhysicsSimulated for LayingObject {
    fn get_bodies(&mut self) -> BodyHierarchyRoot {
        let mut hierarchy = BodyHierarchyRoot::default();
        hierarchy.add_body(&mut self.body);
        hierarchy
    }

    fn handle_physics_messages(&mut self, messages: MessageHierarchy) {}

    fn physics_update(&mut self, delta_time: f32) {}
}
