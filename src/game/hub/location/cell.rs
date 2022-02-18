use crate::game::game_entity::*;
use crate::game::hub::building::Building;
use crate::game::renderer::Renderer;
use super::surface::Surface;
use crate::game::hub::message::*;
use crate::game::hub::location::surface::*;

#[derive(Default)]
pub struct Cell {
    surface : Surface,
    building : Option<Box<dyn Building>>
}

impl Cell {
    pub fn new(surface : Surface) -> Cell {
        Cell {
            surface,
            building : None
        }
    }

    pub fn build(&mut self, building : Box<dyn Building>) {
        self.building = Some(building);
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