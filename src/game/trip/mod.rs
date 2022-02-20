use crate::game::game_entity::*;
use crate::game::player::Player;

mod location;
use location::*;

pub struct Trip {
    location: Location,
}

impl Trip {
    pub fn new() -> Trip {
        Trip {
            location: Location::new(),
        }
    }

    pub fn set_player(&mut self, player: Player) {
        self.location.set_player(player);
    }

    pub fn take_player(&mut self) -> Player {
        self.location.take_player()
    }
}

impl GameEntity for Trip {
    fn update(&mut self, parameters: &UpdateParameters) {
        self.location.update(parameters);
    }

    fn tick(&mut self, tick_id: u32) {
        self.location.tick(tick_id);
    }

    fn render(&mut self, renderer: &mut Renderer, transform: SpriteTransform) {
        self.location.render(renderer, transform);
    }
}
