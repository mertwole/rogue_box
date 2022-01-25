use super::*;

pub struct TransportedItem {
    pub(in crate::game::hub) item : Item,
    // Item can be moved by transport belts only once per tick.
    pub(in crate::game::hub) last_tick_moved : u32
}

impl TransportedItem {
    pub fn new(item : Item) -> TransportedItem {
        TransportedItem {
            item,
            last_tick_moved : std::u32::MAX
        }
    } 

    pub fn get_id(&self) -> ItemId {
        self.item.get_id()
    }
}