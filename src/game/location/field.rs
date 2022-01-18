use crate::common::math::IVec2;
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

    fn push_message_back(&mut self, message : Message, message_id : u32) {
        let sender = self.get_cell_mut_unchecked(message.sender);
        let building = sender.building.as_mut().unwrap();

        let result = MessageSendResult { 
            tick_id : message.tick_id,  
            message_id,
            message : Some(message)
        };

        building.message_send_result(result);
    }

    fn process_message(&mut self, message : Message, message_id : u32) {
        let receiver = self.get_cell_mut(message.receiver);

        if receiver.is_none() { 
            self.push_message_back(message, message_id);
            return;
        }
        let receiver_building = &mut receiver.unwrap().building;

        if receiver_building.is_none() { 
            self.push_message_back(message, message_id);
            return;
        }
        let building = receiver_building.as_mut().unwrap();

        let tick_id = message.tick_id;
        let sender_pos = message.sender;
        let back_message = building.try_push_message(message);
        if back_message.is_none() { 
            let result = MessageSendResult {
                message_id,
                tick_id : tick_id,
                message : None
            };
            let sender = self.get_cell_mut_unchecked(sender_pos);
            let sender_building = sender.building.as_mut().unwrap();
            sender_building.message_send_result(result);
            return;
        }
        let back_message = back_message.unwrap();
        let sender = self.get_cell_mut_unchecked(back_message.sender);
        let sender_building = sender.building.as_mut().unwrap();

        let result = MessageSendResult {
            message_id,
            tick_id : back_message.tick_id,
            message : Some(back_message)
        };

        sender_building.message_send_result(result);
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
                            self.process_message(message, message_id as u32); 
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