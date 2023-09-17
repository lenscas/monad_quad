use macroquad::{
    prelude::{vec2, Vec2},
    window::screen_width,
};

use crate::settings::Settings;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum OnScreen {
    MainMenu,
    Settings,
}

#[derive(Clone, Debug)]
pub enum SwitchingTo {
    OnScreen(OnScreen),
    Switch {
        from: OnScreen,
        to: OnScreen,
        full_progress: f32,
        at: f32,
    },
}

impl SwitchingTo {
    pub fn get_progress_from(&self, from: OnScreen) -> Option<f32> {
        match self {
            SwitchingTo::Switch {
                from: from_2,
                full_progress,
                ..
            } if from == *from_2 => Some(*full_progress),
            SwitchingTo::Switch {
                to, full_progress, ..
            } if from == *to => Some(-full_progress),
            _ => None,
        }
    }
    pub fn get_location_of_window(&self, from: OnScreen) -> Vec2 {
        let progress = self.get_progress_from(from).unwrap_or(0.);
        let loc = if progress.is_sign_negative() {
            screen_width() - ((-progress) * screen_width())
        } else {
            progress * screen_width()
        };
        vec2(loc, 0.)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MainMenuButtonSelected {
    StartGame,
    Settings,
}

#[derive(Clone, Debug)]
pub struct MainMenuProperties {
    pub started_game: bool,
    pub switching: SwitchingTo,
    pub selected_button: Option<MainMenuButtonSelected>,
    pub settings: Settings,
}
impl MainMenuProperties {
    pub fn new() -> Self {
        Self {
            started_game: false,
            switching: SwitchingTo::OnScreen(OnScreen::MainMenu),
            selected_button: None,
            settings: Default::default(),
        }
    }
}

impl Default for MainMenuProperties {
    fn default() -> Self {
        Self::new()
    }
}
