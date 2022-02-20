use crate::game::game_entity::*;
use crate::game::player::Player;

mod room;

use room::Room;

pub struct Location {
    rooms : Vec<Room>,
    current_room : usize
}

impl Location {
    pub fn new() -> Location {
        Location {
            rooms : vec![],
            current_room : 0
        }
    }

    pub fn set_player(&mut self, player : Player) {
        self.rooms[self.current_room].set_player(player);  
    }

    pub fn take_player(&mut self) -> Player {
        self.rooms[self.current_room].take_player()
    }

    fn set_room(&mut self, id : usize) {
        let player = self.rooms[self.current_room].take_player();
        self.rooms[id].set_player(player);
        
        self.current_room = id;
    }
}

impl GameEntity for Location {
    fn update(&mut self, parameters : &UpdateParameters) {
        self.rooms[self.current_room].update(parameters);
    }

    fn tick(&mut self, tick_id : u32) {
        self.rooms[self.current_room].tick(tick_id);
    }

    fn render(&mut self, renderer : &mut Renderer, transform : SpriteTransform) {
        self.rooms[self.current_room].render(renderer, transform.clone());
    }
}