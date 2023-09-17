use macroquad::{
    prelude::{mouse_position, vec2, Color, Vec2},
    shapes::draw_rectangle,
    text::draw_text,
    time::get_frame_time,
    window::clear_background,
};

pub struct Context {
    viewport_size: Vec2,
}

impl Context {
    pub fn new(viewport_size: Vec2) -> Self {
        Self { viewport_size }
    }
    pub fn viewport_size(&self) -> Vec2 {
        self.viewport_size
    }
    pub fn window_size(&self) -> Vec2 {
        vec2(
            macroquad::window::screen_width(),
            macroquad::window::screen_height(),
        )
    }
    pub fn get_scale(&self) -> f32 {
        let viewport = self.viewport_size();
        let window = self.window_size();
        f32::min(window.x / viewport.x, window.y / viewport.y)
    }
    pub fn get_mouse_location(&self) -> Vec2 {
        let mouse_pos = mouse_position();
        let scale = self.get_scale();
        let window = self.window_size();
        let viewport = self.viewport_size();
        Vec2 {
            x: (mouse_pos.0 - (window.x - (viewport.x * scale))) / scale,
            y: (mouse_pos.1 - (window.y - (viewport.y * scale))) / scale,
        }
    }
    pub fn clear_background(&self, color: Color) {
        clear_background(color)
    }
    pub fn draw_rectangle(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        draw_rectangle(x, y, w, h, color)
    }
    pub fn draw_text(&self, text: &str, x: f32, y: f32, font_size: f32, color: Color) {
        draw_text(text, x, y, font_size, color)
    }
    pub fn get_delta(&self) -> f32 {
        get_frame_time()
    }
    pub fn set_default_material(&self) {
        macroquad::material::gl_use_default_material()
    }
}
