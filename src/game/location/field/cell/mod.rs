use crate::game::common::math::Vec2;
use crate::game::game_entity::*;
use crate::game::renderer::Renderer;

use crate::game::{
    location::{
        field::building::Building,
        physics_scene::{
            message as physics_message, Body, BodyCollection, BodyHierarchyRoot, Collider,
            ColliderShape, PhysicsSimulated,
        },
    },
    message::*,
};

pub mod surface;
use surface::Surface;

#[derive(Default)]
pub struct Cell {
    surface: Surface,
    building: Option<Box<dyn Building>>,
    body: Option<Body>,
}

impl Cell {
    pub fn new(surface: Surface) -> Cell {
        Cell {
            surface,
            building: None,
            body: None,
        }
    }

    pub fn build(&mut self, building: Box<dyn Building>, center: Vec2) {
        self.building = Some(building);

        self.body = Some(Body::new_static(
            Collider::new(
                ColliderShape::Box {
                    size: Vec2::new(1.0, 1.0),
                },
                Vec2::zero(),
            ),
            center,
        ));
    }
}

impl GameEntity for Cell {
    fn update(&mut self, parameters: &UpdateParameters) {
        if let Some(building) = self.building.as_mut() {
            building.update(parameters);
        }
    }

    fn tick(&mut self, tick_id: u32) {
        if let Some(building) = self.building.as_mut() {
            building.tick(tick_id);
        }
    }

    fn render(&mut self, renderer: &mut Renderer, transform: SpriteTransform) {
        self.surface.render(renderer, transform.clone());
        if let Some(building) = self.building.as_mut() {
            building.render(renderer, transform);
        }
    }
}

impl MessageReceiver for Cell {
    fn try_push_message(&mut self, message: Message) -> Option<Message> {
        match &mut self.building {
            Some(building) => building.try_push_message(message),
            None => Some(message),
        }
    }
}

impl MessageSender for Cell {
    fn pull_messages(&mut self, tick_id: u32) -> Vec<Message> {
        match &mut self.building {
            Some(building) => building.pull_messages(tick_id),
            None => {
                vec![]
            }
        }
    }
}

impl PhysicsSimulated for Cell {
    fn get_bodies(&mut self) -> BodyHierarchyRoot {
        let mut bodies = BodyCollection::default();
        if self.body.is_some() {
            bodies.push(self.body.as_mut().unwrap());
        }
        BodyHierarchyRoot::new(vec![], bodies)
    }

    fn handle_physics_messages(&mut self, messages: physics_message::MessageHierarchy) {}

    fn physics_update(&mut self, delta_time: f32) {}
}
