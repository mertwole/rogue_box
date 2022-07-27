use ggez::event::KeyCode;
use ggez::input::keyboard;
use ggez::Context;

use crate::game::common::asset_manager::AssetManager;
use crate::game::common::math::{Math, Vec2};
use crate::game::game_entity::*;
use crate::game::renderer::Sprite;

use crate::game::location::physics_scene::{
    collision_data::CollisionData, message as physics_message,
    message::MessageBody as PhysicsMessageBody, BodyCollection, *,
};

pub struct Player {
    sprite: Sprite,
    pub body: Body,

    speed: f32,
    friction: f32,
    acceleration_impact: f32,

    velocity: Vec2,
    direction: Vec2,
}

impl Player {
    pub fn new(position: Vec2) -> Player {
        let tex = AssetManager::get_asset_id("textures/character/test.png");
        let sprite = Sprite::new(tex);

        let collider = Collider::new(
            ColliderShape::Box {
                size: Vec2::new_xy(1.0),
            },
            Vec2::zero(),
        );
        let body = Body::new_kinematic(collider, 1.0, position);

        Player {
            sprite,
            body,

            speed: 10.0,
            friction: 2.0,
            acceleration_impact: 20.0,

            velocity: Vec2::zero(),
            direction: Vec2::zero(),
        }
    }

    pub fn process_keyboard_input(&mut self, context: &Context) {
        let mut dir = Vec2::zero();

        if keyboard::is_key_pressed(context, KeyCode::A) {
            dir = dir + Vec2::new(-1.0, 0.0);
        }
        if keyboard::is_key_pressed(context, KeyCode::D) {
            dir = dir + Vec2::new(1.0, 0.0);
        }
        if keyboard::is_key_pressed(context, KeyCode::W) {
            dir = dir + Vec2::new(0.0, 1.0);
        }
        if keyboard::is_key_pressed(context, KeyCode::S) {
            dir = dir + Vec2::new(0.0, -1.0);
        }

        self.direction = dir;
    }

    fn apply_movement(&mut self, delta_time: f32) {
        let direction_is_zero = Math::small_enought(self.direction.sqr_length());
        let acceleration = if direction_is_zero {
            Vec2::zero()
        } else {
            self.direction.normalized()
        };

        let movement = self.velocity * delta_time;
        self.body.set_position(self.body.get_position() + movement);

        self.velocity =
            self.velocity + acceleration * delta_time * self.acceleration_impact * self.speed;
        if self.velocity.sqr_length() > self.speed * self.speed {
            self.velocity = self.velocity.normalized() * self.speed;
        }

        if direction_is_zero {
            self.velocity = self.velocity * (1.0 - self.friction * self.speed * delta_time);
        }
    }
}

impl GameEntity for Player {
    fn update(&mut self, parameters: &UpdateParameters) {
        self.apply_movement(parameters.delta_time);
    }

    fn tick(&mut self, tick_id: u32) {}

    fn render(&mut self, renderer: &mut Renderer, mut transform: SpriteTransform) {
        transform = transform.add_translation(self.body.get_position());
        renderer.queue_render_sprite(self.sprite.clone(), transform);
    }
}

impl PhysicsSimulated for Player {
    fn get_bodies(&mut self) -> BodyHierarchyRoot {
        let mut bodies = BodyCollection::default();
        bodies.push(&mut self.body);
        BodyHierarchyRoot::new(vec![], bodies)
    }

    fn handle_physics_messages(&mut self, messages: physics_message::MessageHierarchy) {
        if let Some(messages) = messages.messages.get(&self.body.id) {
            for msg in messages {
                match &msg.body {
                    PhysicsMessageBody::Collided(data) => {
                        self.body
                            .set_position(self.body.get_position() + data.normal * data.depth);
                    }
                    _ => {}
                }
            }
        }
    }
}
