pub use crate::game::common::math::Vec2;
pub use imgui::Ui;

pub trait WithGui {
    fn render_gui(&mut self, ui: &Ui, screen_size: Vec2);
}
