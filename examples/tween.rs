use macroquad::prelude::{vec2, GREEN};
use monad_quad::components::{
    animation::{Tween, TweenConfig, TweenKind},
    logic::{Comp, StateFull},
    render::{Rectangle, RectangleProps},
};

struct MainState {
    tween_state: TweenConfig<f32>,
}

#[macroquad::main("Sample game")]
async fn main() {
    let state = MainState {
        tween_state: TweenConfig {
            tween_data: 0.,
            at_time: 0.,
            time_in_seconds: 2.,
            tween_kind: TweenKind::EaseInBounce,
            run: true,
        },
    };
    StateFull::new_from(state)
        .render((
            Comp::<_, Rectangle>::map_in(|v: &MainState| RectangleProps {
                size: vec2(20., 20.),
                color: GREEN,
                location: vec2(v.tween_state.tween_data * 250., 40.),
            })
            .map_out(|_, _| {}),
            Comp::<_, Tween<_>>::map_in(|v: &MainState| v.tween_state.to_owned()).map_out_with(
                |y, x| {
                    *x = y;
                },
                |x, y| y.tween_state = x,
            ),
        ))
        .await;
}
