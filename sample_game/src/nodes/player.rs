use macroquad::{
    prelude::{vec2, Vec2, GREEN},
    window::{screen_height, screen_width},
};
use monad_quad::components::{
    render::{Rectangle, RectangleProps},
    Component,
};

use super::{ControlProps, Controls};

#[derive(Clone)]
pub struct PlayerProps {
    pub score: i64,
    pub location: Vec2,
    pub control: ControlProps,
    pub size: Vec2,
    pub lives: u8,
}

pub struct Player {
    render: Rectangle,
    controls: Controls,
}
impl Component<PlayerProps> for Player {
    type Input = ();

    fn instantiate(_: Self::Input) -> Self
    where
        Self: Sized,
    {
        Self {
            render: Rectangle,
            controls: Controls::instantiate(()),
        }
    }

    fn process(&mut self, state: &mut PlayerProps) {
        self.controls.process(&mut state.control);
        let new_x = state.control.dir.x + state.location.x;
        let new_y = state.control.dir.y + state.location.y;
        state.location.x = new_x.max(0.).min(screen_width());
        state.location.y = new_y.max(0.).min(screen_height());
        state.control.dir = vec2(0., 0.);
    }

    fn render(&self, props: &PlayerProps) {
        self.render.render(&RectangleProps {
            color: GREEN,
            location: props.location,
            size: props.size,
        });
    }
    fn ui(&mut self, ui: &mut macroquad::ui::Ui, state: &mut PlayerProps) {
        self.render.ui(
            ui,
            &mut RectangleProps {
                color: GREEN,
                location: state.location,
                size: state.size,
            },
        )
    }
}
