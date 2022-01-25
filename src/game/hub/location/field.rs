use crate::game::common::math::IVec2;
use crate::game::common::direction::Direction;
use crate::game::game_entity::*;
use crate::game::renderer::Renderer;
use crate::game::hub::message::*;
use crate::game::hub::location::surface::*;
use crate::game::common::asset_manager::AssetManager;

use super::cell::Cell;

pub struct Field {
    min_coord : IVec2,
    max_coord : IVec2,

    cells : Vec<Vec<Cell>>
}

impl Field {
    pub fn new(min_coord : IVec2, max_coord : IVec2, asset_manager : &AssetManager) -> Field {
        let surface_json = AssetManager::get_asset_id("dictionaries/surfaces.json");
        let surface_dict = asset_manager.get_json(surface_json);
        let surface_factory = SurfaceFactory::new(surface_dict);
        let grass_surface_id = SurfaceFactory::get_surface_id_by_name("grass");
        let grass_surface = surface_factory.create_surface(grass_surface_id);

        let x_count = (max_coord.x - min_coord.x) as usize;
        let y_count = (max_coord.y - min_coord.y) as usize;
        let mut cells = Vec::with_capacity(x_count);
        for _ in 0..x_count {
            let mut cells_row = Vec::with_capacity(y_count);
            for _ in 0..y_count {
                cells_row.push(Cell::new(grass_surface.clone()));
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
        let sender = self.get_cell_mut_unchecked(message.sender.get_pos());
        let building = sender.building.as_mut().unwrap();

        let result = MessageSendResult { 
            tick_id : message.tick_id,  
            message_id : message.id,
            message : Some(message)
        };

        building.message_send_result(result);
    }

    fn try_process_message(&mut self, message : Message) -> Option<Message> {
        let receiver_cell = self.get_cell_mut(message.receiver.get_pos());

        if receiver_cell.is_none() { return Some(message); }
        let receiver_building = &mut receiver_cell.unwrap().building;

        if receiver_building.is_none() { return Some(message); }
        let receiver_building = receiver_building.as_mut().unwrap();

        let tick_id = message.tick_id;
        let sender_pos = message.sender.get_pos();
        let message_id = message.id;
        let back_message = receiver_building.try_push_message(message);
        if back_message.is_none() { 
            let result = MessageSendResult {
                message_id,
                tick_id : tick_id,
                message : None
            };
            let sender = self.get_cell_mut_unchecked(sender_pos);
            let sender_building = sender.building.as_mut().unwrap();
            sender_building.message_send_result(result);
            return None;
        }

        return back_message;
    }

    fn process_message(&mut self, mut message : Message) {
        let targets = 
        match message.target {
            Target::Direction(dir) => { 
                vec![message.sender.get_pos() + dir.to_ivec2()] 
            }
            Target::BroadcastNeighbors => { 
                vec![   message.sender.get_pos() + Direction::Up.to_ivec2(),
                        message.sender.get_pos() + Direction::Right.to_ivec2(),
                        message.sender.get_pos() + Direction::Down.to_ivec2(),
                        message.sender.get_pos() + Direction::Left.to_ivec2()]
            }
            Target::BroadcastAllConnectedElectricInputs => {
                let sender_cell = self.get_cell_mut_unchecked(message.sender.get_pos());
                let sender_building = sender_cell.building.as_ref().unwrap().as_ref();
                let mut connected = Vec::new();
                let sender_ports = sender_building.get_electric_ports();

                for port in sender_ports {
                    match port.as_output() {
                        Some(out) => { 
                            connected.append(&mut out.get_connected_inputs().clone());
                        }
                        None => { } 
                    }
                }

                connected
            }
        };

        for target in targets {
            message.receiver = MessageExchangeActor::AtPosition(target);
            message = 
            match self.try_process_message(message) {
                Some(msg) => { msg }
                None => { return; }
            }
        }

        self.push_message_back(message);
    }
}

impl GameEntity for Field {
    fn update(&mut self, parameters : &UpdateParameters) {
        for cell_column in &mut self.cells {
            for cell in cell_column {
                cell.update(parameters);
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
                        for (message_id, mut message) in messages.into_iter().enumerate() { 
                            message.sender = MessageExchangeActor::AtPosition(IVec2::new(x, y));
                            message.id = message_id as u32;
                            self.process_message(message); 
                        }
                    }
                    None => { }
                }
            }
        }
    }

    fn render(&mut self, renderer : &mut Renderer, transform : SpriteTransform) {
        for (x, cell_column) in self.cells.iter_mut().enumerate() {
            for (y, cell) in cell_column.iter_mut().enumerate() {
                let cell_pos = self.min_coord + IVec2::new(x as isize, y as isize);
                let cell_transform = SpriteTransform::default().add_translation(cell_pos.to_vec2());
                cell.render(renderer, transform.combine(&cell_transform));
            }
        }
    }
}