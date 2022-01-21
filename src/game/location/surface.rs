use std::collections::{HashMap, hash_map::DefaultHasher};
use std::rc::Rc;
use std::hash::{Hash, Hasher};

use crate::game::renderer::Sprite;
use crate::common::asset_manager::AssetManager;
use crate::game::game_entity::*;

#[derive(Clone)]
pub struct Surface {
    id : SurfaceId,
    sprite : Sprite
}

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct SurfaceId(u64);

impl Surface {
    fn new_error() -> Surface {
        let texture = AssetManager::get_asset_id("error_fallbacks/texture.png");
        let sprite = Sprite::new(texture);
        Surface {
            id : SurfaceId(0),
            sprite
        }
    }

    fn from_json_value(value : &serde_json::Value) -> Surface {
        match value {
            serde_json::Value::Object(surface_obj) => {
                let err_name = &String::from("error");
                let name = match surface_obj.get("name") {
                    Some(serde_json::Value::String(name)) => { name }
                    _ => { 
                        log::error!("Item dictionary haven't been succesfully loaded : wrong JSON file structure(wrong item format)");
                        &err_name
                    }
                };

                let err_tex_path = &String::from("error_fallbacks/texture.png");
                let tex_path = match surface_obj.get("texture") {
                    Some(serde_json::Value::String(path)) => { path }
                    _ => { 
                        log::error!("Item dictionary haven't been succesfully loaded : wrong JSON file structure(wrong item format)");
                        &err_tex_path
                    }
                };

                let sprite = Sprite::new(AssetManager::get_asset_id(tex_path.as_str()));
                Surface {
                    id : SurfaceFactory::get_surface_id_by_name(name),
                    sprite
                }
            }
            _ => { 
                log::error!("Surface dictionary haven't been succesfully loaded : wrong JSON file structure"); 
                Self::new_error()
            }
        }
    }
}

impl GameEntity for Surface {
    fn update(&mut self, parameters : &UpdateParameters) {

    }

    fn tick(&mut self, tick_id : u32) {

    }

    fn render(&mut self, renderer : &mut Renderer, transform : SpriteTransform) {
        renderer.queue_render_sprite(self.sprite.clone(), transform);
    }
}

pub struct SurfaceFactory {
    surfaces : HashMap<SurfaceId, Surface>
}

impl SurfaceFactory {
    pub fn new(json : Rc<str>) -> SurfaceFactory {
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
            _ => { log::error!("Surface dictionary haven't been succesfully loaded : wrong JSON file structure(the top-level must be an array)"); }
        }

        log::info!("{} surfaces are loaded", surfaces.len());
        SurfaceFactory { surfaces }
    }

    pub fn create_surface(&self, id : SurfaceId) -> Surface {
        match self.surfaces.get(&id) {
            Some(surface) => { surface.clone() }
            None => {
                log::error!("There's no such item {:#034x}", id.0);
                Surface::new_error()
            }
        }
    }

    pub fn get_surface_id_by_name(name : &str) -> SurfaceId {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        SurfaceId(hasher.finish())
    }
}