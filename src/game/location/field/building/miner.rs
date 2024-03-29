use std::collections::HashMap;

use crate::game::{
    common::{
        asset_manager::{AssetId, AssetManager},
        json_reader::JsonReader,
    },
    game_entity::GameEntity,
    gui::with_gui::*,
    location::field::{
        building::{item::ItemFactory, recycler::Recycler, Building, BuildingClone},
        cell::surface::*,
    },
    message::*,
    renderer::{Renderer, Sprite},
    SpriteTransform, UpdateParameters,
};

pub struct Miner {
    name: String,
    texture: AssetId,

    surface_recyclers: HashMap<SurfaceId, Box<Recycler>>,
    curr_recycler: Option<Box<Recycler>>,
}

impl GameEntity for Miner {
    fn update(&mut self, parameters: &UpdateParameters) {
        match &mut self.curr_recycler {
            Some(recycler) => {
                recycler.update(parameters);
            }
            None => {}
        }
    }

    fn tick(&mut self, tick_id: u32) {
        match &mut self.curr_recycler {
            Some(recycler) => {
                recycler.tick(tick_id);
            }
            None => {}
        }
    }

    fn render(&mut self, renderer: &mut Renderer, transform: SpriteTransform) {
        let sprite = Sprite::new(self.texture);
        renderer.queue_render_sprite(sprite, transform);
    }
}

impl WithGui for Miner {
    fn render_gui(&mut self, params: &mut GuiRenderParams) {}
}

impl BuildingClone for Miner {
    fn clone_box(&self) -> Box<dyn Building> {
        let mut surface_recyclers = HashMap::new();
        for (id, recycler) in &self.surface_recyclers {
            let cloned =
                unsafe { Box::from_raw(Box::into_raw(recycler.clone_box()) as *mut Recycler) };
            surface_recyclers.insert(*id, cloned);
        }

        let curr_recycler = match &self.curr_recycler {
            Some(recycler) => {
                let cloned =
                    unsafe { Box::from_raw(Box::into_raw(recycler.clone_box()) as *mut Recycler) };
                Some(cloned)
            }
            None => None,
        };

        Box::from(Miner {
            name: self.name.clone(),
            texture: self.texture,

            surface_recyclers,
            curr_recycler,
        })
    }
}

impl Building for Miner {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

impl Miner {
    pub fn from_json_object(obj: &serde_json::Value) -> Miner {
        let mut error = false;

        let name = JsonReader::read_string(obj, "name", &mut error);

        let tex_path = JsonReader::read_string(obj, "texture", &mut error);
        let texture = AssetManager::get_asset_id(&tex_path);

        let mut surface_recyclers = HashMap::new();
        let surfaces = JsonReader::read_vec(obj, "surfaces", &mut error);

        for surface_obj in surfaces {
            let surface_name = JsonReader::read_string(&surface_obj, "surface", &mut error);
            let recycler_obj = JsonReader::read_obj(&surface_obj, "recycler", &mut error);
            let recycler = Recycler::from_json_object(&recycler_obj);
            let surface_id = SurfaceFactory::get_surface_id_by_name(surface_name.as_str());
            surface_recyclers.insert(surface_id, Box::from(recycler));
        }

        if error {
            log::error!(
                "Failed to parse Miner from json ({})",
                if name.is_empty() {
                    "error loading name"
                } else {
                    &name
                }
            );
        } else {
            log::info!("Miner succesfully loaded({})", name);
        }

        Miner {
            name,
            texture,

            surface_recyclers,
            curr_recycler: None,
        }
    }

    pub fn init(&mut self, surface_id: SurfaceId, item_factory: &ItemFactory) {
        self.curr_recycler = match self.surface_recyclers.get(&surface_id) {
            Some(recycler) => {
                let mut cloned =
                    unsafe { Box::from_raw(Box::into_raw(recycler.clone_box()) as *mut Recycler) };
                (&mut *cloned).init_items(item_factory);
                Some(cloned)
            }
            None => None,
        }
    }
}

impl MessageSender for Miner {
    fn pull_messages(&mut self, tick_id: u32) -> Vec<Message> {
        match &mut self.curr_recycler {
            Some(recycler) => recycler.pull_messages(tick_id),
            None => {
                vec![]
            }
        }
    }
}

impl MessageReceiver for Miner {
    fn try_push_message(&mut self, message: Message) -> Option<Message> {
        match &mut self.curr_recycler {
            Some(recycler) => recycler.try_push_message(message),
            None => Some(message),
        }
    }
}
