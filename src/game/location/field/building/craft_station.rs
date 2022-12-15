use crate::game::{
    common::{
        asset_manager::{AssetId, AssetManager},
        json_reader::JsonReader,
    },
    renderer::Sprite,
};

use super::{
    item::{Item, ItemFactory},
    *,
};

#[derive(Clone)]
struct CraftRecipe {
    inputs: Vec<(Item, usize)>,
    outputs: Vec<(Item, usize)>,
}

pub struct CraftStation {
    name: String,
    texture: AssetId,

    recipes: Vec<CraftRecipe>,
}

impl CraftStation {
    fn common_data_from_json_object(&mut self, obj: &serde_json::Value, error: &mut bool) {
        self.name = JsonReader::read_string(obj, "name", error);
        let tex_path = JsonReader::read_string(obj, "texture", error);
        self.texture = AssetManager::get_asset_id(&tex_path);
    }

    fn recipes_from_json_object(
        &mut self,
        item_factory: &ItemFactory,
        obj: &serde_json::Value,
        error: &mut bool,
    ) {
        let recipes = JsonReader::read_vec(obj, "recipes", error);
        self.recipes = recipes
            .into_iter()
            .map(|recipe_json| CraftRecipe {
                inputs: JsonReader::read_vec(&recipe_json, "inputs", error)
                    .into_iter()
                    .map(|input_json| {
                        (
                            item_factory.create_item(ItemFactory::get_item_id_by_name(
                                &JsonReader::read_string(&input_json, "item", error),
                            )),
                            JsonReader::read_i32(&input_json, "amount", error) as usize,
                        )
                    })
                    .collect(),
                outputs: JsonReader::read_vec(&recipe_json, "outputs", error)
                    .into_iter()
                    .map(|output_json| {
                        (
                            item_factory.create_item(ItemFactory::get_item_id_by_name(
                                &JsonReader::read_string(&output_json, "item", error),
                            )),
                            JsonReader::read_i32(&output_json, "amount", error) as usize,
                        )
                    })
                    .collect(),
            })
            .collect();
    }

    pub fn from_json_object(obj: &serde_json::Value, item_factory: &ItemFactory) -> CraftStation {
        let mut station = CraftStation {
            name: String::new(),
            texture: AssetId::null(),
            recipes: vec![],
        };

        let mut error = false;
        station.common_data_from_json_object(obj, &mut error);
        station.recipes_from_json_object(item_factory, obj, &mut error);

        station
    }
}

impl GameEntity for CraftStation {
    fn update(&mut self, _parameters: &UpdateParameters) {}

    fn tick(&mut self, _tick_id: u32) {}

    fn render(&mut self, renderer: &mut Renderer, transform: SpriteTransform) {
        let sprite = Sprite::new(self.texture);
        renderer.queue_render_sprite(sprite, transform);
    }
}

impl WithGui for CraftStation {
    fn render_gui(&mut self, params: &mut GuiRenderParams) {
        imgui::Window::new("craft")
            .position([0.0, 0.0], imgui::Condition::Always)
            .build(&params.ui, || {
                params.ui.text("recipes:");
                for recipe in &self.recipes {
                    params.ui.separator();

                    params.ui.button("craft");
                    params.ui.same_line();

                    let mut first = true;
                    for (item, amount) in &recipe.inputs {
                        if !first {
                            params.ui.text(" + ");
                            params.ui.same_line();
                        } else {
                            first = false;
                        }

                        imgui::Image::new(
                            params.get_or_load_texture_id(item.get_sprite_asset_id()),
                            [20.0, 20.0],
                        )
                        .build(params.ui);
                        params.ui.same_line();
                        params.ui.text(&format!(" x{} ", amount));
                        params.ui.same_line();
                    }

                    params.ui.text(" -> ");

                    let mut first = true;
                    for (item, amount) in &recipe.outputs {
                        params.ui.same_line();
                        if !first {
                            params.ui.text(" + ");
                            params.ui.same_line();
                        } else {
                            first = false;
                        }

                        imgui::Image::new(
                            params.get_or_load_texture_id(item.get_sprite_asset_id()),
                            [20.0, 20.0],
                        )
                        .build(params.ui);
                        params.ui.same_line();
                        params.ui.text(&format!(" x{} ", amount));
                    }
                }
            });
    }
}

impl BuildingClone for CraftStation {
    fn clone_box(&self) -> Box<dyn Building> {
        Box::from(CraftStation {
            name: self.name.clone(),
            texture: self.texture.clone(),
            recipes: self.recipes.clone(),
        })
    }
}

impl Building for CraftStation {
    fn get_name(&self) -> &str {
        "error"
    }
}

impl MessageSender for CraftStation {
    fn pull_messages(&mut self, _tick_id: u32) -> Vec<Message> {
        Vec::new()
    }
}

impl MessageReceiver for CraftStation {
    fn try_push_message(&mut self, message: Message) -> Option<Message> {
        Some(message)
    }
}
