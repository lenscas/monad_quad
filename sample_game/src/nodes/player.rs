use macroquad::prelude::{vec2, Vec2, GREEN};
use monad_quad::components::{
    render::{Rectangle, RectangleProps},
    Component, Context,
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
impl Component<&PlayerProps, &mut PlayerProps> for Player {
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

    fn process<'a>(
        &mut self,
        context: &Context,
        state: &'a mut PlayerProps,
    ) -> &'a mut PlayerProps {
        self.controls.process(context, &mut state.control);
        let new_x = state.control.dir.x + state.location.x;
        let new_y = state.control.dir.y + state.location.y;
        let viewport_size = context.viewport_size();
        state.location.x = new_x.max(0.).min(viewport_size.x);
        state.location.y = new_y.max(0.).min(viewport_size.y);
        state.control.dir = vec2(0., 0.);
        state
    }

    fn render(&self, context: &Context, props: &PlayerProps) {
        self.render.render(
            context,
            &RectangleProps {
                color: GREEN,
                location: props.location,
                size: props.size,
            },
        );
    }
}
