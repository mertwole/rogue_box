use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use super::*;

pub struct SurfaceFactory {
    surfaces: HashMap<SurfaceId, Surface>,
}

impl SurfaceFactory {
    pub fn new(json: Rc<str>) -> SurfaceFactory {
        let mut surfaces = HashMap::new();

        let surfaces_arr = serde_json::from_str(json.as_ref()).unwrap_or_else(|e| {
            log::error!("Surface dictionary haven't been succesfully loaded : {}", e);
            serde_json::Value::Array(Vec::new())
        });

        match surfaces_arr {
            serde_json::Value::Array(arr) => {
                for item in arr {
                    let new_surface = Surface::from_json_value(&item);
                    surfaces.insert(new_surface.id, new_surface);
                }
            }
            _ => {
                log::error!("Surface dictionary haven't been succesfully loaded : wrong JSON file structure(the top-level must be an array)");
            }
        }

        log::info!("{} surfaces are loaded", surfaces.len());
        SurfaceFactory { surfaces }
    }

    pub fn create_surface(&self, id: SurfaceId) -> Surface {
        match self.surfaces.get(&id) {
            Some(surface) => surface.clone(),
            None => {
                log::error!("There's no such item {:#034x}", id.0);
                Surface::new_error()
            }
        }
    }

    pub fn get_surface_id_by_name(name: &str) -> SurfaceId {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        SurfaceId(hasher.finish())
    }
}
