use crate::game::{
    common::{
        direction::Direction,
        math::{IVec2, Math, Vec2},
    },
    game_entity::*,
    location::{
        physics_scene::{BodyCollection, BodyHierarchyRoot, PhysicsSimulated},
        player::Player,
    },
    message::*,
    renderer::Renderer,
};

use std::iter::once;

pub mod building;
pub mod cell;
pub mod laying_object;
pub mod message;

use cell::Cell;
use laying_object::LayingObject;

pub struct Field {
    min_coord: IVec2,
    max_coord: IVec2,
    cells: Vec<Vec<Cell>>,
    player: Player,
    laying_objects: Vec<LayingObject>,
}

impl Field {
    pub fn new(min_coord: IVec2, max_coord: IVec2) -> Field {
        let x_count = (max_coord.x - min_coord.x) as usize;
        let y_count = (max_coord.y - min_coord.y) as usize;
        let mut cells = Vec::with_capacity(x_count);
        for _ in 0..x_count {
            let mut cells_row = Vec::with_capacity(y_count);
            for _ in 0..y_count {
                cells_row.push(Cell::default());
            }
            cells.push(cells_row);
        }

        let test_laying_objects = vec![
            LayingObject::new(Vec2::new(5.0, 5.0), 100.0),
            LayingObject::new(Vec2::new(5.0, 4.0), 10.0),
            LayingObject::new(Vec2::new(5.0, 3.0), 5.0),
            LayingObject::new(Vec2::new(5.0, 2.0), 1.0),
            LayingObject::new(Vec2::new(5.0, 1.0), 0.1),
        ];

        Field {
            min_coord,
            max_coord,
            cells,
            player: Player::new(Vec2::new(2.5, 2.5)),
            laying_objects: test_laying_objects,
        }
    }

    pub fn get_cell_mut(&mut self, coords: IVec2) -> Option<&mut Cell> {
        if coords.x >= self.min_coord.x
            && coords.x <= self.max_coord.x
            && coords.y >= self.min_coord.y
            && coords.y <= self.max_coord.y
        {
            return Some(self.get_cell_mut_unchecked(coords));
        }
        None
    }

    fn get_cell_mut_unchecked(&mut self, coords: IVec2) -> &mut Cell {
        let arr_coords = coords - self.min_coord;
        &mut self.cells[arr_coords.x as usize][arr_coords.y as usize]
    }

    // TODO : IT'S DEBUG
    pub fn process_keyboard_input(&mut self, context: &ggez::Context) {
        self.player.process_keyboard_input(context);
    }
}

impl GameEntity for Field {
    fn update(&mut self, parameters: &UpdateParameters) {
        for cell_column in &mut self.cells {
            for cell in cell_column {
                cell.update(parameters);
            }
        }

        for laying_obj in &mut self.laying_objects {
            laying_obj.update(parameters);
        }

        self.player.update(parameters);
    }

    fn tick(&mut self, tick_id: u32) {
        for x in self.min_coord.x..self.max_coord.x {
            for y in self.min_coord.y..self.max_coord.y {
                let cell = self.get_cell_mut_unchecked(IVec2::new(x, y));
                cell.tick(tick_id);
            }
        }

        for laying_obj in &mut self.laying_objects {
            laying_obj.tick(tick_id);
        }

        self.player.tick(tick_id);
    }

    fn render(&mut self, renderer: &mut Renderer, transform: SpriteTransform) {
        let mut render_bounds = renderer.get_render_bounds();
        render_bounds.min = transform.reverse().apply(render_bounds.min);
        render_bounds.max = transform.reverse().apply(render_bounds.max);

        let mut min_visible_cell = render_bounds.min.to_ivec2() - IVec2::new(1, 1);
        min_visible_cell.x = Math::max(min_visible_cell.x, self.min_coord.x);
        min_visible_cell.y = Math::max(min_visible_cell.y, self.min_coord.y);

        let mut max_visible_cell = render_bounds.max.to_ivec2() + IVec2::new(1, 1);
        max_visible_cell.x = Math::min(max_visible_cell.x, self.max_coord.x);
        max_visible_cell.y = Math::min(max_visible_cell.y, self.max_coord.y);

        for x in min_visible_cell.x..max_visible_cell.x {
            for y in min_visible_cell.y..max_visible_cell.y {
                let cell_pos = IVec2::new(x as isize, y as isize);
                let cell_transform = SpriteTransform::default().add_translation(cell_pos.to_vec2());
                let cell_index = cell_pos - self.min_coord;
                let cell = &mut self.cells[cell_index.x as usize][cell_index.y as usize];
                cell.render(renderer, transform.combine(&cell_transform));
            }
        }

        for laying_obj in &mut self.laying_objects {
            laying_obj.render(renderer, transform.clone());
        }

        self.player.render(renderer, transform);
    }
}

impl MessageSender for Field {
    fn pull_messages(&mut self, tick_id: u32) -> Vec<Message> {
        let mut messages = vec![];
        for x in self.min_coord.x..self.max_coord.x {
            for y in self.min_coord.y..self.max_coord.y {
                let cell = self.get_cell_mut_unchecked(IVec2::new(x, y));
                let mut cell_messages = cell.pull_messages(tick_id);
                for msg in &mut cell_messages {
                    if let Message::FieldMessage(ref mut msg) = msg {
                        msg.sender.set_position(IVec2::new(x, y));
                    }
                }
                messages.append(&mut cell_messages);
            }
        }
        messages
    }
}

impl MessageReceiver for Field {
    fn try_push_message(&mut self, mut message: Message) -> Option<Message> {
        match message {
            Message::FieldMessage(ref mut msg) => {
                if msg.refund {
                    if let Some(refunded) = self
                        .get_cell_mut_unchecked(msg.sender.get_position())
                        .try_push_message(message)
                    {
                        log::error!("Couldn't process refund message");
                    }
                    return None;
                }

                let sender_pos = msg.sender.get_position();
                match msg.target {
                    field_message::Target::Directions(ref mut directions) => {
                        if directions.len() == 0 {
                            msg.refund = true;
                            return Some(message);
                        }

                        let dir = directions.pop().unwrap();
                        let receiver_pos = sender_pos + dir.to_ivec2();
                        msg.receiver.set_position(receiver_pos);
                        if let Some(receiver_cell) = self.get_cell_mut(receiver_pos) {
                            receiver_cell.try_push_message(message)
                        } else {
                            log::error!("Trying to access non-existing cell");
                            Some(message)
                        }
                    }
                }
            }
            _ => Some(message),
        }
    }
}

// TODO : Automatize in macro.
impl PhysicsSimulated for Field {
    fn get_bodies(&mut self) -> BodyHierarchyRoot {
        BodyHierarchyRoot::new(
            self.cells
                .iter_mut()
                .flatten()
                .map(|cell| cell.get_bodies())
                .chain(once(self.player.get_bodies()))
                .chain(self.laying_objects.iter_mut().map(|obj| obj.get_bodies()))
                .collect(),
            BodyCollection::default(),
        )
    }

    fn handle_physics_messages(&mut self, mut messages: physics_message::MessageHierarchy) {
        for laying_obj in self.laying_objects.iter_mut().rev() {
            laying_obj.handle_physics_messages(messages.nested.pop().unwrap());
        }
        self.player
            .handle_physics_messages(messages.nested.pop().unwrap());
        let mut cell_messages = messages.nested.into_iter().rev();
        for cell in self.into_iter() {
            cell.handle_physics_messages(cell_messages.next().unwrap());
        }
    }

    fn physics_update(&mut self, delta_time: f32) {
        self.into_iter()
            .for_each(|cell| cell.physics_update(delta_time));
        self.laying_objects
            .iter_mut()
            .for_each(|obj| obj.physics_update(delta_time));
        self.player.physics_update(delta_time);
    }
}

pub struct Iter<'a, Cell> {
    cells: &'a Vec<Vec<Cell>>,
    size: IVec2,
    curr: IVec2,
}

impl<'a, Cell> Iterator for Iter<'a, Cell> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.x >= self.size.x - 1 {
            if self.curr.y >= self.size.y - 1 {
                None
            } else {
                let cell = Some(&self.cells[self.curr.y as usize][self.curr.x as usize]);
                self.curr.y += 1;
                self.curr.x = 0;
                cell
            }
        } else {
            let cell = Some(&self.cells[self.curr.y as usize][self.curr.x as usize]);
            self.curr.x += 1;
            cell
        }
    }
}

impl<'a> IntoIterator for &'a Field {
    type Item = &'a Cell;
    type IntoIter = Iter<'a, Cell>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            cells: &self.cells,
            size: self.max_coord - self.min_coord,
            curr: IVec2::zero(),
        }
    }
}

pub struct IterMut<'a, Cell> {
    cells: &'a mut Vec<Vec<Cell>>,
    size: IVec2,
    curr: IVec2,
}

// TODO : remove unsafe
impl<'a, Cell> Iterator for IterMut<'a, Cell> {
    type Item = &'a mut Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.x >= self.size.x - 1 {
            if self.curr.y >= self.size.y - 1 {
                None
            } else {
                let cell = unsafe {
                    std::mem::transmute(&mut self.cells[self.curr.y as usize][self.curr.x as usize])
                };
                self.curr.y += 1;
                self.curr.x = 0;
                Some(cell)
            }
        } else {
            let cell = unsafe {
                std::mem::transmute(&mut self.cells[self.curr.y as usize][self.curr.x as usize])
            };
            self.curr.x += 1;
            Some(cell)
        }
    }
}

impl<'a> IntoIterator for &'a mut Field {
    type Item = &'a mut Cell;
    type IntoIter = IterMut<'a, Cell>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            cells: &mut self.cells,
            size: self.max_coord - self.min_coord,
            curr: IVec2::zero(),
        }
    }
}
