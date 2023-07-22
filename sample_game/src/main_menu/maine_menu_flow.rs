use macroquad::{
    prelude::vec2,
    window::{screen_height, screen_width},
};
use monad_quad::components::ui::{ButtonProperties, WindowProperties};

use super::{main_menu_state::MainMenuButtonSelected, MainMenuProperties, OnScreen, SwitchingTo};

impl MainMenuProperties {
    pub fn to_top_window_state(&self) -> WindowProperties<MainMenuProperties> {
        let progress = match self.switching {
            SwitchingTo::Switch {
                from: OnScreen::MainMenu,
                full_progress,
                ..
            } => full_progress,
            SwitchingTo::Switch {
                from: OnScreen::Settings,
                full_progress,
                ..
            } => -full_progress,
            _ => 0.,
        };
        let loc = progress * screen_width();
        WindowProperties {
            location: vec2(loc, 0.),
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
}
