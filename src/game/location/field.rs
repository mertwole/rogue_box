use crate::common::math::IVec2;
use crate::common::direction::Direction;
use crate::game::game_entity::GameEntity;
use crate::game::renderer::Renderer;
use crate::game::message::*;

use super::cell::Cell;

pub struct Field {
    min_coord : IVec2,
    max_coord : IVec2,

    cells : Vec<Vec<Cell>>
}

impl Field {
    pub fn new(min_coord : IVec2, max_coord : IVec2) -> Field {
        let x_count = (max_coord.x - min_coord.x) as usize;
        let y_count = (max_coord.y - min_coord.y) as usize;
        let mut cells = Vec::with_capacity(x_count);
        for x in 0..x_count {
            let mut cells_row = Vec::with_capacity(y_count);
            for y in 0..y_count {
                cells_row.push(Cell::new(IVec2::new(x as isize, y as isize) + min_coord));
            }
            cells.push(cells_row);
        }

        Field { min_coord, max_coord, cells }
    }

    pub fn get_cell_mut(&mut self, coords : IVec2) -> Option<&mut Cell> {
        if coords.x >= self.min_coord.x && coords.x <= self.max_coord.x {
            if coords.y >= self.min_coord.y && coords.y <= self.max_coord.y {
                return Some(self.get_cell_mut_unchecked(coords));
            }
        } 
        None
    }

    fn get_cell_mut_unchecked(&mut self, coords : IVec2) -> &mut Cell {
        let arr_coords = coords - self.min_coord;
        &mut self.cells[arr_coords.x as usize][arr_coords.y as usize]
    }

    fn push_message_back(&mut self, message : Message) {
        let sender = self.get_cell_mut_unchecked(message.sender);
        let building = sender.building.as_mut().unwrap();

        let result = MessageSendResult { 
            tick_id : message.tick_id,  
            message_id : message.id,
            message : Some(message),
            computed_receiver : None
        };

        building.message_send_result(result);
    }

    fn try_process_message(&mut self, message : Message, receiver : IVec2) -> Option<Message> {
        let receiver_cell = self.get_cell_mut(receiver);

        if receiver_cell.is_none() { return Some(message); }
        let receiver_building = &mut receiver_cell.unwrap().building;

        if receiver_building.is_none() { return Some(message); }
        let receiver_building = receiver_building.as_mut().unwrap();

        let tick_id = message.tick_id;
        let sender_pos = message.sender;
        let message_id = message.id;
        let back_message = receiver_building.try_push_message(message);
        if back_message.is_none() { 
            let result = MessageSendResult {
                message_id,
                tick_id : tick_id,
                message : None,
                computed_receiver : Some(receiver)
            };
            let sender = self.get_cell_mut_unchecked(sender_pos);
            let sender_building = sender.building.as_mut().unwrap();
            sender_building.message_send_result(result);
            return None;
        }

        return back_message;
    }

    fn process_message(&mut self, mut message : Message) {
        let receivers = 
        match message.receiver {
            Receiver::Direction(dir) => { 
                vec![message.sender + dir.to_ivec2()] 
            }
            Receiver::Broadcast => { 
                vec![   message.sender + Direction::Up.to_ivec2(),
                        message.sender + Direction::Right.to_ivec2(),
                        message.sender + Direction::Down.to_ivec2(),
                        message.sender + Direction::Left.to_ivec2()]
            }
        };

        for receiver in receivers {
            message = 
            match self.try_process_message(message, receiver) {
                Some(msg) => { msg }
                None => { return; }
            }
        }

        self.push_message_back(message);
    }
}

impl GameEntity for Field {
    fn update(&mut self, delta_time : f32) {
        for cell_row in &mut self.cells {
            for cell in cell_row {
                cell.update(delta_time);
            }
        }
    }

    fn tick(&mut self, tick_id : u32) {
        for x in self.min_coord.x..self.max_coord.x {
            for y in self.min_coord.y..self.max_coord.y {
                let cell = self.get_cell_mut_unchecked(IVec2::new(x, y));
                cell.tick(tick_id);
            }
        }

        for x in self.min_coord.x..self.max_coord.x {
            for y in self.min_coord.y..self.max_coord.y {
                let cell = self.get_cell_mut_unchecked(IVec2::new(x, y));
                match &mut cell.building {
                    Some(building) => { 
                        let messages = building.pull_messages(tick_id);
                        for (message_id, message) in messages.into_iter().enumerate() { 
                            self.process_message(message); 
                        }
                    }
                    None => { }
                }
            }
        }
    }

    fn render(&mut self, renderer : &mut Renderer) {
        for cell_row in &mut self.cells {
            for cell in cell_row {
                cell.render(renderer);
            }
        }
    }
}