use macroquad::{
    prelude::{KeyCode, Vec2},
    time::get_frame_time,
};
use monad_quad::components::{events::KeyDown, Component, Context};

#[derive(Clone)]
pub struct ControlProps {
    pub left: KeyCode,
    pub right: KeyCode,
    pub up: KeyCode,
    pub down: KeyCode,
    pub speed: f32,
    pub dir: Vec2,
}

type BasicKeyDown = KeyDown<fn(&ControlProps) -> KeyCode, fn(bool, &mut ControlProps)>;

pub struct Controls {
    left: BasicKeyDown,
    right: BasicKeyDown,
    up: BasicKeyDown,
    down: BasicKeyDown,
}
impl Component<&ControlProps, &mut ControlProps> for Controls {
    type Input = ();

    fn instantiate(_: Self::Input) -> Self
    where
        Self: Sized,
    {
        fn left(state: &ControlProps) -> KeyCode {
            state.left
        }
        fn right(state: &ControlProps) -> KeyCode {
            state.right
        }
        fn up(state: &ControlProps) -> KeyCode {
            state.up
        }
        fn down(state: &ControlProps) -> KeyCode {
            state.down
        }
        fn left_map(b: bool, state: &mut ControlProps) {
            if b {
                state.dir.x -= get_frame_time() * state.speed
            }
        }
        fn right_map(b: bool, state: &mut ControlProps) {
            if b {
                state.dir.x += get_frame_time() * state.speed
            }
        }
        fn up_map(b: bool, state: &mut ControlProps) {
            if b {
                state.dir.y -= get_frame_time() * state.speed
            }
        }
        fn down_map(b: bool, state: &mut ControlProps) {
            if b {
                state.dir.y += get_frame_time() * state.speed
            }
        }

        Self {
            left: KeyDown::new::<ControlProps>(left, left_map),
            right: KeyDown::new::<ControlProps>(right, right_map),
            down: KeyDown::new::<ControlProps>(down, down_map),
            up: KeyDown::new::<ControlProps>(up, up_map),
        }
    }

    fn process<'c>(
        &mut self,
        context: &Context,
        state: &'c mut ControlProps,
    ) -> &'c mut ControlProps {
        Component::<&ControlProps, &mut ControlProps>::process(&mut self.left, context, state);
        Component::<&ControlProps, &mut ControlProps>::process(&mut self.right, context, state);
        Component::<&ControlProps, &mut ControlProps>::process(&mut self.up, context, state);
        Component::<&ControlProps, &mut ControlProps>::process(&mut self.down, context, state);
        state.dir = state.dir.normalize_or_zero();
        state
    }
}
