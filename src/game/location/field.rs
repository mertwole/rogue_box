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
                let arr_coords = coords - self.min_coord;
                return Some(&mut self.cells[arr_coords.x as usize][arr_coords.y as usize]);
            }
        } 
        None
    }

    pub fn message_exchange(&mut self, tick_id : u32) {
        let mut message_queue = Vec::new();

        for cell_row in &mut self.cells {
            for cell in cell_row {
                match &mut cell.building {
                    Some(building) => { 
                        message_queue.extend(building.pull_messages());
                    }
                    None => { }
                }
            }
        }

        for message in &mut message_queue { message.tick_id = tick_id; }

        loop {
            let message = message_queue.pop();
            if message.is_none() { break; }
            let message = message.unwrap();
        
            let cell = self.get_cell_mut(message.receiver);
            match cell {
                Some(cell) => {
                    match &mut cell.building {
                        Some(building) => { 
                            let back_message = building.try_push_message(message);
                            match back_message {
                                Some(back_message) => {
                                    let sender = self.get_cell_mut(back_message.sender).unwrap().building.as_mut().unwrap();
                                    sender.push_back_message(back_message);
                                    continue;
                                }
                                None => { continue; }
                            }
                        }
                        None => {  }
                    }
                }
                None => {  }
            }

            let sender = self.get_cell_mut(message.sender).unwrap().building.as_mut().unwrap();
            sender.push_back_message(message);
            
        }
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
        for cell_row in &mut self.cells {
            for cell in cell_row {
                cell.tick(tick_id);
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