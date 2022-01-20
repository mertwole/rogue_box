use super::*;

struct ErrorBuilding { }

impl GameEntity for ErrorBuilding {
    fn update(&mut self, parameters : &UpdateParameters) { }
    fn tick(&mut self, tick_id : u32) { }
    fn render(&mut self, renderer : &mut Renderer) { }
}

impl BuildingClone for ErrorBuilding {
    fn clone_box(&self) -> Box<dyn Building> { Box::from(ErrorBuilding::new()) }
}

impl Building for ErrorBuilding {
    fn get_name(&self) -> &str { "error" }
}

impl ErrorBuilding {
    pub fn new() -> ErrorBuilding { ErrorBuilding { } }
}

impl MessageSender for ErrorBuilding {
    fn pull_messages(&mut self, tick_id : u32) -> Vec<Message> {
        Vec::new()
    }

    fn message_send_result(&mut self, result : MessageSendResult) { }
}

impl MessageReceiver for ErrorBuilding {
    fn try_push_message(&mut self, message : Message) -> Option<Message> {
        Some(message)
    }
}