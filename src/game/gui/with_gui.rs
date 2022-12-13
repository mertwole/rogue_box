pub use super::GuiRenderParams;

pub trait WithGui {
    fn render_gui(&mut self, params: &mut GuiRenderParams);
}
