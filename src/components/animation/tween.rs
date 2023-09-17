use crate::{components::Context, Component};

use super::animate_state::AnimateState;

#[derive(Clone, Copy, Debug)]
pub enum TweenKind {
    Linear,
    EaseInSine,
    EaseOutSine,
    EaseInOutSine,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInQuint,
    EaseOutQuint,
    EaseInOutQuint,
    EaseInCirc,
    EaseOutCirc,
    EaseInOutCirc,
    EaseInElastic,
    EaseOutElastic,
    EaseInOutElastic,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInQuart,
    EaseOutQuart,
    EaseInOutQuart,
    EaseInExpo,
    EaseOutExpo,
    EaseInOutExpo,
    EaseInBack,
    EaseOutBack,
    EaseInOutBack,
    EaseInBounce,
    EaseOutBounce,
    EaseInOutBounce,
}

impl TweenKind {
    fn process(&self, x: f32) -> f32 {
        match self {
            TweenKind::Linear => x,
            TweenKind::EaseInSine => 1. - f32::cos((x * std::f32::consts::PI) / 2.),
            TweenKind::EaseOutSine => f32::sin((x * std::f32::consts::PI) / 2.),
            TweenKind::EaseInOutSine => -(f32::cos(std::f32::consts::PI * x) - 1.) / 2.,
            TweenKind::EaseInCubic => x * x * x,
            TweenKind::EaseOutCubic => 1. - f32::powf(1. - x, 3.),
            TweenKind::EaseInOutCubic => {
                if x < 0.5 {
                    4. * x * x * x
                } else {
                    1. - f32::powf(-2. * x + 2., 3.) / 2.
                }
            }
            TweenKind::EaseInQuint => x * x * x * x * x,
            TweenKind::EaseOutQuint => 1. - f32::powf(1. - x, 5.),
            TweenKind::EaseInOutQuint => {
                if x < 0.5 {
                    16. * x * x * x * x * x
                } else {
                    1. - f32::powf(-2. * x + 2., 5.) / 2.
                }
            }
            TweenKind::EaseInCirc => f32::sqrt(1. - f32::powf(x, 2.)),
            TweenKind::EaseOutCirc => f32::sqrt(1. - f32::powf(x - 1., 2.)),
            TweenKind::EaseInOutCirc => {
                if x < 0.5 {
                    (1. - f32::sqrt(1. - f32::powf(2. * x, 2.))) / 2.
                } else {
                    (f32::sqrt(1. - f32::powf(-2. * x + 2., 2.)) + 1.) / 2.
                }
            }
            TweenKind::EaseInElastic => {
                let c4 = (2. * std::f32::consts::PI) / 3.;
                if x <= 0. {
                    0.
                } else if x >= 1. {
                    1.
                } else {
                    -f32::powf(2., 10. * x - 10.) * f32::sin((x * 10. - 10.75) * c4)
                }
            }
            TweenKind::EaseOutElastic => {
                let c4 = (2. * std::f32::consts::PI) / 3.;

                if x < 0. {
                    0.
                } else if x > 1. {
                    1.
                } else {
                    f32::powf(2., -10. * x) * f32::sin((x * 10. - 0.75) * c4) + 1.
                }
            }
            TweenKind::EaseInOutElastic => {
                let c5 = (2. * std::f32::consts::PI) / 4.5;

                if x < 0. {
                    0.
                } else if x > 1. {
                    1.
                } else if x < 0.5 {
                    -(f32::powf(2., 20. * x - 10.) * f32::sin((20. * x - 11.125) * c5)) / 2.
                } else {
                    (f32::powf(2., -20. * x + 10.) * f32::sin((20. * x - 11.125) * c5)) / 2.
                }
            }
            TweenKind::EaseInQuad => x * x,
            TweenKind::EaseOutQuad => 1. - (1. - x) * (1. - x),
            TweenKind::EaseInOutQuad => {
                if x < 0.5 {
                    2. * x * x
                } else {
                    1. - f32::powf(-2. * x + 2., 2.) / 2.
                }
            }
            TweenKind::EaseInQuart => x * x * x * x,
            TweenKind::EaseOutQuart => 1. - f32::powf(1. - x, 4.),
            TweenKind::EaseInOutQuart => {
                if x < 0.5 {
                    8. * x * x * x * x
                } else {
                    1. - f32::powf(-2. * x + 2., 4.) / 2.
                }
            }
            TweenKind::EaseInExpo => {
                if x == 0. {
                    0.
                } else {
                    f32::powf(2., 10. * x - 10.)
                }
            }
            TweenKind::EaseOutExpo => {
                if x == 1. {
                    1.
                } else {
                    1. - f32::powf(2., -10. * x)
                }
            }
            TweenKind::EaseInOutExpo => {
                if x < 0. {
                    0.
                } else if x > 1. {
                    1.
                } else if x < 0.5 {
                    f32::powf(2., 20. * x - 10.) / 2.
                } else {
                    (2. - f32::powf(2., -20. * x + 10.)) / 2.
                }
            }
            TweenKind::EaseInBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.;

                c3 * x * x * x - c1 * x * x
            }
            TweenKind::EaseOutBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.;

                1. + c3 * f32::powf(x - 1., 3.) + c1 * f32::powf(x - 1., 2.)
            }
            TweenKind::EaseInOutBack => {
                let c1 = 1.70158;
                let c2 = c1 * 1.525;

                if x < 0.5 {
                    (f32::powf(2. * x, 2.) * ((c2 + 1.) * 2. * x - c2)) / 2.
                } else {
                    (f32::powf(2. * x - 2., 2.) * ((c2 + 1.) * (x * 2. - 2.) + c2) + 2.) / 2.
                }
            }
            TweenKind::EaseInBounce => 1. - &TweenKind::EaseOutBounce.process(1. - x),
            TweenKind::EaseOutBounce => {
                let n1 = 7.5625;
                let d1 = 2.75;
                let mut x = x;
                if x < 1. / d1 {
                    n1 * x * x
                } else if x < 2. / d1 {
                    (x -= 1.5 / d1);
                    n1 * x * x + 0.75
                } else if x < 2.5 / d1 {
                    x -= 2.25 / d1;
                    n1 * (x) * x + 0.9375
                } else {
                    x -= 2.625 / d1;
                    n1 * (x) * x + 0.984375
                }
            }
            TweenKind::EaseInOutBounce => {
                if x < 0.5 {
                    (1. - TweenKind::EaseOutBounce.process(1. - 2. * x)) / 2.
                } else {
                    (1. + TweenKind::EaseOutBounce.process(2. * x - 1.)) / 2.
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct TweenConfig<T> {
    pub tween_data: T,
    pub at_time: f32,
    pub time_in_seconds: f32,
    pub tween_kind: TweenKind,
    pub run: bool,
}

struct AnimateData {
    max_seconds: f32,
    at: f32,
}

pub struct Tween<Func> {
    child: AnimateState<fn(f32, &mut AnimateData)>,
    func: Func,
}

impl<T, Func: Fn(f32, &mut T)> Component<&TweenConfig<T>, &mut TweenConfig<T>> for Tween<Func> {
    type Input = Func;

    fn instantiate(input: Self::Input) -> Self
    where
        Self: Sized,
    {
        fn update(delta: f32, at: &mut AnimateData) {
            at.at += delta / at.max_seconds
        }
        Self {
            child: AnimateState::instantiate(update),
            func: input,
        }
    }

    fn process<'c>(
        &mut self,
        context: &Context,
        state: &'c mut TweenConfig<T>,
    ) -> &'c mut TweenConfig<T> {
        if state.run && state.at_time < 1. {
            let mut config = AnimateData {
                max_seconds: state.time_in_seconds,
                at: state.at_time,
            };
            self.child.process(context, &mut config);
            state.at_time = config.at;
            let at = state.tween_kind.process(state.at_time);
            (self.func)(at, &mut state.tween_data);
        }
        state
    }
}
