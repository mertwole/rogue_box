use crate::game::common::math::IVec2;
use crate::game::field::Field;
use crate::game::game_entity::*;
use crate::game::player::Player;

mod cell;
use cell::Cell;

pub struct Room {
    player: Option<Player>,
    field: Field<Cell>,
}

impl Room {
    pub fn new() -> Room {
        Room {
            player: None,
            field: Field::new(IVec2::new(-10, -10), IVec2::new(10, 10)),
        }
    }

    pub fn set_player(&mut self, player: Player) {
        self.player = Some(player);
    }

    pub fn take_player(&mut self) -> Player {
        self.player.take().unwrap()
    }
}

impl GameEntity for Room {
    fn update(&mut self, parameters: &UpdateParameters) {}

    fn tick(&mut self, tick_id: u32) {}

    fn render(&mut self, renderer: &mut Renderer, transform: SpriteTransform) {}
}
