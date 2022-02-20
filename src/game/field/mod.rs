use crate::game::common::direction::Direction;
use crate::game::common::math::{IVec2, Math};
use crate::game::game_entity::*;
use crate::game::physics_scene::message as physics_message;
use crate::game::physics_scene::{BodyCollection, PhysicsSimulated};
use crate::game::renderer::Renderer;

pub mod message;
use message::*;

pub trait FieldCell:
    Default + GameEntity + MessageSender + MessageReceiver + PhysicsSimulated
{
}

impl<T> FieldCell for T where
    T: Default + GameEntity + MessageSender + MessageReceiver + PhysicsSimulated
{
}

pub struct Field<T>
where
    T: FieldCell,
{
    min_coord: IVec2,
    max_coord: IVec2,

    cells: Vec<Vec<T>>,
}

impl<T> Field<T>
where
    T: FieldCell,
{
    pub fn new(min_coord: IVec2, max_coord: IVec2) -> Field<T> {
        let x_count = (max_coord.x - min_coord.x) as usize;
        let y_count = (max_coord.y - min_coord.y) as usize;
        let mut cells = Vec::with_capacity(x_count);
        for _ in 0..x_count {
            let mut cells_row = Vec::with_capacity(y_count);
            for _ in 0..y_count {
                cells_row.push(T::default());
            }
            cells.push(cells_row);
        }

        Field {
            min_coord,
            max_coord,
            cells,
        }
    }

    pub fn get_cell_mut(&mut self, coords: IVec2) -> Option<&mut T> {
        if coords.x >= self.min_coord.x
            && coords.x <= self.max_coord.x
            && coords.y >= self.min_coord.y
            && coords.y <= self.max_coord.y
        {
            return Some(self.get_cell_mut_unchecked(coords));
        }
        None
    }

    fn get_cell_mut_unchecked(&mut self, coords: IVec2) -> &mut T {
        let arr_coords = coords - self.min_coord;
        &mut self.cells[arr_coords.x as usize][arr_coords.y as usize]
    }

    fn push_message_back(&mut self, message: Message) {
        let sender = self.get_cell_mut_unchecked(message.sender.get_position());
        let result = MessageSendResult {
            tick_id: message.tick_id,
            message_id: message.id,
            message: Some(message),
        };
        sender.message_send_result(result);
    }

    fn try_process_message(&mut self, message: Message) -> Option<Message> {
        let receiver_cell = self.get_cell_mut(message.receiver.get_position());
        if receiver_cell.is_none() {
            return Some(message);
        }
        let sender_pos = message.sender.get_position();

        let result = MessageSendResult {
            message_id: message.id,
            tick_id: message.tick_id,
            message: None,
        };

        let back_message = receiver_cell.unwrap().try_push_message(message);
        if back_message.is_none() {
            let sender = self.get_cell_mut_unchecked(sender_pos);
            sender.message_send_result(result);
            return None;
        }

        back_message
    }

    fn process_message(&mut self, mut message: Message) {
        let receivers: Vec<MessageExchangeActor> = match &message.target {
            Target::Direction(dir) => {
                vec![MessageExchangeActor::at_position(
                    message.sender.get_position() + dir.to_ivec2(),
                )]
            }
            Target::BroadcastNeighbors => vec![
                message.sender.get_position() + Direction::Up.to_ivec2(),
                message.sender.get_position() + Direction::Right.to_ivec2(),
                message.sender.get_position() + Direction::Down.to_ivec2(),
                message.sender.get_position() + Direction::Left.to_ivec2(),
            ]
            .into_iter()
            .map(MessageExchangeActor::at_position)
            .collect(),
            Target::ElectricInputs(inputs) => inputs
                .iter()
                .map(|(pos, port)| {
                    let mut actor = MessageExchangeActor::at_position(*pos);
                    actor.set_electric_port(*port);
                    actor
                })
                .collect(),
        };

        for receiver in receivers {
            message.receiver = receiver;
            message = match self.try_process_message(message) {
                Some(msg) => msg,
                None => {
                    return;
                }
            };
        }

        self.push_message_back(message);
    }
}

impl<T> GameEntity for Field<T>
where
    T: FieldCell,
{
    fn update(&mut self, parameters: &UpdateParameters) {
        for cell_column in &mut self.cells {
            for cell in cell_column {
                cell.update(parameters);
            }
        }
    }

    fn tick(&mut self, tick_id: u32) {
        for x in self.min_coord.x..self.max_coord.x {
            for y in self.min_coord.y..self.max_coord.y {
                let cell = self.get_cell_mut_unchecked(IVec2::new(x, y));
                cell.tick(tick_id);
            }
        }

        for x in self.min_coord.x..self.max_coord.x {
            for y in self.min_coord.y..self.max_coord.y {
                let cell = self.get_cell_mut_unchecked(IVec2::new(x, y));
                let messages = cell.pull_messages(tick_id);
                for (message_id, mut message) in messages.into_iter().enumerate() {
                    message.sender.set_position(IVec2::new(x, y));
                    message.id = message_id as u32;
                    self.process_message(message);
                }
            }
        }
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
    }
}

impl<T> PhysicsSimulated for Field<T>
where
    T: FieldCell,
{
    fn get_all_bodies(&mut self) -> BodyCollection {
        self.cells
            .iter_mut()
            .fold(BodyCollection::new(), |mut acc, x| {
                acc.append(x.iter_mut().fold(BodyCollection::new(), |mut acc, x| {
                    acc.append(x.get_all_bodies());
                    acc
                }));
                acc
            })
    }

    fn handle_physics_messages(&mut self, messages: Vec<physics_message::Message>) {}
}
