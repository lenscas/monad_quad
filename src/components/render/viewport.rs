use macroquad::{
    prelude::{set_camera, set_default_camera, vec2, Camera2D, Rect, Vec2, BLACK, WHITE},
    texture::{draw_texture_ex, render_target, DrawTextureParams, RenderTarget},
    window::{clear_background, screen_height, screen_width},
};

use crate::Component;

pub struct Viewport<Child> {
    child: Child,
    size: Vec2,
    camera: Camera2D,
    render_target: RenderTarget,
}

pub struct ScreenSizeConfig<Child> {
    pub child: Child,
    pub size: Vec2,
}

impl<Child> Viewport<Child> {
    pub fn new<T>(size: Vec2, child: Child) -> Self
    where
        Child: Component<T>,
        Self: Sized,
    {
        Self::instantiate(ScreenSizeConfig { child, size })
    }
}

impl<T, Child: Component<T>> Component<T> for Viewport<Child> {
    type Input = ScreenSizeConfig<Child>;

    fn instantiate(input: Self::Input) -> Self
    where
        Self: Sized,
    {
        let render_target = render_target(input.size.x as u32, input.size.y as u32);
        render_target
            .texture
            .set_filter(macroquad::texture::FilterMode::Linear);
        let mut render_target_cam =
            Camera2D::from_display_rect(Rect::new(0., 0., input.size.x, input.size.y));
        render_target_cam.render_target = Some(render_target.clone());
        Self {
            child: input.child,
            size: input.size,
            camera: render_target_cam,
            render_target,
        }
    }

    fn process(&mut self, state: &mut T) {
        self.child.process(state);
    }

    fn render(&self, props: &T) {
        let scale = f32::min(screen_width() / self.size.x, screen_height() / self.size.y);
        set_camera(&self.camera);
        self.child.render(props);
        set_default_camera();
        clear_background(BLACK);
        draw_texture_ex(
            &self.render_target.texture,
            (screen_width() - (self.size.x * scale)) * 0.5,
            (screen_height() - (self.size.y * scale)) * 0.5,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.size.x * scale, self.size.y * scale)),
                flip_y: true, // Must flip y otherwise 'render_target' will be upside down
                ..Default::default()
            },
        )
    }

    fn ui(&mut self, ui: &mut macroquad::ui::Ui, state: &mut T) {
        //self.child.ui(ui, state)
        let scale = f32::min(screen_width() / self.size.x, screen_height() / self.size.y);
        set_camera(&self.camera);
        self.child.ui(ui, state);
        set_default_camera();
        clear_background(BLACK);
        draw_texture_ex(
            &self.render_target.texture,
            (screen_width() - (self.size.x * scale)) * 0.5,
            (screen_height() - (self.size.y * scale)) * 0.5,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.size.x * scale, self.size.y * scale)),
                flip_y: true, // Must flip y otherwise 'render_target' will be upside down
                ..Default::default()
            },
        )
    }
}
