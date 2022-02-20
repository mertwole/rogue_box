use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

extern crate include_dir;

use ggez::{graphics::Image, Context};
use include_dir::{include_dir, Dir, File};

const RESOURCES_DIR: Dir = include_dir!("./assets");

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct AssetId(u64);

impl AssetId {
    pub fn null() -> AssetId {
        AssetId(0)
    }
}

pub struct AssetManager {
    textures: HashMap<AssetId, Rc<Image>>,
    jsons: HashMap<AssetId, Rc<str>>,
}

// Loads all the textures and JSON files from ./resources dir and its subdirs.
impl AssetManager {
    pub fn new() -> AssetManager {
        AssetManager {
            textures: HashMap::new(),
            jsons: HashMap::new(),
        }
    }

    pub fn load_assets(&mut self, context: &mut Context) {
        self.load_all_assets_in_dir(context, &RESOURCES_DIR);
        log::info!("Loaded {} files", self.textures.len() + self.jsons.len());
    }

    fn load_all_assets_in_dir(&mut self, context: &mut Context, dir: &Dir) {
        for file in dir.files() {
            match file.path().extension() {
                Some(ext) => match ext.to_str().unwrap() {
                    "png" => {
                        self.load_texture(context, file);
                    }
                    "json" => {
                        self.load_json(file);
                    }
                    _ => {
                        continue;
                    }
                },
                None => {
                    continue;
                }
            }
        }

        for dir in dir.dirs() {
            self.load_all_assets_in_dir(context, dir);
        }
    }

    fn load_texture(&mut self, context: &mut Context, file: &File) {
        let id = Self::get_asset_id(file.path().to_str().unwrap());
        let texture_data = file.contents();
        let texture = Image::from_bytes(context, texture_data).unwrap();
        self.textures.insert(id, Rc::from(texture));
        log::info!("Loaded texture {}", file.path().to_str().unwrap());
    }

    fn load_json(&mut self, file: &File) {
        let contents = file.contents_utf8().unwrap();
        let id = Self::get_asset_id(file.path().to_str().unwrap());
        self.jsons.insert(id, Rc::from(contents));
        log::info!("Loaded JSON {}", file.path().to_str().unwrap());
    }

    pub fn get_asset_id(path: &str) -> AssetId {
        let mut hasher = DefaultHasher::new();
        path.hash(&mut hasher);
        AssetId(hasher.finish())
    }

    pub fn get_texture(&self, id: AssetId) -> Rc<Image> {
        match self.textures.get(&id) {
            Some(tex) => tex.clone(),
            None => {
                log::error!("Requested texture {:#034x} not found", id.0);
                self.get_texture(Self::get_asset_id("error_fallbacks/texture.png"))
            }
        }
    }

    pub fn get_json(&self, id: AssetId) -> Rc<str> {
        match self.jsons.get(&id) {
            Some(json) => json.clone(),
            None => {
                log::error!("Requested JSON file {:#034x} not found", id.0);
                self.get_json(Self::get_asset_id("error_fallbacks/json.json"))
            }
        }
    }
}
