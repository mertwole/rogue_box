pub use crate::game::renderer::Renderer;
pub use crate::game::renderer::SpriteTransform;

pub struct UpdateParameters {
    pub delta_time : f32,
    pub from_last_tick : f32,
    pub last_tick_id : u32
}

pub trait GameEntity { 
    fn update(&mut self, parameters : &UpdateParameters);
    fn tick(&mut self, tick_id : u32);
    fn render(&mut self, renderer : &mut Renderer, transform : SpriteTransform);
}