use monad_quad::{components::Context, Component};

use super::{main_menu_state::OnScreen, MainMenuProperties};

pub struct DrawScreens<MainMenu, Settings> {
    main_menu: MainMenu,
    settings: Settings,
}

impl<
        MainMenu: for<'a> Component<&'a MainMenuProperties, &'a mut MainMenuProperties>,
        Settings: for<'a> Component<&'a MainMenuProperties, &'a mut MainMenuProperties>,
    > Component<&MainMenuProperties, &mut MainMenuProperties> for DrawScreens<MainMenu, Settings>
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

    fn process<'c>(
        &mut self,
        context: &Context,
        state: &'c mut MainMenuProperties,
    ) -> &'c mut MainMenuProperties {
        match state.switching {
            super::SwitchingTo::OnScreen(OnScreen::MainMenu)
            | super::SwitchingTo::Switch {
                from: OnScreen::MainMenu,
                to: OnScreen::MainMenu,
                ..
            } => {
                self.main_menu.process(context, state);
            }
            super::SwitchingTo::OnScreen(OnScreen::Settings)
            | super::SwitchingTo::Switch {
                from: OnScreen::Settings,
                to: OnScreen::Settings,
                ..
            } => {
                self.settings.process(context, state);
            }

            super::SwitchingTo::Switch {
                from: OnScreen::MainMenu,
                to: OnScreen::Settings,
                ..
            } => {
                self.main_menu.process(context, state);
                self.settings.process(context, state);
            }
            super::SwitchingTo::Switch {
                from: OnScreen::Settings,
                to: OnScreen::MainMenu,
                ..
            } => {
                self.main_menu.process(context, state);
                self.settings.process(context, state);
            }
        }
        state
    }

    fn render(&self, context: &Context, props: &MainMenuProperties) {
        match &props.switching {
            super::SwitchingTo::OnScreen(OnScreen::MainMenu)
            | super::SwitchingTo::Switch {
                from: OnScreen::MainMenu,
                to: OnScreen::MainMenu,
                ..
            } => self.main_menu.render(context, props),
            super::SwitchingTo::OnScreen(OnScreen::Settings)
            | super::SwitchingTo::Switch {
                from: OnScreen::Settings,
                to: OnScreen::Settings,
                ..
            } => self.settings.render(context, props),

            super::SwitchingTo::Switch {
                from: OnScreen::MainMenu,
                to: OnScreen::Settings,
                ..
            } => {
                self.main_menu.render(context, props);
                self.settings.render(context, props)
            }
            super::SwitchingTo::Switch {
                from: OnScreen::Settings,
                to: OnScreen::MainMenu,
                ..
            } => {
                self.main_menu.render(context, props);
                self.settings.render(context, props)
            }
        }
    }

    fn ui<'c>(
        &mut self,
        context: &Context,
        ui: &mut macroquad::ui::Ui,
        state: &'c mut MainMenuProperties,
    ) -> &'c mut MainMenuProperties {
        match state.switching {
            super::SwitchingTo::OnScreen(OnScreen::MainMenu)
            | super::SwitchingTo::Switch {
                from: OnScreen::MainMenu,
                to: OnScreen::MainMenu,
                ..
            } => self.main_menu.ui(context, ui, state),
            super::SwitchingTo::OnScreen(OnScreen::Settings)
            | super::SwitchingTo::Switch {
                from: OnScreen::Settings,
                to: OnScreen::Settings,
                ..
            } => self.settings.ui(context, ui, state),

            super::SwitchingTo::Switch {
                from: OnScreen::MainMenu,
                to: OnScreen::Settings,
                ..
            } => {
                self.main_menu.ui(context, ui, state);
                self.settings.ui(context, ui, state)
            }
            super::SwitchingTo::Switch {
                from: OnScreen::Settings,
                to: OnScreen::MainMenu,
                ..
            } => {
                self.main_menu.ui(context, ui, state);
                self.settings.ui(context, ui, state)
            }
        }
    }
}
