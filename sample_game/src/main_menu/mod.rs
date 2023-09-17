use macroquad::hash;
use monad_quad::{
    components::{
        animation::{Tween, TweenConfig, TweenKind},
        logic::Comp,
        ui::{Button, Window},
        Context,
    },
    Component,
};

mod main_menu_drawer;
mod main_menu_state;
mod maine_menu_flow;
mod settings_menu;
pub use main_menu_state::{MainMenuProperties, OnScreen, SwitchingTo};

#[derive(Clone, Debug)]
struct ScrollerConfig {
    data: SwitchingTo,
    tween_func: TweenKind,
}

struct Scroller {
    tweener: Tween<fn(f32, &mut f32)>,
}

impl Component<&ScrollerConfig, &mut ScrollerConfig> for Scroller {
    type Input = ();

    fn instantiate(_: Self::Input) -> Self
    where
        Self: Sized,
    {
        fn update(delta: f32, at: &mut f32) {
            *at = delta
        }
        Self {
            tweener: Tween::instantiate(update),
        }
    }

    fn process<'c>(
        &mut self,
        context: &Context,
        state: &'c mut ScrollerConfig,
    ) -> &'c mut ScrollerConfig {
        match state.data {
            SwitchingTo::OnScreen(_) => (),
            SwitchingTo::Switch {
                from,
                to,
                full_progress,
                at,
            } => {
                let mut config = TweenConfig {
                    at_time: at,
                    run: true,
                    tween_data: full_progress,
                    tween_kind: state.tween_func,
                    time_in_seconds: 2.,
                };
                self.tweener.process(context, &mut config);
                if config.at_time >= 1. {
                    state.data = SwitchingTo::OnScreen(to)
                } else {
                    state.data = SwitchingTo::Switch {
                        from,
                        to,
                        full_progress: config.tween_data,
                        at: config.at_time,
                    };
                }
            }
        }
        state
    }
}

fn main_menu() -> impl for<'a> Component<&'a MainMenuProperties, &'a mut MainMenuProperties> {
    Comp::map_in(MainMenuProperties::to_top_window_state).map_out_for(
        MainMenuProperties::merge_from_top_window_state,
        Window::new(
            hash!(),
            (
                Comp::map_in(MainMenuProperties::to_start_game_properties).map_out_for(
                    MainMenuProperties::merge_start_game_button,
                    Button::instantiate(|v| {
                        v.extra_data = true;
                    }),
                ),
                Comp::map_in(MainMenuProperties::to_settings_menu_button_properties).map_out_for(
                    MainMenuProperties::merge_to_settings_menu_button,
                    Button::instantiate(|v| {
                        v.extra_data = SwitchingTo::Switch {
                            from: OnScreen::MainMenu,
                            to: OnScreen::Settings,
                            full_progress: 0.0,
                            at: 0.,
                        };
                    }),
                ),
            ),
        ),
    )
}

pub fn main_menu2() -> impl for<'a> Component<&'a MainMenuProperties, &'a mut MainMenuProperties> {
    (
        main_menu_drawer::DrawScreens::instantiate((
            main_menu(),
            settings_menu::draw_settings_menu(),
        )),
        Comp::map_in(|x: &MainMenuProperties| ScrollerConfig {
            data: x.switching.to_owned(),
            tween_func: TweenKind::EaseInOutExpo,
        })
        .map_out_for(|x, y| y.switching = x.data, Scroller::instantiate(())),
    )
}
