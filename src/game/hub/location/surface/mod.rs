use std::hash::Hash;

use crate::game::renderer::Sprite;
use crate::game::common::asset_manager::AssetManager;
use crate::game::game_entity::*;

mod surface_factory;
pub use surface_factory::*;

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