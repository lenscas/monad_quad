use macroquad::prelude::{vec2, Color, Vec2, RED};
use monad_quad::components::{
    render::{Rectangle, RectangleProps, Text, TextProperties},
    Component,
};

pub struct ScoreDisplayProperties {
    pub score: i64,
    pub location: Vec2,
    pub font_size: f32,
    pub color: Color,
    pub lives: u8,
    pub lives_size: f32,
    pub lives_location: Vec2,
}

pub struct ScoreDisplay {
    text: Text,
    lives: [Rectangle; 3],
}
impl Component<ScoreDisplayProperties> for ScoreDisplay {
    type Input = ();

    fn instantiate(_: Self::Input) -> Self
    where
        Self: Sized,
    {
        Self {
            text: Text,
            lives: [Rectangle, Rectangle, Rectangle],
        }
    }

    fn process(&mut self, _: &mut ScoreDisplayProperties) {}

    fn render(&self, props: &ScoreDisplayProperties) {
        self.text.render(&TextProperties {
            text: format!("Score: {}", props.score),
            location: props.location,
            font_size: props.font_size,
            color: props.color,
        });

        for k in 0..(props.lives.min(3)) {
            let life = &self.lives[usize::from(k)];
            life.render(&RectangleProps {
                size: vec2(props.lives_size, props.lives_size),
                color: RED,
                location: vec2(
                    props.lives_location.x + (k as f32 * (2. * props.lives_size)),
                    props.lives_location.y,
                ),
            })
        }
    }
    fn ui(&mut self, ui: &mut macroquad::ui::Ui, state: &mut ScoreDisplayProperties) {
        self.text.ui(
            ui,
            &mut TextProperties {
                text: format!("Score: {}", state.score),
                location: state.location,
                font_size: state.font_size,
                color: state.color,
            },
        );
        for k in 0..(state.lives.min(3)) {
            let life = &mut self.lives[usize::from(k)];
            life.ui(
                ui,
                &mut RectangleProps {
                    size: vec2(state.lives_size, state.lives_size),
                    color: RED,
                    location: vec2(
                        state.lives_location.x + (k as f32 * (2. * state.lives_size)),
                        state.lives_location.y,
                    ),
                },
            )
        }
    }
}
