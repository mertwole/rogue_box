pub use crate::game::renderer::Renderer;

pub trait GameEntity { 
    fn update(&mut self, delta_time : f32);
    fn tick(&mut self, tick_id : u32);
    fn render(&mut self, renderer : &mut Renderer);
}