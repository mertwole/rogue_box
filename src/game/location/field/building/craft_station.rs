use super::*;

pub struct CraftStation {}

impl GameEntity for CraftStation {
    fn update(&mut self, _parameters: &UpdateParameters) {}
    fn tick(&mut self, _tick_id: u32) {}
    fn render(&mut self, _renderer: &mut Renderer, _transform: SpriteTransform) {}
}

impl WithGui for CraftStation {
    fn render_gui(&mut self, ui: &Ui, screen_size: Vec2) {
        imgui::Window::new("craft")
            .size([200.0, 100.0], imgui::Condition::Always)
            .position([0.0, 0.0], imgui::Condition::Always)
            .build(&ui, || {
                ui.text("test");
            });
    }
}

impl BuildingClone for CraftStation {
    fn clone_box(&self) -> Box<dyn Building> {
        Box::from(CraftStation::new())
    }
}

impl Building for CraftStation {
    fn get_name(&self) -> &str {
        "error"
    }
}

impl CraftStation {
    pub fn new() -> CraftStation {
        CraftStation {}
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
