use crate::common::math::IVec2;
use crate::game::game_entity::GameEntity;
use crate::game::renderer::Renderer;

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
}

impl GameEntity for Field {
    fn update(&mut self, delta_time : f32) {
        for cell_row in &mut self.cells {
            for cell in cell_row {
                cell.update(delta_time);
            }
        }
    }

    fn tick(&mut self) {
        for cell_row in &mut self.cells {
            for cell in cell_row {
                cell.tick();
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