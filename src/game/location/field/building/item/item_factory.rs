use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use super::*;

pub struct ItemFactory {
    items: HashMap<ItemId, Item>,
}

impl ItemFactory {
    pub fn new(json: Rc<str>) -> ItemFactory {
        let mut items = HashMap::new();

        let items_arr = serde_json::from_str(json.as_ref()).unwrap_or_else(|e| {
            log::error!("Item dictionary haven't been succesfully loaded : {}", e);
            serde_json::Value::Array(Vec::new())
        });

        match items_arr {
            serde_json::Value::Array(arr) => {
                for item in arr {
                    let new_item = Item::from_json_value(&item);
                    items.insert(new_item.id, new_item);
                }
            }
            _ => {
                log::error!("Item dictionary haven't been succesfully loaded : wrong JSON file structure(the top-level must be an array)");
            }
        }

        log::info!("{} items are loaded", items.len());
        ItemFactory { items }
    }

    pub fn create_item(&self, id: ItemId) -> Item {
        match self.items.get(&id) {
            Some(item) => item.clone(),
            None => {
                log::error!("There's no such item {:#034x}", id.0);
                Item::new_error()
            }
        }
    }

    pub fn get_item_id_by_name(name: &str) -> ItemId {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        ItemId(hasher.finish())
    }
}
