use monad_quad::Component;

use super::{main_menu_state::OnScreen, MainMenuProperties};

pub struct DrawScreens<MainMenu, Settings> {
    main_menu: MainMenu,
    settings: Settings,
}

impl<MainMenu: Component<MainMenuProperties>, Settings: Component<MainMenuProperties>>
    Component<MainMenuProperties> for DrawScreens<MainMenu, Settings>
{
    type Input = (MainMenu, Settings);

    fn instantiate((main_menu, settings): Self::Input) -> Self
    where
        Self: Sized,
    {
        Self {
            main_menu,
            settings,
        }
    }

    fn process(&mut self, state: &mut MainMenuProperties) {
        match (&state.switching, state.on_screen) {
            (super::SwitchingTo::None, OnScreen::MainMenu)
            | (
                super::SwitchingTo::Switch {
                    from: OnScreen::MainMenu,
                    to: OnScreen::MainMenu,
                    ..
                },
                _,
            ) => self.main_menu.process(state),
            (super::SwitchingTo::None, OnScreen::Settings)
            | (
                super::SwitchingTo::Switch {
                    from: OnScreen::Settings,
                    to: OnScreen::Settings,
                    ..
                },
                _,
            ) => self.settings.process(state),
            (
                super::SwitchingTo::Switch {
                    from: OnScreen::MainMenu,
                    to: OnScreen::Settings,
                    ..
                },
                _,
            ) => {
                self.main_menu.process(state);
                self.settings.process(state)
            }
            (
                super::SwitchingTo::Switch {
                    from: OnScreen::Settings,
                    to: OnScreen::MainMenu,
                    ..
                },
                _,
            ) => {
                self.main_menu.process(state);
                self.settings.process(state)
            }
        }
    }

    fn render(&self, props: &MainMenuProperties) {
        match (&props.switching, props.on_screen) {
            (super::SwitchingTo::None, OnScreen::MainMenu)
            | (
                super::SwitchingTo::Switch {
                    from: OnScreen::MainMenu,
                    to: OnScreen::MainMenu,
                    ..
                },
                _,
            ) => self.main_menu.render(props),
            (super::SwitchingTo::None, OnScreen::Settings)
            | (
                super::SwitchingTo::Switch {
                    from: OnScreen::Settings,
                    to: OnScreen::Settings,
                    ..
                },
                _,
            ) => self.settings.render(props),
            (
                super::SwitchingTo::Switch {
                    from: OnScreen::MainMenu,
                    to: OnScreen::Settings,
                    ..
                },
                _,
            ) => {
                self.main_menu.render(props);
                self.settings.render(props)
            }
            (
                super::SwitchingTo::Switch {
                    from: OnScreen::Settings,
                    to: OnScreen::MainMenu,
                    ..
                },
                _,
            ) => {
                self.main_menu.render(props);
                self.settings.render(props)
            }
        }
    }

    fn ui(&mut self, ui: &mut macroquad::ui::Ui, state: &mut MainMenuProperties) {
        match (&state.switching, state.on_screen) {
            (super::SwitchingTo::None, OnScreen::MainMenu)
            | (
                super::SwitchingTo::Switch {
                    from: OnScreen::MainMenu,
                    to: OnScreen::MainMenu,
                    ..
                },
                _,
            ) => self.main_menu.ui(ui, state),
            (super::SwitchingTo::None, OnScreen::Settings)
            | (
                super::SwitchingTo::Switch {
                    from: OnScreen::Settings,
                    to: OnScreen::Settings,
                    ..
                },
                _,
            ) => self.settings.ui(ui, state),
            (
                super::SwitchingTo::Switch {
                    from: OnScreen::MainMenu,
                    to: OnScreen::Settings,
                    ..
                },
                _,
            ) => {
                self.main_menu.ui(ui, state);
                self.settings.ui(ui, state)
            }
            (
                super::SwitchingTo::Switch {
                    from: OnScreen::Settings,
                    to: OnScreen::MainMenu,
                    ..
                },
                _,
            ) => {
                self.main_menu.ui(ui, state);
                self.settings.ui(ui, state)
            }
        }
    }
}
