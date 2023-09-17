use macroquad::{
    prelude::vec2,
    window::{screen_height, screen_width},
};
use monad_quad::components::ui::{ButtonProperties, WindowProperties};

use super::{
    main_menu_state::MainMenuButtonSelected,
    settings_menu::{self, SettingsMenuProperties},
    MainMenuProperties, OnScreen, SwitchingTo,
};

impl MainMenuProperties {
    pub fn to_top_window_state(&self) -> WindowProperties<MainMenuProperties> {
        let loc = self.switching.get_location_of_window(OnScreen::MainMenu);
        WindowProperties {
            location: loc,
            size: vec2(screen_width(), screen_height()),
            label: Some("Monad Quad SAMPLE!".into()),
            title_bar: true,
            extra_data: self.to_owned(),
            moveable: false,
            close_button: false,
        }
    }
    pub fn merge_from_top_window_state(from: WindowProperties<MainMenuProperties>, me: &mut Self) {
        *me = from.extra_data;
    }
    pub fn to_start_game_properties<'a>(&self) -> ButtonProperties<'a, bool> {
        ButtonProperties {
            size: vec2(100., 50.),
            selected: self
                .selected_button
                .as_ref()
                .map(|v| *v == MainMenuButtonSelected::StartGame)
                .unwrap_or(false),
            position: vec2(screen_width() / 2. - 50., screen_height() / 2. - 35.),
            extra_data: self.started_game,
            content: "Start game!".into(),
        }
    }
    pub fn merge_start_game_button(from: ButtonProperties<bool>, me: &mut Self) {
        //me.start_game_button_selected = from.selected;
        me.started_game = from.extra_data;
    }
    pub fn to_settings_menu_button_properties<'a>(&self) -> ButtonProperties<'a, SwitchingTo> {
        ButtonProperties {
            size: vec2(100., 50.),
            selected: self
                .selected_button
                .as_ref()
                .map(|v| *v == MainMenuButtonSelected::Settings)
                .unwrap_or(false),
            position: vec2(screen_width() / 2. - 50., screen_height() / 2. + 30.),
            extra_data: self.switching.clone(),
            content: "Settings!".into(),
        }
    }
    pub fn merge_to_settings_menu_button(from: ButtonProperties<SwitchingTo>, me: &mut Self) {
        me.switching = from.extra_data;
    }
    pub fn to_settings_menu_properties(&self) -> settings_menu::SettingsMenuProperties {
        SettingsMenuProperties {
            settings: self.settings,
            switch: self.switching.to_owned(),
            has_back_button_selected: false,
        }
    }
    pub fn merge_from_settings_menu_properties(settings: SettingsMenuProperties, me: &mut Self) {
        me.settings = settings.settings;
        me.switching = settings.switch;
    }
}
