use crate::game::{gui::with_gui::*, location::field::building::item::Item};

#[derive(Default)]
enum Slot {
    #[default]
    Empty,
    Filled {
        item: Item,
        amount: usize,
    },
}

pub struct Inventory {
    slots: Vec<Slot>,
    slot_capacity: usize,
}

impl Inventory {
    pub fn new(slot_amount: usize, slot_capacity: usize) -> Inventory {
        Inventory {
            slots: std::iter::repeat(())
                .map(|_| Slot::default())
                .take(slot_amount)
                .collect(),
            slot_capacity,
        }
    }

    /// Returns remaining items(if there are).
    pub fn try_put_items(&mut self, item: Item, amount: usize) -> Option<(Item, usize)> {
        let item_to_put = item;
        let mut amount_to_put = amount;

        for slot in &mut self.slots {
            match slot {
                Slot::Empty => {
                    *slot = Slot::Filled {
                        item: item_to_put.clone(),
                        amount: amount_to_put.min(self.slot_capacity),
                    };
                    if amount_to_put > self.slot_capacity {
                        amount_to_put -= self.slot_capacity;
                    } else {
                        return None;
                    }
                }
                Slot::Filled { item, amount } if item.get_id() == item_to_put.get_id() => {
                    let prev_amount = *amount;
                    *amount = self.slot_capacity.min(*amount + amount_to_put);
                    if prev_amount + amount_to_put > self.slot_capacity {
                        amount_to_put -= self.slot_capacity - prev_amount;
                    } else {
                        return None;
                    }
                }
                _ => {}
            }
        }

        if amount_to_put > 0 {
            Some((item_to_put, amount_to_put))
        } else {
            None
        }
    }
}

impl WithGui for Inventory {
    fn render_gui(&mut self, params: &mut GuiRenderParams) {
        for i in 0..self.slots.len() {
            self.slots[i] = Slot::Filled {
                item: Item::new_error(),
                amount: i * 10,
            };
        }

        imgui::Window::new("inventory")
            .size([320.0, 50.0], imgui::Condition::Always)
            .no_decoration()
            .position(
                [params.screen_size.x * 0.5, params.screen_size.y],
                imgui::Condition::Always,
            )
            .position_pivot([0.5, 1.0])
            .build(&params.ui, || {
                params.ui.columns(self.slots.len() as i32, "slots", false);

                for slot in &self.slots {
                    match slot {
                        Slot::Filled { item, amount } => {
                            imgui::Image::new(
                                params.get_or_load_texture_id(item.get_sprite_asset_id()),
                                [20.0, 20.0],
                            )
                            .build(params.ui);
                            params.ui.text(format!("x{}", amount));
                        }
                        Slot::Empty => {
                            params.ui.invisible_button("", [20.0, 20.0]);
                        }
                    }

                    params.ui.next_column();
                }
            });
    }
}
