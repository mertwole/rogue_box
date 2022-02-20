use crate::game::common::math::Vec2;
use crate::game::game_entity::*;
use crate::game::hub::building::Building;
use crate::game::renderer::Renderer;
use super::surface::Surface;
use crate::game::field::message::*;
use crate::game::hub::location::surface::*;

use crate::game::physics_scene::message as physics_message;
use crate::game::physics_scene::{BodyCollection, PhysicsSimulated, Body, Collider, ColliderShape};

#[derive(Default)]
pub struct Cell {
    surface : Surface,
    building : Option<Box<dyn Building>>,
    body : Option<Body>
}

impl Cell {
    pub fn new(surface : Surface) -> Cell {
        Cell {
            surface,
            building : None,
            body : None
        }
    }

    pub fn build(&mut self, building : Box<dyn Building>) {
        self.building = Some(building);

        self.body = Some(
            Body::new_static(
                Collider::new(
                    ColliderShape::Box { size : Vec2::new(1.0, 1.0) }, Vec2::zero()
                ), 
                Vec2::zero()
            )
        );
    }
}

impl GameEntity for Cell {
    fn update(&mut self, parameters : &UpdateParameters) {
        self.building.as_mut()
        .map(|building| building.update(parameters));
    }

    fn tick(&mut self, tick_id : u32) {
        self.building.as_mut()
        .map(|building| building.tick(tick_id));
    }

    fn render(&mut self, renderer : &mut Renderer, transform : SpriteTransform) {
        self.surface.render(renderer, transform.clone());
        self.building.as_mut()
        .map(|building| building.render(renderer, transform));
    }
}

impl MessageReceiver for Cell {
    fn try_push_message(&mut self, message : Message) -> Option<Message> {
        match &mut self.building {
            Some(building) => { building.try_push_message(message) }
            None => { Some(message) }
        }
    }
}

impl MessageSender for Cell {
    fn pull_messages(&mut self, tick_id : u32) -> Vec<Message> {
        match &mut self.building {
            Some(building) => { building.pull_messages(tick_id) }
            None => { vec![] }
        }
    }

    fn message_send_result(&mut self, result : MessageSendResult) {
        match &mut self.building {
            Some(building) => { building.message_send_result(result); }
            None => { }
        }
    }
}

impl PhysicsSimulated for Cell {
    fn get_all_bodies(&mut self) -> BodyCollection {
        let mut bc = BodyCollection::new();
        if self.body.is_some() {
            bc.push(self.body.as_mut().unwrap());
        }
        bc
    }

    fn handle_physics_messages(&mut self, messages : Vec<physics_message::Message>) {

    }
}