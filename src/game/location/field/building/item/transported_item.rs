use super::*;

#[derive(Clone)]
struct ItemMovement {
    from: Vec2,
    to: Vec2,
    tick_id: u32,
}

#[derive(Clone)]
pub struct TransportedItem {
    item: Item,
    // Item can be moved by transport belts only once per tick.
    pub(in crate::game::location::field::building) last_tick_moved: u32,
    movement: Option<ItemMovement>,
}

impl TransportedItem {
    pub fn new(item: Item) -> TransportedItem {
        TransportedItem {
            item,
            last_tick_moved: std::u32::MAX,
            movement: None,
        }
    }

    pub fn get_id(&self) -> ItemId {
        self.item.get_id()
    }

    pub fn set_movement(&mut self, from: Vec2, to: Vec2, tick_id: u32) {
        self.movement = Some(ItemMovement { from, to, tick_id });
    }
}

impl GameEntity for TransportedItem {
    fn update(&mut self, parameters: &UpdateParameters) {
        match &self.movement {
            None => {
                self.item.sprite.local_transform.translation = Vec2::zero();
            }
            Some(movement) => {
                if movement.tick_id + 1 == parameters.last_tick_id {
                    let interpolation = parameters.from_last_tick / crate::game::TICK_PERIOD;
                    self.item.sprite.local_transform.translation =
                        movement.from + (movement.to - movement.from) * interpolation;
                } else {
                    self.item.sprite.local_transform.translation = movement.to;
                }
            }
        }
    }

    fn tick(&mut self, tick_id: u32) {}

    fn render(&mut self, renderer: &mut Renderer, transform: SpriteTransform) {
        renderer.queue_render_sprite(self.item.sprite.clone(), transform);
    }
}
