use ggez::event::{KeyCode, KeyMods, MouseButton};
use ggez::graphics::{self, BackendSpec};
use ggez::Context;

use gfx_core::{
    handle::RenderTargetView,
    memory::Typed,
    texture::{FilterMethod, SamplerInfo, WrapMode},
    Factory,
};

use imgui::*;
use imgui_gfx_renderer::*;

use std::collections::HashMap;
use std::time::Instant;

use super::common::asset_manager::{AssetId, AssetManager};

pub mod with_gui;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct MouseState {
    pos: (i32, i32),
    /// mouse buttons: (left, right, middle)
    pressed: (bool, bool, bool),
    wheel: f32,
    wheel_h: f32,
}

pub struct Gui {
    pub imgui: imgui::Context,
    pub renderer: Renderer<gfx_core::format::Rgba8, gfx_device_gl::Resources>,
    last_frame: Instant,
    mouse_state: MouseState,
    loaded_images: HashMap<AssetId, TextureId>,
}

pub struct GuiRenderParams<'a> {
    pub ui: &'a Ui<'a>,
    renderer: &'a mut Renderer<gfx_core::format::Rgba8, gfx_device_gl::Resources>,
    asset_manager: &'a AssetManager,
    loaded_images: &'a HashMap<AssetId, TextureId>,
    ctx: &'a mut Context,
}

impl GuiRenderParams<'_> {
    pub fn get_or_load_texture_id(&mut self, asset_id: AssetId) -> TextureId {
        match self.loaded_images.get(&asset_id) {
            Some(loaded) => *loaded,
            None => {
                let tex = self.asset_manager.get_texture(asset_id);
                let tex_view = tex.as_ref().get_raw_texture_view();
                let backend: ggez::graphics::GlBackendSpec =
                    ggez::conf::Conf::default().backend.into();
                let shader_resource = backend.raw_to_typed_shader_resource(tex_view);
                let sampler_info = SamplerInfo::new(FilterMethod::Bilinear, WrapMode::Clamp);
                let factory = graphics::gfx_objects(self.ctx).0;
                let sampler = factory.create_sampler(sampler_info);
                self.renderer.textures().insert((shader_resource, sampler))
            }
        }
    }
}

impl Gui {
    pub fn new(ctx: &mut Context) -> Self {
        let mut imgui = imgui::Context::create();
        let (factory, gfx_device, _, _, _) = graphics::gfx_objects(ctx);

        let shaders = {
            let version = gfx_device.get_info().shading_language;
            if version.is_embedded {
                if version.major >= 3 {
                    Shaders::GlSlEs300
                } else {
                    Shaders::GlSlEs100
                }
            } else if version.major >= 4 {
                Shaders::GlSl400
            } else if version.major >= 3 {
                Shaders::GlSl130
            } else {
                Shaders::GlSl110
            }
        };

        let renderer = Renderer::init(&mut imgui, &mut *factory, shaders).unwrap();

        {
            let io = imgui.io_mut();
            io[Key::Tab] = KeyCode::Tab as _;
            io[Key::LeftArrow] = KeyCode::Left as _;
            io[Key::RightArrow] = KeyCode::Right as _;
            io[Key::UpArrow] = KeyCode::Up as _;
            io[Key::DownArrow] = KeyCode::Down as _;
            io[Key::PageUp] = KeyCode::PageUp as _;
            io[Key::PageDown] = KeyCode::PageDown as _;
            io[Key::Home] = KeyCode::Home as _;
            io[Key::End] = KeyCode::End as _;
            io[Key::Insert] = KeyCode::Insert as _;
            io[Key::Delete] = KeyCode::Delete as _;
            io[Key::Backspace] = KeyCode::Back as _;
            io[Key::Space] = KeyCode::Space as _;
            io[Key::Enter] = KeyCode::Return as _;
            io[Key::Escape] = KeyCode::Escape as _;
            io[Key::KeyPadEnter] = KeyCode::NumpadEnter as _;
            io[Key::A] = KeyCode::A as _;
            io[Key::C] = KeyCode::C as _;
            io[Key::V] = KeyCode::V as _;
            io[Key::X] = KeyCode::X as _;
            io[Key::Y] = KeyCode::Y as _;
            io[Key::Z] = KeyCode::Z as _;
        }

        Self {
            imgui,
            renderer,
            last_frame: Instant::now(),
            mouse_state: MouseState::default(),
            loaded_images: HashMap::new(),
        }
    }

    pub fn render<F: FnOnce(GuiRenderParams) -> ()>(
        &mut self,
        ctx: &mut Context,
        asset_manager: &AssetManager,
        hidpi_factor: f32,
        render_fn: F,
    ) {
        self.update_mouse();

        let now = Instant::now();
        let delta = now - self.last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        self.last_frame = now;

        let (draw_width, draw_height) = graphics::drawable_size(ctx);
        self.imgui.io_mut().display_size = [draw_width, draw_height];
        self.imgui.io_mut().display_framebuffer_scale = [hidpi_factor, hidpi_factor];
        self.imgui.io_mut().delta_time = delta_s;

        let ui = self.imgui.frame();

        render_fn(GuiRenderParams {
            ui: &ui,
            renderer: &mut self.renderer,
            asset_manager,
            loaded_images: &self.loaded_images,
            ctx,
        });

        let (factory, _, encoder, _, render_target) = graphics::gfx_objects(ctx);
        let draw_data = ui.render();
        self.renderer
            .render(
                &mut *factory,
                encoder,
                &mut RenderTargetView::new(render_target.clone()),
                draw_data,
            )
            .unwrap();
    }

    fn update_mouse(&mut self) {
        self.imgui.io_mut().mouse_pos =
            [self.mouse_state.pos.0 as f32, self.mouse_state.pos.1 as f32];

        self.imgui.io_mut().mouse_down = [
            self.mouse_state.pressed.0,
            self.mouse_state.pressed.1,
            self.mouse_state.pressed.2,
            false,
            false,
        ];

        self.imgui.io_mut().mouse_wheel = self.mouse_state.wheel;
        self.mouse_state.wheel = 0.0;

        self.imgui.io_mut().mouse_wheel_h = self.mouse_state.wheel_h;
        self.mouse_state.wheel_h = 0.0;
    }

    pub fn update_mouse_pos(&mut self, x: f32, y: f32) {
        self.mouse_state.pos = (x as i32, y as i32);
    }

    pub fn update_mouse_down(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => self.mouse_state.pressed.0 = true,
            MouseButton::Right => self.mouse_state.pressed.1 = true,
            MouseButton::Middle => self.mouse_state.pressed.2 = true,
            _ => (),
        }
    }

    pub fn update_mouse_up(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => self.mouse_state.pressed.0 = false,
            MouseButton::Right => self.mouse_state.pressed.1 = false,
            MouseButton::Middle => self.mouse_state.pressed.2 = false,
            _ => (),
        }
    }

    pub fn update_key_down(&mut self, key: KeyCode, mods: KeyMods) {
        self.imgui.io_mut().key_shift = mods.contains(KeyMods::SHIFT);
        self.imgui.io_mut().key_ctrl = mods.contains(KeyMods::CTRL);
        self.imgui.io_mut().key_alt = mods.contains(KeyMods::ALT);
        self.imgui.io_mut().keys_down[key as usize] = true;
    }

    pub fn update_key_up(&mut self, key: KeyCode, mods: KeyMods) {
        if mods.contains(KeyMods::SHIFT) {
            self.imgui.io_mut().key_shift = false;
        }
        if mods.contains(KeyMods::CTRL) {
            self.imgui.io_mut().key_ctrl = false;
        }
        if mods.contains(KeyMods::ALT) {
            self.imgui.io_mut().key_alt = false;
        }
        self.imgui.io_mut().keys_down[key as usize] = false;
    }

    pub fn update_text(&mut self, val: char) {
        self.imgui.io_mut().add_input_character(val);
    }

    pub fn update_scroll(&mut self, x: f32, y: f32) {
        self.mouse_state.wheel += y;
        self.mouse_state.wheel_h += x;
    }
}
