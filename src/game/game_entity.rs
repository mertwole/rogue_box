pub use crate::game::renderer::Renderer;

pub trait GameEntity { 
    fn update(&mut self, delta_time : f32);
    fn tick(&mut self);
    fn render(&mut self, renderer : &mut Renderer);
}