use super::*;

struct ErrorBuilding {}

impl GameEntity for ErrorBuilding {
    fn update(&mut self, _parameters: &UpdateParameters) {}
    fn tick(&mut self, _tick_id: u32) {}
    fn render(&mut self, _renderer: &mut Renderer, _transform: SpriteTransform) {}
}

impl WithGui for ErrorBuilding {
    fn render_gui(&mut self, ui: &Ui, screen_size: Vec2) {}
}

impl BuildingClone for ErrorBuilding {
    fn clone_box(&self) -> Box<dyn Building> {
        Box::from(ErrorBuilding::new())
    }
}

impl Building for ErrorBuilding {
    fn get_name(&self) -> &str {
        "error"
    }
}

impl ErrorBuilding {
    pub fn new() -> ErrorBuilding {
        ErrorBuilding {}
    }
}

impl MessageSender for ErrorBuilding {
    fn pull_messages(&mut self, _tick_id: u32) -> Vec<Message> {
        Vec::new()
    }
}

impl MessageReceiver for ErrorBuilding {
    fn try_push_message(&mut self, message: Message) -> Option<Message> {
        Some(message)
    }
}
